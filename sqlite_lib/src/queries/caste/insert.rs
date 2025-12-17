//! Insert helper methods for the `[Caste]` struct.
use dfraw_parser::Caste;
use dfraw_parser::tags::CasteTag;
use turso::params;

use crate::client::DbClient;
use crate::queries::caste::get::get_caste_name_id_by_caste_id_and_tag_position;
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
            super::INSERT_CASTE_IDENTITY,
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
#[allow(clippy::too_many_lines)]
pub async fn insert_caste_tag(
    conn: &turso::Connection,
    caste_id: i64,
    tag_id: i64,
    position: i64,
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
            // Insert this into the caste_flags table
            conn.execute(
                super::INSERT_CASTE_FLAG,
                params![caste_id, tag_id, position],
            )
            .await?;
        }
        // Case B: Valued flags
        //  B.0: boolean flags
        CasteTag::BuildingDestroyer {
            door_and_furniture_focused,
        } => {
            let bit_value = i64::from(*door_and_furniture_focused);

            conn.execute(
                super::INSERT_CASTE_BOOLEAN_FLAG,
                params![caste_id, tag_id, position, bit_value],
            )
            .await?;
        }
        //  B.1: String flags
        CasteTag::AltTile { tile }
        | CasteTag::GlowTile { tile }
        | CasteTag::SoldierTile { tile }
        | CasteTag::SoldierAltTile { tile }
        | CasteTag::Tile { tile } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, tile.as_str()],
            )
            .await?;
        }
        CasteTag::Extract { material } | CasteTag::Webber { material } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, material.as_str()],
            )
            .await?;
        }
        CasteTag::BodyPartAddType { body_part_type }
        | CasteTag::BodyPartRemoveType { body_part_type } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, body_part_type.as_str()],
            )
            .await?;
        }
        CasteTag::CreatureVariationAddTag { tag }
        | CasteTag::CreatureVariationRemoveTag { tag } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, tag.as_str()],
            )
            .await?;
        }
        CasteTag::BodyGloss { gloss } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, gloss.as_str()],
            )
            .await?;
        }
        CasteTag::CanDoInteraction { interaction } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, interaction.as_str()],
            )
            .await?;
        }
        CasteTag::CreatureClass { class } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, class.as_str()],
            )
            .await?;
        }
        CasteTag::Description { description } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, description.as_str()],
            )
            .await?;
        }
        CasteTag::Gait { gait } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, gait.as_str()],
            )
            .await?;
        }
        CasteTag::Gnawer { verb } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, verb.as_str()],
            )
            .await?;
        }
        CasteTag::GobbleVerminClass { vermin_class } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, vermin_class.as_str()],
            )
            .await?;
        }
        CasteTag::Habit { habit } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, habit.as_str()],
            )
            .await?;
        }
        CasteTag::ExtraButcherObjectShape { shape } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, shape.as_str()],
            )
            .await?;
        }
        CasteTag::LairCharacteristic { characteristic } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, characteristic.as_str()],
            )
            .await?;
        }
        CasteTag::LairHunterSpeech { speech_file } | CasteTag::SlainSpeech { speech_file } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, speech_file.as_str()],
            )
            .await?;
        }
        CasteTag::MannerismNose { nose } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, nose.as_str()],
            )
            .await?;
        }
        CasteTag::MannerismEar { ear } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, ear.as_str()],
            )
            .await?;
        }
        CasteTag::MannerismHead { head } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, head.as_str()],
            )
            .await?;
        }
        CasteTag::MannerismEyes { eyes } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, eyes.as_str()],
            )
            .await?;
        }
        CasteTag::MannerismMouth { mouth } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, mouth.as_str()],
            )
            .await?;
        }
        CasteTag::MannerismHair { hair } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, hair.as_str()],
            )
            .await?;
        }
        CasteTag::MannerismKnuckles { knuckles } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, knuckles.as_str()],
            )
            .await?;
        }
        CasteTag::MannerismLips { lips } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, lips.as_str()],
            )
            .await?;
        }
        CasteTag::MannerismCheek { cheek } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, cheek.as_str()],
            )
            .await?;
        }
        CasteTag::MannerismNails { nails } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, nails.as_str()],
            )
            .await?;
        }
        CasteTag::MannerismFeet { feet } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, feet.as_str()],
            )
            .await?;
        }
        CasteTag::MannerismArms { arms } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, arms.as_str()],
            )
            .await?;
        }
        CasteTag::MannerismHands { hands } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, hands.as_str()],
            )
            .await?;
        }
        CasteTag::MannerismTongue { tongue } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, tongue.as_str()],
            )
            .await?;
        }
        CasteTag::MannerismLeg { leg } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, leg.as_str()],
            )
            .await?;
        }
        CasteTag::ModValue { value } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, value.as_str()],
            )
            .await?;
        }
        CasteTag::OdorString { odor_string } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, odor_string.as_str()],
            )
            .await?;
        }
        CasteTag::RemainsColor { remains_color } => {
            conn.execute(
                super::INSERT_CASTE_STRING_FLAG,
                params![caste_id, tag_id, position, remains_color.as_str()],
            )
            .await?;
        }
        //  B.2: Integer flags
        CasteTag::Baby { age } | CasteTag::Child { age } => {
            conn.execute(
                super::INSERT_CASTE_INTEGER_FLAG,
                params![caste_id, tag_id, position, age],
            )
            .await?;
        }
        CasteTag::BeachFrequency { frequency } => {
            conn.execute(
                super::INSERT_CASTE_INTEGER_FLAG,
                params![caste_id, tag_id, position, frequency],
            )
            .await?;
        }
        CasteTag::ChangeBodySizePercent { percent } => {
            conn.execute(
                super::INSERT_CASTE_INTEGER_FLAG,
                params![caste_id, tag_id, position, percent],
            )
            .await?;
        }
        CasteTag::Difficulty { difficulty } => {
            conn.execute(
                super::INSERT_CASTE_INTEGER_FLAG,
                params![caste_id, tag_id, position, difficulty],
            )
            .await?;
        }
        CasteTag::EggSize { size } => {
            conn.execute(
                super::INSERT_CASTE_INTEGER_FLAG,
                params![caste_id, tag_id, position, size],
            )
            .await?;
        }
        CasteTag::FixedTemp { temperature } => {
            conn.execute(
                super::INSERT_CASTE_INTEGER_FLAG,
                params![caste_id, tag_id, position, temperature],
            )
            .await?;
        }
        CasteTag::GravitateBodySize { target } => {
            conn.execute(
                super::INSERT_CASTE_INTEGER_FLAG,
                params![caste_id, tag_id, position, target],
            )
            .await?;
        }
        CasteTag::Grazer { grazer } => {
            conn.execute(
                super::INSERT_CASTE_INTEGER_FLAG,
                params![caste_id, tag_id, position, grazer],
            )
            .await?;
        }
        CasteTag::HabitNumber { number } => {
            conn.execute(
                super::INSERT_CASTE_INTEGER_FLAG,
                params![caste_id, tag_id, position, number],
            )
            .await?;
        }
        CasteTag::Homeotherm { temperature } => {
            conn.execute(
                super::INSERT_CASTE_INTEGER_FLAG,
                params![caste_id, tag_id, position, temperature],
            )
            .await?;
        }
        CasteTag::GrassTrample { trample } => {
            conn.execute(
                super::INSERT_CASTE_INTEGER_FLAG,
                params![caste_id, tag_id, position, trample],
            )
            .await?;
        }
        CasteTag::LowLightVision { vision } => {
            conn.execute(
                super::INSERT_CASTE_INTEGER_FLAG,
                params![caste_id, tag_id, position, vision],
            )
            .await?;
        }
        CasteTag::OdorLevel { odor_level } => {
            conn.execute(
                super::INSERT_CASTE_INTEGER_FLAG,
                params![caste_id, tag_id, position, odor_level],
            )
            .await?;
        }
        CasteTag::PenetratePower { penetrate_power } => {
            conn.execute(
                super::INSERT_CASTE_INTEGER_FLAG,
                params![caste_id, tag_id, position, penetrate_power],
            )
            .await?;
        }
        CasteTag::PetValue { pet_value } => {
            conn.execute(
                super::INSERT_CASTE_INTEGER_FLAG,
                params![caste_id, tag_id, position, pet_value],
            )
            .await?;
        }
        CasteTag::PetValueDivisor { divisor } => {
            conn.execute(
                super::INSERT_CASTE_INTEGER_FLAG,
                params![caste_id, tag_id, position, divisor],
            )
            .await?;
        }
        CasteTag::PopulationRatio { pop_ratio } => {
            conn.execute(
                super::INSERT_CASTE_INTEGER_FLAG,
                params![caste_id, tag_id, position, pop_ratio],
            )
            .await?;
        }
        CasteTag::ProneToRage { rage_chance } => {
            conn.execute(
                super::INSERT_CASTE_INTEGER_FLAG,
                params![caste_id, tag_id, position, rage_chance],
            )
            .await?;
        }
        CasteTag::ItemCorpseQuality { quality } => {
            conn.execute(
                super::INSERT_CASTE_INTEGER_FLAG,
                params![caste_id, tag_id, position, quality],
            )
            .await?;
        }
        CasteTag::SkillLearnRates { rate } => {
            conn.execute(
                super::INSERT_CASTE_INTEGER_FLAG,
                params![caste_id, tag_id, position, rate],
            )
            .await?;
        }
        CasteTag::TradeCapacity { capacity } => {
            conn.execute(
                super::INSERT_CASTE_INTEGER_FLAG,
                params![caste_id, tag_id, position, capacity],
            )
            .await?;
        }
        CasteTag::ViewRange { view_range } => {
            conn.execute(
                super::INSERT_CASTE_INTEGER_FLAG,
                params![caste_id, tag_id, position, view_range],
            )
            .await?;
        }
        //  B.3: Range flags
        CasteTag::ClutchSize { min, max }
        | CasteTag::LitterSize { min, max }
        | CasteTag::MaxAge { min, max } => {
            conn.execute(
                super::INSERT_CASTE_MIN_MAX_FLAG,
                params![caste_id, tag_id, position, min, max],
            )
            .await?;
        }
        CasteTag::GeneralMaterialForceMultiplier { value_a, value_b } => {
            conn.execute(
                super::INSERT_CASTE_MIN_MAX_FLAG,
                params![caste_id, tag_id, position, value_a, value_b],
            )
            .await?;
        }
        //  B.4: 7-spread range flags
        CasteTag::MentalAttributeRange { attribute, ranges }
        | CasteTag::PhysicalAttributeRange { attribute, ranges } => {
            conn.execute(
                super::INSERT_CASTE_7SPREAD_RANGE_FLAG,
                params![
                    caste_id,
                    tag_id,
                    position,
                    attribute.as_str(),
                    ranges[0],
                    ranges[1],
                    ranges[2],
                    ranges[3],
                    ranges[4],
                    ranges[5],
                    ranges[6],
                ],
            )
            .await?;
        }
        CasteTag::BodyAppearanceModifier { attribute, values } => {
            conn.execute(
                super::INSERT_CASTE_7SPREAD_RANGE_FLAG,
                params![
                    caste_id,
                    tag_id,
                    position,
                    attribute.as_str(),
                    values[0],
                    values[1],
                    values[2],
                    values[3],
                    values[4],
                    values[5],
                    values[6],
                ],
            )
            .await?;
        }
        //  B.5: String - Integer flags
        CasteTag::SyndromeDilutionFactor {
            syndrome,
            percentage,
        } => {
            conn.execute(
                super::INSERT_CASTE_STRING_INTEGER_FLAG,
                params![caste_id, tag_id, position, syndrome.as_str(), percentage],
            )
            .await?;
        }
        CasteTag::Tendons {
            material,
            healing_rate,
        }
        | CasteTag::Ligaments {
            material,
            healing_rate,
        } => {
            conn.execute(
                super::INSERT_CASTE_STRING_INTEGER_FLAG,
                params![caste_id, tag_id, position, material.as_str(), healing_rate,],
            )
            .await?;
        }
        CasteTag::NaturalSkill { skill, level } => {
            conn.execute(
                super::INSERT_CASTE_STRING_INTEGER_FLAG,
                params![caste_id, tag_id, position, skill.as_str(), level],
            )
            .await?;
        }
        CasteTag::MentalAttributeCapPercentage {
            attribute,
            percentage,
        }
        | CasteTag::PhysicalAttributeCapPercentage {
            attribute,
            percentage,
        } => {
            conn.execute(
                super::INSERT_CASTE_STRING_INTEGER_FLAG,
                params![caste_id, tag_id, position, attribute.as_str(), percentage,],
            )
            .await?;
        }
        CasteTag::Milkable {
            material,
            frequency,
        } => {
            conn.execute(
                super::INSERT_CASTE_STRING_INTEGER_FLAG,
                params![caste_id, tag_id, position, material.as_str(), frequency,],
            )
            .await?;
        }
        //  B.6: String - String flags
        CasteTag::MannerismFingers { finger, fingers } => {
            conn.execute(
                super::INSERT_CASTE_STRING_STRING_FLAG,
                params![
                    caste_id,
                    tag_id,
                    position,
                    finger.as_str(),
                    fingers.as_str()
                ],
            )
            .await?;
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
                super::INSERT_SPECIFIC_FOOD,
                params![
                    caste_id,
                    tag_id,
                    position,
                    food_type_object_type_id,
                    identifier.as_str()
                ],
            )
            .await?;
        }
        // Case C: Properties in special tables
        CasteTag::Attack { name, body_part } => {
            conn.execute(
                super::INSERT_CASTE_ATTACK,
                params![caste_id, position, name.as_str(), body_part.as_str()],
            )
            .await?;
        }
        CasteTag::AttackTrigger {
            population,
            exported_wealth,
            created_wealth,
        } => {
            conn.execute(
                super::INSERT_CASTE_ATTACK_TRIGGER,
                params![
                    caste_id,
                    position,
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
                super::INSERT_BODY_DETAIL_PLAN_IDENTITY,
                params![caste_id, position, body_plan.as_str()],
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
                super::INSERT_CASTE_ITEM_TAG,
                params![caste_id, tag_id, dyn_item_id, position],
            )
            .await?;
        }
        CasteTag::Pus { material, state }
        | CasteTag::Blood { material, state }
        | CasteTag::EggMaterial { material, state } => {
            let dyn_mat_id = get_or_insert_dynamic_material_in_state(conn, material, state).await?;

            conn.execute(
                super::INSERT_CASTE_MATERIAL_TAG,
                params![caste_id, tag_id, dyn_mat_id, position],
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
                super::INSERT_CASTE_CREATURE_CASTE_REF_TAG,
                params![caste_id, creature_caste_tag_id, position],
            )
            .await?;
        }
        CasteTag::Lair { lair, probability } => {
            let ref_lair_id = get_ref_lair_token_id(conn, lair).await?;

            if let Some(id) = ref_lair_id {
                conn.execute(
                    super::INSERT_CASTE_LAIR_REF_TAG,
                    params![caste_id, tag_id, id, position, probability],
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
            conn.execute(
                super::INSERT_CASTE_STRING_INTEGER_INTEGER_INTEGER_FLAG,
                params![
                    caste_id,
                    tag_id,
                    position,
                    improvement_rate,
                    decay_rate_unused,
                    decay_rate_rusty,
                    decay_rate_demotion,
                ],
            )
            .await?;
        }
        CasteTag::SkillRustRates {
            decay_rate_unused,
            decay_rate_rusty,
            decay_rate_demotion,
        } => {
            conn.execute(
                super::INSERT_CASTE_INTEGER_INTEGER_INTEGER_FLAG,
                params![
                    caste_id,
                    tag_id,
                    position,
                    decay_rate_unused,
                    decay_rate_rusty,
                    decay_rate_demotion,
                ],
            )
            .await?;
        }
        CasteTag::SkillRustRate {
            skill,
            decay_rate_unused,
            decay_rate_rusty,
            decay_rate_demotion,
        } => {
            conn.execute(
                super::INSERT_CASTE_STRING_INTEGER_INTEGER_INTEGER_FLAG,
                params![
                    caste_id,
                    tag_id,
                    position,
                    skill.as_str(),
                    decay_rate_unused,
                    decay_rate_rusty,
                    decay_rate_demotion,
                ],
            )
            .await?;
        }
        CasteTag::SkillRate {
            skill,
            improvement_rate,
            decay_rate_unused,
            decay_rate_rusty,
            decay_rate_demotion,
        } => {
            conn.execute(
                super::INSERT_CASTE_STRING_INTEGER_INTEGER_INTEGER_INTEGER_FLAG,
                params![
                    caste_id,
                    tag_id,
                    position,
                    skill.as_str(),
                    improvement_rate,
                    decay_rate_unused,
                    decay_rate_rusty,
                    decay_rate_demotion,
                ],
            )
            .await?;
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
            conn.execute(
                super::INSERT_CASTE_STRING_INTEGER_INTEGER_INTEGER_INTEGER_FLAG,
                params![
                    caste_id,
                    tag_id,
                    position,
                    attribute.as_str(),
                    improvement_cost,
                    decay_rate_unused,
                    decay_rate_rusty,
                    decay_rate_demotion,
                ],
            )
            .await?;
        }
        CasteTag::Orientation {
            caste,
            disinterested_chance,
            casual_chance,
            strong_chance,
        } => {
            conn.execute(
                super::INSERT_CASTE_STRING_INTEGER_INTEGER_INTEGER_FLAG,
                params![
                    caste_id,
                    tag_id,
                    position,
                    caste.as_str(),
                    disinterested_chance,
                    casual_chance,
                    strong_chance,
                ],
            )
            .await?;
        }
        CasteTag::Personality {
            personality_trait,
            low,
            median,
            high,
        } => {
            conn.execute(
                super::INSERT_CASTE_STRING_INTEGER_INTEGER_INTEGER_FLAG,
                params![
                    caste_id,
                    tag_id,
                    position,
                    personality_trait.as_str(),
                    low,
                    median,
                    high,
                ],
            )
            .await?;
        }
        CasteTag::PlusBodyPartGroup {
            body_part_selector,
            body_part_group,
        } => {
            conn.execute(
                super::INSERT_CASTE_STRING_STRING_FLAG,
                params![
                    caste_id,
                    tag_id,
                    position,
                    body_part_selector.as_str(),
                    body_part_group.as_str(),
                ],
            )
            .await?;
        }
        CasteTag::ProfessionName {
            profession,
            singular,
            plural,
        } => {
            let dyn_name_id =
                get_or_insert_dynamic_name(conn, singular.as_str(), plural.as_str(), None).await?;

            conn.execute(
                super::INSERT_CASTE_NAME_TAG,
                params![caste_id, tag_id, dyn_name_id, position],
            )
            .await?;

            let caste_name_id =
                get_caste_name_id_by_caste_id_and_tag_position(conn, caste_id, position).await?;

            conn.execute(
                super::INSERT_CASTE_PROFESSION_NAME_TAG,
                params![caste_name_id, profession.as_str(), tag_id, position],
            )
            .await?;
        }
        CasteTag::Remains { singular, plural }
        | CasteTag::BabyName { singular, plural }
        | CasteTag::ChildName { singular, plural } => {
            let dyn_name_id =
                get_or_insert_dynamic_name(conn, singular.as_str(), plural.as_str(), None).await?;

            conn.execute(
                super::INSERT_CASTE_NAME_TAG,
                params![caste_id, tag_id, dyn_name_id, position],
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
                super::INSERT_CASTE_NAME_TAG,
                params![caste_id, tag_id, dyn_name_id, position],
            )
            .await?;
        }
        CasteTag::RelativeSize {
            body_part_selector,
            body_part,
            relative_size,
        } => {
            conn.execute(
                super::INSERT_CASTE_STRING_STRING_INTEGER_FLAG,
                params![
                    caste_id,
                    tag_id,
                    position,
                    body_part_selector.as_str(),
                    body_part.as_str(),
                    relative_size,
                ],
            )
            .await?;
        }
        CasteTag::RetractIntoBodyPart {
            body_part_selector,
            body_part,
            second_person,
            third_person,
            second_person_cancel,
            third_person_cancel,
        } => {
            conn.execute(
                super::INSERT_CASTE_STRING_STRING_STRING_STRING_STRING_STRING_FLAG,
                params![
                    caste_id,
                    tag_id,
                    position,
                    body_part_selector.as_str(),
                    body_part.as_str(),
                    second_person.as_str(),
                    third_person.as_str(),
                    second_person_cancel.as_str(),
                    third_person_cancel.as_str()
                ],
            )
            .await?;
        }
        CasteTag::RootAround {
            body_part_selector,
            body_part,
            second_person_verb,
            third_person_verb,
        } => {
            conn.execute(
                super::INSERT_CASTE_STRING_STRING_STRING_STRING_FLAG,
                params![
                    caste_id,
                    tag_id,
                    position,
                    body_part_selector.as_str(),
                    body_part.as_str(),
                    second_person_verb.as_str(),
                    third_person_verb.as_str(),
                ],
            )
            .await?;
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
                super::INSERT_CASTE_SECRETION,
                params![
                    caste_id,
                    tag_id,
                    position,
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
            conn.execute(
                super::INSERT_CASTE_STRING_STRING_INTEGER_INTEGER_INTEGER_FLAG,
                params![
                    caste_id,
                    tag_id,
                    position,
                    creature_class.as_str(),
                    tile.as_str(),
                    foreground,
                    background,
                    brightness,
                ],
            )
            .await?;
        }
        CasteTag::SetBodyPartGroup {
            body_part_selector,
            body_part,
        } => {
            conn.execute(
                super::INSERT_CASTE_STRING_STRING_FLAG,
                params![
                    caste_id,
                    tag_id,
                    position,
                    body_part_selector.as_str(),
                    body_part.as_str(),
                ],
            )
            .await?;
        }
        CasteTag::Sound {
            sound_type,
            sound_range,
            sound_interval,
            requires_breathing,
            first_person,
            third_person,
            out_of_sight,
        } => todo!(),
        CasteTag::TissueLayer {
            body_part_selector,
            body_part,
            tissue,
            location,
        } => todo!(),
        CasteTag::TissueLayerUnder {
            body_part_selector,
            body_part,
            tissue,
        } => todo!(),
        CasteTag::VerminBite {
            chance,
            verb,
            material,
            material_state,
        } => todo!(),
        CasteTag::VisionArc {
            binocular,
            non_binocular,
        } => todo!(),
        CasteTag::SkillLearnRate { skill, rate } => todo!(),
        CasteTag::BodySize { year, days, size } => todo!(),
        CasteTag::BodyPartAppearanceModifier { quality, spread } => todo!(),
        CasteTag::ApplyCreatureVariation { id, args } => todo!(),
        CasteTag::ExtraButcherObject {
            object_type,
            arguments,
        } => todo!(),
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
                super::INSERT_CASTE_COLOR_TAG,
                params![caste_id, tag_id, color_id, position],
            )
            .await?;
        }
        CasteTag::InteractionDetail { args } => todo!(),
        CasteTag::Body { body_parts } => todo!(),
    }

    Ok(())
}
