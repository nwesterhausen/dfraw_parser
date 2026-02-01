use std::{
    collections::HashMap,
    mem::{Discriminant, discriminant},
    sync::OnceLock,
};

use crate::{raw_definitions::CASTE_TOKENS, tokens::CasteToken, traits::RawToken};

impl RawToken for CasteToken {
    fn get_key(&self) -> Option<&'static str> {
        // Define static storage for the reverse map
        static REVERSE_MAP: OnceLock<HashMap<Discriminant<CasteToken>, &'static str>> =
            OnceLock::new();

        // Initialize it lazily (only runs once)
        let map = REVERSE_MAP.get_or_init(|| {
            let mut m = HashMap::new();
            // Iterate the existing PHF map
            for (key, tag_template) in &CASTE_TOKENS {
                // Key: The Enum Variant (Discriminant)
                // Value: The String Token (e.g., "FLIER")
                m.insert(discriminant(tag_template), *key);
            }
            m
        });

        // Lookup the key using the discriminant of 'self'
        map.get(&discriminant(self)).copied()
    }

    fn to_raw_token(&self) -> String {
        let key = match self.get_key() {
            Some(key) => key,
            None => return String::new(),
        };

        match self {
            CasteToken::AltTile { tile } => format!("[{key}:{tile}]"),
            CasteToken::ApplyCreatureVariation { id, args } => {
                if args.is_empty() {
                    format!("[{key}:{id}]")
                } else {
                    let args_str = args
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(":");
                    format!("[{key}:{id}:{args_str}]")
                }
            }
            CasteToken::Attack { verb, selector } => {
                format!("[{key}:{verb}:{}]", selector.join(":"))
            }
            CasteToken::AttackTrigger {
                population,
                exported_wealth,
                created_wealth,
            } => format!("[{key}:{population}:{exported_wealth}:{created_wealth}]"),
            CasteToken::Baby { age } => format!("[{key}:{age}]"),
            CasteToken::BabyName { name } => format!("[{key}:{}]", name.as_vec().join(":")),
            CasteToken::BeachFrequency { frequency } => format!("[{key}:{frequency}]"),
            CasteToken::Blood { material, state } => {
                format!("[{key}:{}:{state}]", material.join(":"))
            }
            CasteToken::Body { body_parts } => format!("[{key}:{}]", body_parts.join(":")),
            CasteToken::BodyAppearanceModifier { attribute, values } => {
                let values_str = values
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(":");
                format!("[{key}:{attribute}:{values_str}]")
            }
            CasteToken::BodyDetailPlan {
                body_plan,
                arguments,
            } => {
                if arguments.is_empty() {
                    format!("[{key}:{body_plan}]")
                } else {
                    format!("[{key}:{body_plan}:{}]", arguments.join(":"))
                }
            }
            CasteToken::BodySize { size } => format!("[{key}:{}]", size.as_value()),
            CasteToken::BodyGloss { gloss } => format!("[{key}:{gloss}]"),
            CasteToken::BodyPartAddType { body_part_type } => format!("[{key}:{body_part_type}]"),
            CasteToken::BodyPartAppearanceModifier { quality, spread } => {
                let spread_str = spread
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(":");
                format!("[{key}:{quality}:{spread_str}]")
            }
            CasteToken::BodyPartRemoveType { body_part_type } => {
                format!("[{key}:{body_part_type}]")
            }
            CasteToken::BuildingDestroyer {
                door_and_furniture_focused,
            } => format!("[{key}:{door_and_furniture_focused}]"),
            CasteToken::CanDoInteraction { interaction } => format!("[{key}:{interaction}]"),
            CasteToken::ChangeBodySizePercent { percent } => format!("[{key}:{percent}]"),
            CasteToken::Child { age } => format!("[{key}:{age}]"),
            CasteToken::ChildName { name } => format!("[{key}:{}]", name.as_vec().join(":")),
            CasteToken::ClutchSize { min, max } => format!("[{key}:{min}:{max}]"),
            CasteToken::Color { color } => format!("[{key}:{}]", color.as_value()),
            CasteToken::CreatureClass { class } => format!("[{key}:{class}]"),
            CasteToken::CreatureVariationAddTag { tag } => format!("[{key}:{tag}]"),
            CasteToken::CreatureVariationRemoveTag { tag } => format!("[{key}:{tag}]"),
            CasteToken::Description { description } => format!("[{key}:{description}]"),
            CasteToken::Difficulty { difficulty } => format!("[{key}:{difficulty}]"),
            CasteToken::ExtraButcherObjectItem { item, material } => {
                format!("[{key}:{item}:{}]", material.join(":"))
            }
            CasteToken::ExtraButcherObjectShape { shape } => format!("[{key}:{shape}]"),
            CasteToken::EggMaterial { material, state } => {
                format!("[{key}:{}:{state}]", material.join(":"))
            }
            CasteToken::EggSize { size } => format!("[{key}:{size}]"),
            CasteToken::ExtraButcherObject {
                object_type,
                arguments,
            } => {
                if arguments.is_empty() {
                    format!("[{key}:{object_type}]")
                } else {
                    format!("[{key}:{object_type}:{}]", arguments.join(":"))
                }
            }
            CasteToken::Extract { material } => format!("[{key}:{material}]"),
            CasteToken::FixedTemp { temperature } => format!("[{key}:{temperature}]"),
            CasteToken::Gait { gait_values } => format!("[{key}:{}]", gait_values.join(":")),
            CasteToken::GeneralMaterialForceMultiplier { value_a, value_b } => {
                format!("[{key}:{value_a}:{value_b}]")
            }
            CasteToken::GlowColor { color } => format!("[{key}:{}]", color.as_value()),
            CasteToken::GlowTile { tile } => format!("[{key}:{tile}]"),
            CasteToken::Gnawer { verb } => format!("[{key}:{verb}]"),
            CasteToken::GobbleVerminClass { vermin_class } => format!("[{key}:{vermin_class}]"),
            CasteToken::GobbleVerminCreature {
                vermin_creature,
                vermin_caste,
            } => format!("[{key}:{vermin_creature}:{vermin_caste}]"),
            CasteToken::GrassTrample { trample } => format!("[{key}:{trample}]"),
            CasteToken::GravitateBodySize { target } => format!("[{key}:{target}]"),
            CasteToken::Grazer { grazer } => format!("[{key}:{grazer}]"),
            CasteToken::Habit { habit } => format!("[{key}:{habit}]"),
            CasteToken::HabitNumber { number } => format!("[{key}:{number}]"),
            CasteToken::Homeotherm { temperature } => format!("[{key}:{temperature}]"),
            CasteToken::InteractionDetail { label, args } => {
                if args.is_empty() {
                    format!("[{key}:{label}]")
                } else {
                    format!("[{key}:{label}:{}]", args.join(":"))
                }
            }
            CasteToken::ItemCorpse { item, material } => {
                format!("[{key}:{item}:{}]", material.join(":"))
            }
            CasteToken::ItemCorpseQuality { quality } => format!("[{key}:{quality}]"),
            CasteToken::Lair { lair, probability } => format!("[{key}:{lair}:{probability}]"),
            CasteToken::LairCharacteristic { characteristic } => {
                format!("[{key}:{characteristic}]")
            }
            CasteToken::LairHunterSpeech { speech_file } => format!("[{key}:{speech_file}]"),
            CasteToken::LaysUnusualEggs { item, material } => {
                format!("[{key}:{item}:{}]", material.join(":"))
            }
            CasteToken::Ligaments {
                material,
                healing_rate,
            } => format!("[{key}:{}:{healing_rate}]", material.join(":")),
            CasteToken::LitterSize { min, max } => format!("[{key}:{min}:{max}]"),
            CasteToken::LowLightVision { vision } => format!("[{key}:{vision}]"),
            CasteToken::MannerismFingers { finger, fingers } => {
                format!("[{key}:{finger}:{fingers}]")
            }
            CasteToken::MannerismNose { nose } => format!("[{key}:{nose}]"),
            CasteToken::MannerismEar { ear } => format!("[{key}:{ear}]"),
            CasteToken::MannerismHead { head } => format!("[{key}:{head}]"),
            CasteToken::MannerismEyes { eyes } => format!("[{key}:{eyes}]"),
            CasteToken::MannerismMouth { mouth } => format!("[{key}:{mouth}]"),
            CasteToken::MannerismHair { hair } => format!("[{key}:{hair}]"),
            CasteToken::MannerismKnuckles { knuckles } => format!("[{key}:{knuckles}]"),
            CasteToken::MannerismLips { lips } => format!("[{key}:{lips}]"),
            CasteToken::MannerismCheek { cheek } => format!("[{key}:{cheek}]"),
            CasteToken::MannerismNails { nails } => format!("[{key}:{nails}]"),
            CasteToken::MannerismFeet { feet } => format!("[{key}:{feet}]"),
            CasteToken::MannerismArms { arms } => format!("[{key}:{arms}]"),
            CasteToken::MannerismHands { hands } => format!("[{key}:{hands}]"),
            CasteToken::MannerismTongue { tongue } => format!("[{key}:{tongue}]"),
            CasteToken::MannerismLeg { leg } => format!("[{key}:{leg}]"),
            CasteToken::MaxAge { min, max } => format!("[{key}:{min}:{max}]"),
            CasteToken::MentalAttributeCapPercentage {
                attribute,
                percentage,
            } => format!("[{key}:{attribute}:{percentage}]"),
            CasteToken::MentalAttributeRange { attribute, ranges } => {
                let ranges_str = ranges
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(":");
                format!("[{key}:{attribute}:{ranges_str}]")
            }
            CasteToken::MentalAttributeRate {
                attribute,
                improvement_cost,
                decay_rate_unused,
                decay_rate_rusty,
                decay_rate_demotion,
            } => format!(
                "[{key}:{attribute}:{improvement_cost}:{decay_rate_unused}:{decay_rate_rusty}:{decay_rate_demotion}]"
            ),
            CasteToken::Milkable {
                material,
                frequency,
            } => format!("[{key}:{}:{frequency}]", material.join(":")),
            CasteToken::ModValue { value } => format!("[{key}:{value}]"),
            CasteToken::Name { name } => format!("[{key}:{}]", name.as_vec().join(":")),
            CasteToken::NaturalSkill { skill, level } => format!("[{key}:{skill}:{level}]"),
            CasteToken::OdorLevel { odor_level } => format!("[{key}:{odor_level}]"),
            CasteToken::OdorString { odor_string } => format!("[{key}:{odor_string}]"),
            CasteToken::Orientation {
                caste,
                disinterested_chance,
                casual_chance,
                strong_chance,
            } => format!("[{key}:{caste}:{disinterested_chance}:{casual_chance}:{strong_chance}]"),
            CasteToken::PenetratePower { penetrate_power } => format!("[{key}:{penetrate_power}]"),
            CasteToken::Personality {
                personality_trait,
                low,
                median,
                high,
            } => format!("[{key}:{personality_trait}:{low}:{median}:{high}]"),
            CasteToken::PetValue { pet_value } => format!("[{key}:{pet_value}]"),
            CasteToken::PetValueDivisor { divisor } => format!("[{key}:{divisor}]"),
            CasteToken::PhysicalAttributeCapPercentage {
                attribute,
                percentage,
            } => format!("[{key}:{attribute}:{percentage}]"),
            CasteToken::PhysicalAttributeRange { attribute, ranges } => {
                let ranges_str = ranges
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(":");
                format!("[{key}:{attribute}:{ranges_str}]")
            }
            CasteToken::PhysicalAttributeRate {
                attribute,
                improvement_cost,
                decay_rate_unused,
                decay_rate_rusty,
                decay_rate_demotion,
            } => format!(
                "[{key}:{attribute}:{improvement_cost}:{decay_rate_unused}:{decay_rate_rusty}:{decay_rate_demotion}]"
            ),
            CasteToken::PlusBodyPartGroup { selector } => {
                format!("[{key}:{}]", selector.join(":"))
            }
            CasteToken::PopulationRatio { pop_ratio } => format!("[{key}:{pop_ratio}]"),
            CasteToken::ProfessionName { profession, name } => {
                format!("[{key}:{profession}:{}]", name.as_vec().join(":"))
            }
            CasteToken::ProneToRage { rage_chance } => format!("[{key}:{rage_chance}]"),
            CasteToken::Pus { material, state } => {
                format!("[{key}:{}:{state}]", material.join(":"))
            }
            CasteToken::RelativeSize {
                selector,
                relative_size,
            } => format!("[{key}:{}:{relative_size}]", selector.join(":")),
            CasteToken::Remains { singular, plural } => format!("[{key}:{singular}:{plural}]"),
            CasteToken::RemainsColor { remains_color } => format!("[{key}:{remains_color}]"),
            CasteToken::RetractIntoBodyPart {
                body_part_selector,
                body_part,
                second_person,
                third_person,
                second_person_cancel,
                third_person_cancel,
            } => format!(
                "[{key}:{body_part_selector}:{body_part}:{second_person}:{third_person}:{second_person_cancel}:{third_person_cancel}]"
            ),
            CasteToken::RootAround {
                body_part_selector,
                second_person_verb,
                third_person_verb,
            } => format!(
                "[{key}:{}:{second_person_verb}:{third_person_verb}]",
                body_part_selector.join(":")
            ),
            CasteToken::Secretion {
                material,
                material_state,
                body_part_selector,
                tissue_layer,
                trigger,
            } => format!(
                "[{key}:{}:{material_state}:{}:{tissue_layer}:{trigger}]",
                material.join(":"),
                body_part_selector.join(":")
            ),
            CasteToken::SenseCreatureClass {
                creature_class,
                tile,
                foreground,
                background,
                brightness,
            } => format!("[{key}:{creature_class}:{tile}:{foreground}:{background}:{brightness}]"),
            CasteToken::SetBodyPartGroup { body_part_selector } => {
                format!("[{key}:{}]", body_part_selector.join(":"))
            }
            CasteToken::SkillLearnRate { skill, rate } => format!("[{key}:{skill}:{rate}]"),
            CasteToken::SkillLearnRates { rate } => format!("[{key}:{rate}]"),
            CasteToken::SkillRate {
                skill,
                improvement_rate,
                decay_rate_unused,
                decay_rate_rusty,
                decay_rate_demotion,
            } => format!(
                "[{key}:{skill}:{improvement_rate}:{decay_rate_unused}:{decay_rate_rusty}:{decay_rate_demotion}]"
            ),
            CasteToken::SkillRates {
                improvement_rate,
                decay_rate_unused,
                decay_rate_rusty,
                decay_rate_demotion,
            } => format!(
                "[{key}:{improvement_rate}:{decay_rate_unused}:{decay_rate_rusty}:{decay_rate_demotion}]"
            ),
            CasteToken::SkillRustRate {
                skill,
                decay_rate_unused,
                decay_rate_rusty,
                decay_rate_demotion,
            } => format!(
                "[{key}:{skill}:{decay_rate_unused}:{decay_rate_rusty}:{decay_rate_demotion}]"
            ),
            CasteToken::SkillRustRates {
                decay_rate_unused,
                decay_rate_rusty,
                decay_rate_demotion,
            } => format!("[{key}:{decay_rate_unused}:{decay_rate_rusty}:{decay_rate_demotion}]"),
            CasteToken::SlainSpeech { speech_file } => format!("[{key}:{speech_file}]"),
            CasteToken::SoldierTile { tile } => format!("[{key}:{tile}]"),
            CasteToken::SoldierAltTile { tile } => format!("[{key}:{tile}]"),
            CasteToken::Sound {
                sound_type,
                sound_range,
                sound_interval,
                requires_breathing,
                first_person,
                third_person,
                out_of_sight,
            } => {
                let breath = if *requires_breathing {
                    "VOCALIZATION"
                } else {
                    "NONE"
                };
                format!(
                    "[{key}:{sound_type}:{sound_range}:{sound_interval}:{breath}:{first_person}:{third_person}:{out_of_sight}]"
                )
            }
            CasteToken::SpecificFood {
                food_type,
                identifier,
            } => format!("[{key}:{food_type}:{identifier}]"),
            CasteToken::SyndromeDilutionFactor {
                syndrome,
                percentage,
            } => format!("[{key}:{syndrome}:{percentage}]"),
            CasteToken::Tendons {
                material,
                healing_rate,
            } => format!("[{key}:{}:{healing_rate}]", material.join(":")),
            CasteToken::Tile { tile } => format!("[{key}:{tile}]"),
            CasteToken::TissueLayer {
                body_part_selector,
                tissue,
                positioning,
            } => {
                if positioning.is_empty() {
                    format!("[{key}:{}:{tissue}]", body_part_selector.join(":"))
                } else {
                    format!(
                        "[{key}:{}:{tissue}:{}]",
                        body_part_selector.join(":"),
                        positioning.join(":")
                    )
                }
            }
            CasteToken::TissueLayerUnder {
                body_part_selector,
                body_part,
                tissue,
            } => format!("[{key}:{body_part_selector}:{body_part}:{tissue}]"),
            CasteToken::TradeCapacity { capacity } => format!("[{key}:{capacity}]"),
            CasteToken::VerminBite {
                chance,
                verb,
                material,
                material_state,
            } => format!(
                "[{key}:{chance}:{verb}:{}:{material_state}]",
                material.join(":")
            ),
            CasteToken::ViewRange { view_range } => format!("[{key}:{view_range}]"),
            CasteToken::VisionArc {
                binocular,
                non_binocular,
            } => format!("[{key}:{binocular}:{non_binocular}]"),
            CasteToken::Webber { material } => format!("[{key}:{}]", material.join(":")),
            CasteToken::AdoptsOwner
            | CasteToken::AlcoholDependent
            | CasteToken::AllActive
            | CasteToken::AmbushPredator
            | CasteToken::Amphibious
            | CasteToken::ApplyCurrentCreatureVariation
            | CasteToken::Aquatic
            | CasteToken::ArenaRestricted
            | CasteToken::AtPeaceWithWildlife
            | CasteToken::Benign
            | CasteToken::BloodSucker
            | CasteToken::BoneCarn
            | CasteToken::CanLearn
            | CasteToken::CanSpeak
            | CasteToken::CannotClimb
            | CasteToken::CannotJump
            | CasteToken::CannotUndead
            | CasteToken::CanOpenDoors
            | CasteToken::Carnivore
            | CasteToken::CaveAdaptation
            | CasteToken::CommonDomestic
            | CasteToken::ConvertedSpouse
            | CasteToken::CookableLive
            | CasteToken::Crazed
            | CasteToken::Crepuscular
            | CasteToken::CuriousBeastEater
            | CasteToken::CuriousBeastGuzzler
            | CasteToken::CuriousBeastItem
            | CasteToken::Demon
            | CasteToken::DieWhenVerminBite
            | CasteToken::Diurnal
            | CasteToken::DiveHuntsVermin
            | CasteToken::Equips
            | CasteToken::Extravision
            | CasteToken::FeatureAttackGroup
            | CasteToken::FeatureBeast
            | CasteToken::Female
            | CasteToken::FireImmune
            | CasteToken::FireImmuneSuper
            | CasteToken::FishItem
            | CasteToken::FleeQuick
            | CasteToken::Flier
            | CasteToken::GetsInfectionsFromRot
            | CasteToken::GetsWoundInfections
            | CasteToken::HasNerves
            | CasteToken::HasShell
            | CasteToken::HuntsVermin
            | CasteToken::Immobile
            | CasteToken::ImmobileLand
            | CasteToken::Immolate
            | CasteToken::Intelligent
            | CasteToken::LairHunter
            | CasteToken::LargePredator
            | CasteToken::LaysEggs
            | CasteToken::LightGen
            | CasteToken::LikesFighting
            | CasteToken::Lisp
            | CasteToken::LockPicker
            | CasteToken::Magical
            | CasteToken::MagmaVision
            | CasteToken::Male
            | CasteToken::MannerismLaugh
            | CasteToken::MannerismSmile
            | CasteToken::MannerismWalk
            | CasteToken::MannerismSit
            | CasteToken::MannerismBreath
            | CasteToken::MannerismPosture
            | CasteToken::MannerismStretch
            | CasteToken::MannerismEyelids
            | CasteToken::Matutinal
            | CasteToken::Meanderer
            | CasteToken::Megabeast
            | CasteToken::Mischievous
            | CasteToken::Mount
            | CasteToken::MountExotic
            | CasteToken::MultipartFullVision
            | CasteToken::MultipleLitterRare
            | CasteToken::Natural
            | CasteToken::NightCreatureBogeyman
            | CasteToken::NightCreatureExperimenter
            | CasteToken::NightCreatureHunter
            | CasteToken::NightCreatureNightmare
            | CasteToken::NoConnectionsForMovement
            | CasteToken::NoDizziness
            | CasteToken::NoDrink
            | CasteToken::NoEat
            | CasteToken::NoFall
            | CasteToken::NoFevers
            | CasteToken::NoGender
            | CasteToken::NoPhysicalAttributeGain
            | CasteToken::NoPhysicalAttributeRust
            | CasteToken::NoSleep
            | CasteToken::NoSpring
            | CasteToken::NoSummer
            | CasteToken::NoThoughtCenterForMovement
            | CasteToken::NoUnitTypeColor
            | CasteToken::NoVegetationDisturbance
            | CasteToken::NoWinter
            | CasteToken::NoBones
            | CasteToken::NoBreathe
            | CasteToken::Nocturnal
            | CasteToken::NoEmotion
            | CasteToken::NoExert
            | CasteToken::NoFear
            | CasteToken::NoMeat
            | CasteToken::NoNausea
            | CasteToken::NoPain
            | CasteToken::NoSkin
            | CasteToken::NoSkull
            | CasteToken::NoSmellyRot
            | CasteToken::NoStuckIns
            | CasteToken::NoStun
            | CasteToken::NotButcherable
            | CasteToken::NotLiving
            | CasteToken::NoThought
            | CasteToken::OpposedToLife
            | CasteToken::OutsiderControllable
            | CasteToken::PackAnimal
            | CasteToken::ParalyzeImmune
            | CasteToken::PatternFlier
            | CasteToken::Pearl
            | CasteToken::Pet
            | CasteToken::PetExotic
            | CasteToken::Power
            | CasteToken::RemainsOnVerminBiteDeath
            | CasteToken::RemainsUndetermined
            | CasteToken::ReturnsVerminKillsToOwner
            | CasteToken::SemiMegabeast
            | CasteToken::SlowLearner
            | CasteToken::SmallRemains
            | CasteToken::SpouseConversionTarget
            | CasteToken::SpouseConverter
            | CasteToken::SpreadEvilSpheresIfRuler
            | CasteToken::StanceClimber
            | CasteToken::StandardGrazer
            | CasteToken::StrangeMoods
            | CasteToken::Supernatural
            | CasteToken::SwimsInnate
            | CasteToken::SwimsLearned
            | CasteToken::ThickWeb
            | CasteToken::Titan
            | CasteToken::Trainable
            | CasteToken::TrainableHunting
            | CasteToken::TrainableWar
            | CasteToken::Trances
            | CasteToken::TrapAvoid
            | CasteToken::UnderSwim
            | CasteToken::UniqueDemon
            | CasteToken::Vegetation
            | CasteToken::VerminHateable
            | CasteToken::VerminMicro
            | CasteToken::VerminNoFish
            | CasteToken::VerminNoRoam
            | CasteToken::VerminNoTrap
            | CasteToken::VerminHunter
            | CasteToken::Vespertine
            | CasteToken::WagonPuller
            | CasteToken::WebImmune
            | CasteToken::Unknown
            | CasteToken::NightCreature
            | CasteToken::NotFireImmune
            | CasteToken::HasBlood
            | CasteToken::Grasp
            | CasteToken::RaceGait
            | CasteToken::CannotBreatheWater
            | CasteToken::NaturalAnimal
            | CasteToken::CuriousBeast
            | CasteToken::CannotBreatheAir => format!("[{key}]"),
        }
    }
}
