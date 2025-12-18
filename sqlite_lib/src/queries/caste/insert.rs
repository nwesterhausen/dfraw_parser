//! Insert helper methods for the `[Caste]` struct.
use std::vec;

use dfraw_parser::Caste;
use dfraw_parser::tags::CasteTag;
use turso::params;

use crate::client::DbClient;
use crate::queries::caste::get::get_caste_name_id_by_caste_id_and_tag_position;
use crate::queries::caste::{TagValue, insert_value_tag};
use crate::queries::color::get::get_or_insert_color;
use crate::queries::misc::get::{
    get_or_insert_body_part_group, get_or_insert_dynamic_creature_caste_tag,
    get_or_insert_dynamic_item_of_material, get_or_insert_dynamic_material_in_state,
    get_or_insert_dynamic_name, get_ref_lair_token_id, get_ref_object_type_id,
    get_ref_secretion_triggers_id,
};

impl DbClient {
    /// Inserts a `[Caste]` into the database.
    ///
    /// # Parameters
    ///
    /// - `caste`: the `[Caste]` to insert
    /// - `creature_id`: the id of the parent `[dfraw_parser::Creature]` in the creatures table
    ///
    /// # Errors
    ///
    /// - Will error if a database interaction fails.
    /// - Will error if the index of a token in a caste is outside of bounds for `usize` to `i64` conversion
    pub async fn insert_caste(
        &self,
        caste: &Caste,
        creature_id: i64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // First, we need a `caste_id` to work with, so basic info insertion
        let conn = self.get_connection()?;

        conn.execute(
            super::INSERT_IDENTITY,
            params![creature_id, caste.get_identifier()],
        )
        .await?;
        // Grab the id for what we inserted
        let mut id_rows = conn
            .query(
                super::GET_ID_BY_CREATURE_AND_IDENTIFIER,
                params![creature_id, caste.get_identifier()],
            )
            .await?;

        let caste_id: i64 = id_rows
            .next()
            .await?
            .ok_or("No ID found after caste identity insertion")?
            .get(0)?;

        // Now we will loop through all the tags in the caste and insert them appropriately
        for (tag_position, tag) in caste.get_tags().iter().enumerate() {
            let token_str = tag.get_key().ok_or("Unmapped token found")?;
            let tag_id = self.get_caste_flag_id_by_token(token_str).await?;
            let position = i64::try_from(tag_position)?;

            insert_caste_tag(&conn, caste_id, tag_id, position, tag).await?;
        }

        Ok(())
    }
}

/// Insert a caste tag
///
/// # Errors
///
/// Will error if there's a database interaction error
#[allow(clippy::too_many_lines, clippy::large_stack_frames)]
pub async fn insert_caste_tag(
    conn: &turso::Connection,
    caste_id: i64,
    tag_id: i64,
    tag_position: i64,
    tag: &CasteTag,
) -> Result<(), Box<dyn std::error::Error>> {
    match tag {
        // Case A: Simple Tags (flags)
        CasteTag::AdoptsOwner
        | CasteTag::AlcoholDependent
        | CasteTag::AllActive
        | CasteTag::AmbushPredator
        | CasteTag::Amphibious
        | CasteTag::ApplyCurrentCreatureVariation
        | CasteTag::Aquatic
        | CasteTag::ArenaRestricted
        | CasteTag::AtPeaceWithWildlife
        | CasteTag::Benign
        | CasteTag::BloodSucker
        | CasteTag::BoneCarn
        | CasteTag::CanLearn
        | CasteTag::CanSpeak
        | CasteTag::CannotClimb
        | CasteTag::CannotJump
        | CasteTag::CannotUndead
        | CasteTag::CanOpenDoors
        | CasteTag::Carnivore
        | CasteTag::CaveAdaptation
        | CasteTag::CommonDomestic
        | CasteTag::ConvertedSpouse
        | CasteTag::CookableLive
        | CasteTag::Crazed
        | CasteTag::Crepuscular
        | CasteTag::CuriousBeastEater
        | CasteTag::CuriousBeastGuzzler
        | CasteTag::CuriousBeastItem
        | CasteTag::Demon
        | CasteTag::DieWhenVerminBite
        | CasteTag::Diurnal
        | CasteTag::DiveHuntsVermin
        | CasteTag::Equips
        | CasteTag::Extravision
        | CasteTag::FeatureAttackGroup
        | CasteTag::FeatureBeast
        | CasteTag::Female
        | CasteTag::FireImmune
        | CasteTag::FireImmuneSuper
        | CasteTag::FishItem
        | CasteTag::FleeQuick
        | CasteTag::Flier
        | CasteTag::GetsInfectionsFromRot
        | CasteTag::GetsWoundInfections
        | CasteTag::HasNerves
        | CasteTag::HasShell
        | CasteTag::HuntsVermin
        | CasteTag::Immobile
        | CasteTag::ImmobileLand
        | CasteTag::Immolate
        | CasteTag::Intelligent
        | CasteTag::LairHunter
        | CasteTag::LargePredator
        | CasteTag::LaysEggs
        | CasteTag::LightGen
        | CasteTag::LikesFighting
        | CasteTag::Lisp
        | CasteTag::LockPicker
        | CasteTag::Magical
        | CasteTag::MagmaVision
        | CasteTag::Male
        | CasteTag::MannerismLaugh
        | CasteTag::MannerismSmile
        | CasteTag::MannerismWalk
        | CasteTag::MannerismSit
        | CasteTag::MannerismBreath
        | CasteTag::MannerismPosture
        | CasteTag::MannerismStretch
        | CasteTag::MannerismEyelids
        | CasteTag::Matutinal
        | CasteTag::Meanderer
        | CasteTag::Megabeast
        | CasteTag::Mischievous
        | CasteTag::Mount
        | CasteTag::MountExotic
        | CasteTag::MultipartFullVision
        | CasteTag::MultipleLitterRare
        | CasteTag::Natural
        | CasteTag::NightCreatureBogeyman
        | CasteTag::NightCreatureExperimenter
        | CasteTag::NightCreatureHunter
        | CasteTag::NightCreatureNightmare
        | CasteTag::NoConnectionsForMovement
        | CasteTag::NoDizziness
        | CasteTag::NoDrink
        | CasteTag::NoEat
        | CasteTag::NoFall
        | CasteTag::NoFevers
        | CasteTag::NoGender
        | CasteTag::NoPhysicalAttributeGain
        | CasteTag::NoPhysicalAttributeRust
        | CasteTag::NoSleep
        | CasteTag::NoSpring
        | CasteTag::NoSummer
        | CasteTag::NoThoughtCenterForMovement
        | CasteTag::NoUnitTypeColor
        | CasteTag::NoVegetationDisturbance
        | CasteTag::NoWinter
        | CasteTag::NoBones
        | CasteTag::NoBreathe
        | CasteTag::Nocturnal
        | CasteTag::NoEmotion
        | CasteTag::NoExert
        | CasteTag::NoFear
        | CasteTag::NoMeat
        | CasteTag::NoNausea
        | CasteTag::NoPain
        | CasteTag::NoSkin
        | CasteTag::NoSkull
        | CasteTag::NoSmellyRot
        | CasteTag::NoStuckIns
        | CasteTag::NoStun
        | CasteTag::NotButcherable
        | CasteTag::NotLiving
        | CasteTag::NoThought
        | CasteTag::OpposedToLife
        | CasteTag::OutsiderControllable
        | CasteTag::PackAnimal
        | CasteTag::ParalyzeImmune
        | CasteTag::PatternFlier
        | CasteTag::Pearl
        | CasteTag::Pet
        | CasteTag::PetExotic
        | CasteTag::Power
        | CasteTag::RemainsOnVerminBiteDeath
        | CasteTag::RemainsUndetermined
        | CasteTag::ReturnsVerminKillsToOwner
        | CasteTag::SemiMegabeast
        | CasteTag::SlowLearner
        | CasteTag::SmallRemains
        | CasteTag::SpouseConversionTarget
        | CasteTag::SpouseConverter
        | CasteTag::SpreadEvilSpheresIfRuler
        | CasteTag::StanceClimber
        | CasteTag::StandardGrazer
        | CasteTag::StrangeMoods
        | CasteTag::Supernatural
        | CasteTag::SwimsInnate
        | CasteTag::SwimsLearned
        | CasteTag::ThickWeb
        | CasteTag::Titan
        | CasteTag::Trainable
        | CasteTag::TrainableHunting
        | CasteTag::TrainableWar
        | CasteTag::Trances
        | CasteTag::TrapAvoid
        | CasteTag::UnderSwim
        | CasteTag::UniqueDemon
        | CasteTag::Vegetation
        | CasteTag::VerminHateable
        | CasteTag::VerminMicro
        | CasteTag::VerminNoFish
        | CasteTag::VerminNoRoam
        | CasteTag::VerminNoTrap
        | CasteTag::VerminHunter
        | CasteTag::Vespertine
        | CasteTag::WagonPuller
        | CasteTag::WebImmune
        | CasteTag::Unknown
        | CasteTag::NightCreature
        | CasteTag::NotFireImmune
        | CasteTag::HasBlood
        | CasteTag::Grasp
        | CasteTag::RaceGait
        | CasteTag::CannotBreatheWater
        | CasteTag::NaturalAnimal
        | CasteTag::CuriousBeast
        | CasteTag::CannotBreatheAir => {
            conn.execute(super::INSERT_TAG, params![caste_id, tag_id, tag_position])
                .await?;
        }
        CasteTag::BuildingDestroyer {
            door_and_furniture_focused,
        } => {
            let values = vec![TagValue::Bool(*door_and_furniture_focused)];

            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        //  B.1: String flags
        CasteTag::AltTile { tile }
        | CasteTag::GlowTile { tile }
        | CasteTag::SoldierTile { tile }
        | CasteTag::SoldierAltTile { tile }
        | CasteTag::Tile { tile } => {
            let values = vec![TagValue::String(tile.clone())];

            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::Extract { material } | CasteTag::Webber { material } => {
            let values = vec![TagValue::String(material.clone())];

            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::BodyPartAddType { body_part_type }
        | CasteTag::BodyPartRemoveType { body_part_type } => {
            let values = vec![TagValue::String(body_part_type.clone())];

            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::CreatureVariationAddTag { tag }
        | CasteTag::CreatureVariationRemoveTag { tag } => {
            let values = vec![TagValue::String(tag.clone())];

            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::BodyGloss { gloss } => {
            let values = vec![TagValue::String(gloss.clone())];

            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::CanDoInteraction { interaction } => {
            let values = vec![TagValue::String(interaction.clone())];

            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::CreatureClass { class } => {
            let values = vec![TagValue::String(class.clone())];

            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::Description { description } => {
            let values = vec![TagValue::String(description.clone())];

            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::Gait { gait } => {
            //todo: this needs updated to insert into the `caste_gaits` table
            let values = vec![TagValue::String(gait.clone())];

            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::Gnawer { verb } => {
            let values = vec![TagValue::String(verb.clone())];

            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::GobbleVerminClass { vermin_class } => {
            let values = vec![TagValue::String(vermin_class.clone())];

            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::Habit { habit } => {
            let values = vec![TagValue::String(habit.clone())];

            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::ExtraButcherObjectShape { shape } => {
            let values = vec![TagValue::String(shape.clone())];

            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::LairCharacteristic { characteristic } => {
            let values = vec![TagValue::String(characteristic.clone())];

            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::LairHunterSpeech { speech_file } | CasteTag::SlainSpeech { speech_file } => {
            let values = vec![TagValue::String(speech_file.clone())];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::MannerismNose { nose } => {
            let values = vec![TagValue::String(nose.clone())];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::MannerismEar { ear } => {
            let values = vec![TagValue::String(ear.clone())];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::MannerismHead { head } => {
            let values = vec![TagValue::String(head.clone())];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::MannerismEyes { eyes } => {
            let values = vec![TagValue::String(eyes.clone())];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::MannerismMouth { mouth } => {
            let values = vec![TagValue::String(mouth.clone())];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::MannerismHair { hair } => {
            let values = vec![TagValue::String(hair.clone())];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::MannerismKnuckles { knuckles } => {
            let values = vec![TagValue::String(knuckles.clone())];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::MannerismLips { lips } => {
            let values = vec![TagValue::String(lips.clone())];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::MannerismCheek { cheek } => {
            let values = vec![TagValue::String(cheek.clone())];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::MannerismNails { nails } => {
            let values = vec![TagValue::String(nails.clone())];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::MannerismFeet { feet } => {
            let values = vec![TagValue::String(feet.clone())];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::MannerismArms { arms } => {
            let values = vec![TagValue::String(arms.clone())];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::MannerismHands { hands } => {
            let values = vec![TagValue::String(hands.clone())];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::MannerismTongue { tongue } => {
            let values = vec![TagValue::String(tongue.clone())];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::MannerismLeg { leg } => {
            let values = vec![TagValue::String(leg.clone())];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::ModValue { value } => {
            let values = vec![TagValue::String(value.clone())];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::OdorString { odor_string } => {
            let values = vec![TagValue::String(odor_string.clone())];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::RemainsColor { remains_color } => {
            let values = vec![TagValue::String(remains_color.clone())];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        //  B.2: Integer flags
        CasteTag::Baby { age } | CasteTag::Child { age } => {
            let values = vec![TagValue::Int(i64::from(*age))];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::BeachFrequency { frequency } => {
            let values = vec![TagValue::Int(i64::from(*frequency))];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::ChangeBodySizePercent { percent } => {
            let values = vec![TagValue::Int(i64::from(*percent))];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::Difficulty { difficulty } => {
            let values = vec![TagValue::Int(i64::from(*difficulty))];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::EggSize { size } => {
            let values = vec![TagValue::Int(i64::from(*size))];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::FixedTemp { temperature } => {
            let values = vec![TagValue::Int(i64::from(*temperature))];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::GravitateBodySize { target } => {
            let values = vec![TagValue::Int(i64::from(*target))];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::Grazer { grazer } => {
            let values = vec![TagValue::Int(i64::from(*grazer))];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::HabitNumber { number } => {
            let values = vec![TagValue::Int(i64::from(*number))];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::Homeotherm { temperature } => {
            if let Some(temp) = *temperature {
                let values = vec![TagValue::Int(i64::from(temp))];
                insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
            }
        }
        CasteTag::GrassTrample { trample } => {
            let values = vec![TagValue::Int(i64::from(*trample))];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::LowLightVision { vision } => {
            let values = vec![TagValue::Int(i64::from(*vision))];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::OdorLevel { odor_level } => {
            let values = vec![TagValue::Int(i64::from(*odor_level))];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::PenetratePower { penetrate_power } => {
            let values = vec![TagValue::Int(i64::from(*penetrate_power))];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::PetValue { pet_value } => {
            let values = vec![TagValue::Int(i64::from(*pet_value))];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::PetValueDivisor { divisor } => {
            let values = vec![TagValue::Int(i64::from(*divisor))];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::PopulationRatio { pop_ratio } => {
            let values = vec![TagValue::Int(i64::from(*pop_ratio))];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::ProneToRage { rage_chance } => {
            let values = vec![TagValue::Int(i64::from(*rage_chance))];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::ItemCorpseQuality { quality } => {
            let values = vec![TagValue::Int(i64::from(*quality))];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::SkillLearnRates { rate } => {
            let values = vec![TagValue::Int(i64::from(*rate))];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::TradeCapacity { capacity } => {
            let values = vec![TagValue::Int(i64::from(*capacity))];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::ViewRange { view_range } => {
            let values = vec![TagValue::Int(i64::from(*view_range))];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        //  B.3: Range flags
        CasteTag::ClutchSize { min, max }
        | CasteTag::LitterSize { min, max }
        | CasteTag::MaxAge { min, max } => {
            let values = vec![
                TagValue::Int(i64::from(*min)),
                TagValue::Int(i64::from(*max)),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::GeneralMaterialForceMultiplier { value_a, value_b } => {
            let values = vec![
                TagValue::Int(i64::from(*value_a)),
                TagValue::Int(i64::from(*value_b)),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        //  B.4: 7-spread range flags
        CasteTag::MentalAttributeRange { attribute, ranges }
        | CasteTag::PhysicalAttributeRange { attribute, ranges } => {
            let values = vec![
                TagValue::String(attribute.clone()),
                TagValue::Int(ranges[0].into()),
                TagValue::Int(ranges[1].into()),
                TagValue::Int(ranges[2].into()),
                TagValue::Int(ranges[3].into()),
                TagValue::Int(ranges[4].into()),
                TagValue::Int(ranges[5].into()),
                TagValue::Int(ranges[6].into()),
            ];

            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::BodyAppearanceModifier { attribute, values } => {
            let mut vals: Vec<TagValue> = Vec::with_capacity(8);
            vals.push(TagValue::String(attribute.clone()));
            for v in values {
                vals.push(TagValue::Int(i64::from(*v)));
            }
            insert_value_tag(conn, caste_id, tag_id, tag_position, &vals).await?;
        }
        //  B.5: String - Integer flags
        CasteTag::SyndromeDilutionFactor {
            syndrome,
            percentage,
        } => {
            let values = vec![
                TagValue::String(syndrome.clone()),
                TagValue::Int(i64::from(*percentage)),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::Tendons {
            material,
            healing_rate,
        }
        | CasteTag::Ligaments {
            material,
            healing_rate,
        } => {
            // store as value-tag so dynamic material handling is not bypassed here
            let values = vec![
                TagValue::String(material.clone()),
                TagValue::Int(i64::from(*healing_rate)),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::NaturalSkill { skill, level } => {
            let values = vec![
                TagValue::String(skill.clone()),
                TagValue::Int(i64::from(*level)),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::MentalAttributeCapPercentage {
            attribute,
            percentage,
        }
        | CasteTag::PhysicalAttributeCapPercentage {
            attribute,
            percentage,
        } => {
            let values = vec![
                TagValue::String(attribute.clone()),
                TagValue::Int(i64::from(*percentage)),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::Milkable {
            material,
            frequency,
        } => {
            // store as value-tag to unify value insertion
            let values = vec![
                TagValue::String(material.clone()),
                TagValue::Int(i64::from(*frequency)),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        //  B.6: String - String flags
        CasteTag::MannerismFingers { finger, fingers } => {
            let values = vec![
                TagValue::String(finger.clone()),
                TagValue::String(fingers.clone()),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::SpecificFood {
            food_type,
            identifier,
        } => {
            let Some(food_type_token) = food_type.get_key() else {
                return Err("Failed to get token for food type".into());
            };
            let food_type_object_type_id = get_ref_object_type_id(conn, food_type_token).await?;

            conn.execute(
                super::INSERT_SPECIFIC_FOOD_TAG,
                params![
                    caste_id,
                    tag_id,
                    tag_position,
                    food_type_object_type_id,
                    identifier.as_str()
                ],
            )
            .await?;
        }
        // Case C: Properties in special tables
        CasteTag::Attack { name, body_part } => {
            conn.execute(
                super::INSERT_ATTACK_TAG,
                params![caste_id, tag_position, name.as_str(), body_part.as_str()],
            )
            .await?;
        }
        CasteTag::AttackTrigger {
            population,
            exported_wealth,
            created_wealth,
        } => {
            conn.execute(
                super::INSERT_ATTACK_TRIGGER_TAG,
                params![
                    caste_id,
                    tag_position,
                    population,
                    exported_wealth,
                    created_wealth
                ],
            )
            .await?;
        }
        CasteTag::BodyDetailPlan {
            body_plan,
            arguments,
        } => {
            // Must insert body plan identity and get id first
            conn.execute(
                super::INSERT_BODY_DETAIL_PLAN_IDENTITY_TAG,
                params![caste_id, tag_position, body_plan.as_str()],
            )
            .await?;
            // Grab the id for what we inserted
            let mut id_rows = conn
                .query(
                    super::GET_BODY_DETAIL_PLAN_BY_CASTE_ID_AND_NAME,
                    params![caste_id, body_plan.as_str()],
                )
                .await?;

            let body_detail_plan_id: i64 = id_rows
                .next()
                .await?
                .ok_or("No ID found after caste identity insertion")?
                .get(0)?;

            let mut batch_insert_sql = String::new();
            for (idx, argument) in arguments.iter().enumerate() {
                let argument_index = i64::try_from(idx)?;
                let insert_sql = format!(
                    "INSERT INTO caste_body_detail_plan_args (body_detail_plan_id, argument_index, argument)
                    VALUES ({body_detail_plan_id}, {argument_index}, '{argument}');"
                );
                batch_insert_sql.push_str(&insert_sql);
            }

            conn.execute_batch(&batch_insert_sql).await?;
        }
        CasteTag::ItemCorpse { item, material }
        | CasteTag::ExtraButcherObjectItem { item, material }
        | CasteTag::LaysUnusualEggs { item, material } => {
            let dyn_item_id =
                get_or_insert_dynamic_item_of_material(conn, item.as_str(), material.as_str())
                    .await?;

            conn.execute(
                super::INSERT_ITEM_TAG,
                params![caste_id, tag_id, dyn_item_id, tag_position],
            )
            .await?;
        }
        CasteTag::Pus { material, state }
        | CasteTag::Blood { material, state }
        | CasteTag::EggMaterial { material, state } => {
            let dyn_mat_id = get_or_insert_dynamic_material_in_state(conn, material, state).await?;

            conn.execute(
                super::INSERT_MATERIAL_TAG,
                params![caste_id, tag_id, dyn_mat_id, tag_position],
            )
            .await?;
        }
        CasteTag::GobbleVerminCreature {
            vermin_creature,
            vermin_caste,
        } => {
            let creature_caste_tag_id =
                get_or_insert_dynamic_creature_caste_tag(conn, vermin_creature, vermin_caste)
                    .await?;

            conn.execute(
                super::INSERT_CREATURE_CASTE_TAG,
                params![caste_id, tag_id, creature_caste_tag_id, tag_position],
            )
            .await?;
        }
        CasteTag::Lair { lair, probability } => {
            let ref_lair_id = get_ref_lair_token_id(conn, lair).await?;

            if let Some(id) = ref_lair_id {
                conn.execute(
                    super::INSERT_LAIR_REF_TAG,
                    params![caste_id, tag_id, id, tag_position, probability],
                )
                .await?;
            } else {
                return Err(format!("Unknown lair token encountered: '{lair}'").into());
            }
        }
        CasteTag::SkillRates {
            improvement_rate,
            decay_rate_unused,
            decay_rate_rusty,
            decay_rate_demotion,
        } => {
            let values = vec![
                TagValue::Int(i64::from(*improvement_rate)),
                TagValue::Int(i64::from(*decay_rate_unused)),
                TagValue::Int(i64::from(*decay_rate_rusty)),
                TagValue::Int(i64::from(*decay_rate_demotion)),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::SkillRustRates {
            decay_rate_unused,
            decay_rate_rusty,
            decay_rate_demotion,
        } => {
            let values = vec![
                TagValue::Int(i64::from(*decay_rate_unused)),
                TagValue::Int(i64::from(*decay_rate_rusty)),
                TagValue::Int(i64::from(*decay_rate_demotion)),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::SkillRustRate {
            skill,
            decay_rate_unused,
            decay_rate_rusty,
            decay_rate_demotion,
        } => {
            let values = vec![
                TagValue::String(skill.clone()),
                TagValue::Int(i64::from(*decay_rate_unused)),
                TagValue::Int(i64::from(*decay_rate_rusty)),
                TagValue::Int(i64::from(*decay_rate_demotion)),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::SkillRate {
            skill,
            improvement_rate,
            decay_rate_unused,
            decay_rate_rusty,
            decay_rate_demotion,
        } => {
            let values = vec![
                TagValue::String(skill.clone()),
                TagValue::Int(i64::from(*improvement_rate)),
                TagValue::Int(i64::from(*decay_rate_unused)),
                TagValue::Int(i64::from(*decay_rate_rusty)),
                TagValue::Int(i64::from(*decay_rate_demotion)),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::PhysicalAttributeRate {
            attribute,
            improvement_cost,
            decay_rate_unused,
            decay_rate_rusty,
            decay_rate_demotion,
        }
        | CasteTag::MentalAttributeRate {
            attribute,
            improvement_cost,
            decay_rate_unused,
            decay_rate_rusty,
            decay_rate_demotion,
        } => {
            let values = vec![
                TagValue::String(attribute.clone()),
                TagValue::Int(i64::from(*improvement_cost)),
                TagValue::Int(i64::from(*decay_rate_unused)),
                TagValue::Int(i64::from(*decay_rate_rusty)),
                TagValue::Int(i64::from(*decay_rate_demotion)),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::Orientation {
            caste,
            disinterested_chance,
            casual_chance,
            strong_chance,
        } => {
            let values = vec![
                TagValue::String(caste.clone()),
                TagValue::Int(i64::from(*disinterested_chance)),
                TagValue::Int(i64::from(*casual_chance)),
                TagValue::Int(i64::from(*strong_chance)),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::Personality {
            personality_trait,
            low,
            median,
            high,
        } => {
            let values = vec![
                TagValue::String(personality_trait.clone()),
                TagValue::Int(i64::from(*low)),
                TagValue::Int(i64::from(*median)),
                TagValue::Int(i64::from(*high)),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::PlusBodyPartGroup {
            body_part_selector,
            body_part_group,
        } => {
            let values = vec![
                TagValue::String(body_part_selector.clone()),
                TagValue::String(body_part_group.clone()),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::ProfessionName {
            profession,
            singular,
            plural,
        } => {
            let dyn_name_id =
                get_or_insert_dynamic_name(conn, singular.as_str(), plural.as_str(), None).await?;

            conn.execute(
                super::INSERT_NAME_TAG,
                params![caste_id, tag_id, dyn_name_id, tag_position],
            )
            .await?;

            let caste_name_id =
                get_caste_name_id_by_caste_id_and_tag_position(conn, caste_id, tag_position)
                    .await?;

            conn.execute(
                super::INSERT_PROFESSION_NAME_TAG,
                params![
                    caste_id,
                    tag_id,
                    tag_position,
                    caste_name_id,
                    profession.as_str()
                ],
            )
            .await?;
        }
        CasteTag::Remains { singular, plural }
        | CasteTag::BabyName { singular, plural }
        | CasteTag::ChildName { singular, plural } => {
            let dyn_name_id =
                get_or_insert_dynamic_name(conn, singular.as_str(), plural.as_str(), None).await?;

            conn.execute(
                super::INSERT_NAME_TAG,
                params![caste_id, tag_id, dyn_name_id, tag_position],
            )
            .await?;
        }
        CasteTag::Name {
            singular,
            plural,
            adjective,
        } => {
            let dyn_name_id =
                get_or_insert_dynamic_name(conn, singular, plural, Some(adjective)).await?;

            conn.execute(
                super::INSERT_NAME_TAG,
                params![caste_id, tag_id, dyn_name_id, tag_position],
            )
            .await?;
        }
        CasteTag::RelativeSize {
            body_part_selector,
            body_part,
            relative_size,
        } => {
            let values = vec![
                TagValue::String(body_part_selector.clone()),
                TagValue::String(body_part.clone()),
                TagValue::Int(i64::from(*relative_size)),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::RetractIntoBodyPart {
            body_part_selector,
            body_part,
            second_person,
            third_person,
            second_person_cancel,
            third_person_cancel,
        } => {
            let values = vec![
                TagValue::String(body_part_selector.clone()),
                TagValue::String(body_part.clone()),
                TagValue::String(second_person.clone()),
                TagValue::String(third_person.clone()),
                TagValue::String(second_person_cancel.clone()),
                TagValue::String(third_person_cancel.clone()),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::RootAround {
            body_part_selector,
            body_part,
            second_person_verb,
            third_person_verb,
        } => {
            let values = vec![
                TagValue::String(body_part_selector.clone()),
                TagValue::String(body_part.clone()),
                TagValue::String(second_person_verb.clone()),
                TagValue::String(third_person_verb.clone()),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::Secretion {
            material_token,
            material_state,
            body_part_selector,
            body_part,
            tissue_layer,
            trigger,
        } => {
            let dyn_mat_id =
                get_or_insert_dynamic_material_in_state(conn, material_token, material_state)
                    .await?;
            let body_part_group_id =
                get_or_insert_body_part_group(conn, body_part_selector, body_part).await?;
            let trigger_id = get_ref_secretion_triggers_id(conn, trigger.as_str()).await?;

            conn.execute(
                super::INSERT_SECRETION_TAG,
                params![
                    caste_id,
                    tag_id,
                    tag_position,
                    dyn_mat_id,
                    body_part_group_id,
                    tissue_layer.as_str(),
                    trigger_id
                ],
            )
            .await?;
        }
        CasteTag::SenseCreatureClass {
            creature_class,
            tile,
            foreground,
            background,
            brightness,
        } => {
            let values = vec![
                TagValue::String(creature_class.clone()),
                TagValue::String(tile.clone()),
                TagValue::Int(i64::from(*foreground)),
                TagValue::Int(i64::from(*background)),
                TagValue::Int(i64::from(*brightness)),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::SetBodyPartGroup {
            body_part_selector,
            body_part,
        } => {
            let values = vec![
                TagValue::String(body_part_selector.clone()),
                TagValue::String(body_part.clone()),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::Sound {
            sound_type,
            sound_range,
            sound_interval,
            requires_breathing,
            first_person,
            third_person,
            out_of_sight,
        } => {
            // store basic sound fields as values for now
            let values = vec![
                TagValue::String(sound_type.clone()),
                TagValue::Int(i64::from(*sound_range)),
                TagValue::Int(i64::from(*sound_interval)),
                TagValue::Bool(*requires_breathing),
                TagValue::String(first_person.clone()),
                TagValue::String(third_person.clone()),
                TagValue::String(out_of_sight.clone()),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::TissueLayer {
            body_part_selector,
            body_part,
            tissue,
            location,
        } => {
            // store the arguments as values for now
            let values = vec![
                TagValue::String(body_part_selector.clone()),
                TagValue::String(body_part.clone()),
                TagValue::String(tissue.clone()),
                TagValue::String(location.clone()),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::TissueLayerUnder {
            body_part_selector,
            body_part,
            tissue,
        } => {
            let values = vec![
                TagValue::String(body_part_selector.clone()),
                TagValue::String(body_part.clone()),
                TagValue::String(tissue.clone()),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::VerminBite {
            chance,
            verb,
            material,
            material_state,
        } => {
            // store simple parts; material handling remains more complex and is left to follow-up
            let values = vec![
                TagValue::Int(i64::from(*chance)),
                TagValue::String(verb.clone()),
                TagValue::String(material.clone()),
                TagValue::String(material_state.clone()),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::VisionArc {
            binocular,
            non_binocular,
        } => {
            let values = vec![
                TagValue::Int(i64::from(*binocular)),
                TagValue::Int(i64::from(*non_binocular)),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::SkillLearnRate { skill, rate } => {
            let values = vec![
                TagValue::String(skill.clone()),
                TagValue::Int(i64::from(*rate)),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::BodySize { year, days, size } => {
            let values = vec![
                TagValue::Int(i64::from(*year)),
                TagValue::Int(i64::from(*days)),
                TagValue::Int(i64::from(*size)),
            ];
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::BodyPartAppearanceModifier { quality, spread } => {
            let mut values = vec![TagValue::String(quality.clone())];
            for s in spread {
                values.push(TagValue::Int(i64::from(*s)));
            }
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::ApplyCreatureVariation { id, args } => {
            let mut values = vec![TagValue::String(id.clone())];
            for a in args {
                values.push(TagValue::String(a.clone()));
            }
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::ExtraButcherObject {
            object_type,
            arguments,
        } => {
            let mut values = vec![TagValue::String(object_type.clone())];
            for a in arguments {
                values.push(TagValue::String(a.clone()));
            }
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::GlowColor {
            foreground,
            background,
            brightness,
        }
        | CasteTag::Color {
            foreground,
            background,
            brightness,
        } => {
            let fg = i64::from(*foreground);
            let bg = i64::from(*background);
            let bright = i64::from(*brightness);

            let color_id = get_or_insert_color(conn, fg, bg, bright).await?;

            conn.execute(
                super::INSERT_COLOR_TAG,
                params![caste_id, tag_id, color_id, tag_position],
            )
            .await?;
        }
        CasteTag::InteractionDetail { args } => {
            let mut values: Vec<TagValue> = Vec::new();
            for a in args {
                values.push(TagValue::String(a.clone()));
            }
            insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
        }
        CasteTag::Body { body_parts } => {
            // Body parts are complex structures; store tokens for now
            let mut values: Vec<TagValue> = Vec::new();
            for bp in body_parts {
                // `body_parts` is a Vec<String>; store the string values directly
                values.push(TagValue::String(bp.clone()));
            }
            if !values.is_empty() {
                insert_value_tag(conn, caste_id, tag_id, tag_position, &values).await?;
            }
        }
    }

    Ok(())
}
