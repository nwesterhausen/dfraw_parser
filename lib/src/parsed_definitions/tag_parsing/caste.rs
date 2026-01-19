//! The parsing implementation for `CasteTag`

use crate::{
    raw_definitions::{CASTE_TOKENS, OBJECT_TOKEN_MAP},
    tags::CasteTag,
    traits::{TagOperations, TokenParser},
};

impl TagOperations for CasteTag {
    fn parse(key: &str, value: &str) -> Option<Self>
    where
        Self: Sized,
    {
        let Some(token) = CASTE_TOKENS.get(key) else {
            tracing::error!("parse_token: unknown token: {}", key);
            return None;
        };

        // Split values into an array if possible, using an empty one if no values exist.
        let values: Vec<&str> = if value.is_empty() {
            Vec::new()
        } else {
            value.split(':').collect()
        };

        match token {
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
                // Emits a warning if there are values when we expect none.
                // Using it in the trait allows us to adjust it if needed in the future.
                token.parse_flag(&values, token.clone())
            }

            CasteTag::AltTile { .. } => {
                token.parse_single(&values, |tile| CasteTag::AltTile { tile })
            }
            CasteTag::ApplyCreatureVariation { .. } => {
                token.parse_labeled_vector(&values, |id, args| CasteTag::ApplyCreatureVariation {
                    id,
                    args,
                })
            }
            CasteTag::Attack { .. } => token.parse_labeled_vector(&values, |verb, selector| {
                CasteTag::Attack { verb, selector }
            }),
            CasteTag::AttackTrigger { .. } => {
                token.parse_array(&values, |[population, exported_wealth, created_wealth]| {
                    CasteTag::AttackTrigger {
                        population,
                        exported_wealth,
                        created_wealth,
                    }
                })
            }
            CasteTag::Baby { .. } => token.parse_single(&values, |age| CasteTag::Baby { age }),
            CasteTag::BabyName { .. } => token.parse_array(&values, |[singular, plural]| {
                CasteTag::BabyName { singular, plural }
            }),
            CasteTag::BeachFrequency { .. } => {
                token.parse_single(&values, |frequency| CasteTag::BeachFrequency { frequency })
            }
            CasteTag::Blood { .. } => token.parse_vector_with_tail(&values, |material, state| {
                CasteTag::Blood { material, state }
            }),
            CasteTag::Body { .. } => {
                token.parse_vector(&values, |body_parts| CasteTag::Body { body_parts })
            }
            CasteTag::BodyAppearanceModifier { .. } => token
                .parse_labeled_array(&values, |attribute, values| {
                    CasteTag::BodyAppearanceModifier { attribute, values }
                }),
            CasteTag::BodyDetailPlan { .. } => {
                token.parse_labeled_vector(&values, |body_plan, arguments| {
                    CasteTag::BodyDetailPlan {
                        body_plan,
                        arguments,
                    }
                })
            }
            CasteTag::BodySize { .. } => token.parse_array(&values, |[year, days, size]| {
                CasteTag::BodySize { year, days, size }
            }),
            CasteTag::BodyGloss { .. } => {
                token.parse_single(&values, |gloss| CasteTag::BodyGloss { gloss })
            }
            CasteTag::BodyPartAddType { .. } => token.parse_single(&values, |body_part_type| {
                CasteTag::BodyPartAddType { body_part_type }
            }),
            CasteTag::BodyPartAppearanceModifier { .. } => token
                .parse_labeled_array(&values, |quality, spread| {
                    CasteTag::BodyPartAppearanceModifier { quality, spread }
                }),
            CasteTag::BodyPartRemoveType { .. } => token.parse_single(&values, |body_part_type| {
                CasteTag::BodyPartRemoveType { body_part_type }
            }),
            CasteTag::BuildingDestroyer { .. } => {
                token.parse_single(&values, |int_value: u32| CasteTag::BuildingDestroyer {
                    door_and_furniture_focused: int_value == 1,
                })
            }
            CasteTag::CanDoInteraction { .. } => token.parse_single(&values, |interaction| {
                CasteTag::CanDoInteraction { interaction }
            }),
            CasteTag::ChangeBodySizePercent { .. } => token.parse_single(&values, |percent| {
                CasteTag::ChangeBodySizePercent { percent }
            }),
            CasteTag::Child { .. } => token.parse_single(&values, |age| CasteTag::Child { age }),
            CasteTag::ChildName { .. } => token.parse_array(&values, |[singular, plural]| {
                CasteTag::ChildName { singular, plural }
            }),
            CasteTag::ClutchSize { .. } => {
                token.parse_array(&values, |[min, max]| CasteTag::ClutchSize { min, max })
            }
            CasteTag::Color { .. } => {
                token.parse_array(&values, |[foreground, background, brightness]| {
                    CasteTag::Color {
                        foreground,
                        background,
                        brightness,
                    }
                })
            }
            CasteTag::CreatureClass { .. } => {
                token.parse_single(&values, |class| CasteTag::CreatureClass { class })
            }
            CasteTag::CreatureVariationAddTag { .. } => {
                token.parse_single(&values, |tag| CasteTag::CreatureVariationAddTag { tag })
            }
            CasteTag::CreatureVariationRemoveTag { .. } => {
                token.parse_single(&values, |tag| CasteTag::CreatureVariationRemoveTag { tag })
            }
            CasteTag::Description { .. } => {
                token.parse_single(&values, |description| CasteTag::Description { description })
            }
            CasteTag::Difficulty { .. } => {
                token.parse_single(&values, |difficulty| CasteTag::Difficulty { difficulty })
            }
            CasteTag::ExtraButcherObjectItem { .. } => {
                token.parse_labeled_vector(&values, |item, material| {
                    CasteTag::ExtraButcherObjectItem { item, material }
                })
            }
            CasteTag::ExtraButcherObjectShape { .. } => {
                token.parse_single(&values, |shape| CasteTag::ExtraButcherObjectShape { shape })
            }
            CasteTag::EggMaterial { .. } => {
                token.parse_vector_with_tail(&values, |material, state| CasteTag::EggMaterial {
                    material,
                    state,
                })
            }
            CasteTag::EggSize { .. } => {
                token.parse_single(&values, |size| CasteTag::EggSize { size })
            }
            CasteTag::ExtraButcherObject { .. } => {
                token.parse_labeled_vector(&values, |object_type, arguments| {
                    CasteTag::ExtraButcherObject {
                        object_type,
                        arguments,
                    }
                })
            }
            CasteTag::Extract { .. } => {
                token.parse_single(&values, |material| CasteTag::Extract { material })
            }
            CasteTag::FixedTemp { .. } => {
                token.parse_single(&values, |temperature| CasteTag::FixedTemp { temperature })
            }
            CasteTag::Gait { .. } => {
                token.parse_vector(&values, |gait_values| CasteTag::Gait { gait_values })
            }
            CasteTag::GeneralMaterialForceMultiplier { .. } => token
                .parse_array(&values, |[value_a, value_b]| {
                    CasteTag::GeneralMaterialForceMultiplier { value_a, value_b }
                }),
            CasteTag::GlowColor { .. } => {
                token.parse_array(&values, |[foreground, background, brightness]| {
                    CasteTag::GlowColor {
                        foreground,
                        background,
                        brightness,
                    }
                })
            }
            CasteTag::GlowTile { .. } => {
                token.parse_single(&values, |tile| CasteTag::GlowTile { tile })
            }
            CasteTag::Gnawer { .. } => {
                token.parse_single(&values, |verb| CasteTag::Gnawer { verb })
            }
            CasteTag::GobbleVerminClass { .. } => token.parse_single(&values, |vermin_class| {
                CasteTag::GobbleVerminClass { vermin_class }
            }),
            CasteTag::GobbleVerminCreature { .. } => {
                token.parse_array(&values, |[vermin_creature, vermin_caste]| {
                    CasteTag::GobbleVerminCreature {
                        vermin_creature,
                        vermin_caste,
                    }
                })
            }
            CasteTag::GrassTrample { .. } => {
                token.parse_single(&values, |trample| CasteTag::GrassTrample { trample })
            }
            CasteTag::GravitateBodySize { .. } => {
                token.parse_single(&values, |target| CasteTag::GravitateBodySize { target })
            }
            CasteTag::Grazer { .. } => {
                token.parse_single(&values, |grazer| CasteTag::Grazer { grazer })
            }
            CasteTag::Habit { .. } => {
                token.parse_single(&values, |habit| CasteTag::Habit { habit })
            }
            CasteTag::HabitNumber { .. } => {
                token.parse_single(&values, |number| CasteTag::HabitNumber { number })
            }
            CasteTag::Homeotherm { .. } => {
                token.parse_single(&values, |temperature| CasteTag::Homeotherm { temperature })
            }
            CasteTag::InteractionDetail { .. } => {
                token.parse_labeled_vector(&values, |label, args| CasteTag::InteractionDetail {
                    label,
                    args,
                })
            }
            CasteTag::ItemCorpse { .. } => token.parse_labeled_vector(&values, |item, material| {
                CasteTag::ItemCorpse { item, material }
            }),
            CasteTag::ItemCorpseQuality { .. } => {
                token.parse_single(&values, |quality| CasteTag::ItemCorpseQuality { quality })
            }
            CasteTag::Lair { .. } => token.parse_labeled_array(&values, |lair, [probability]| {
                CasteTag::Lair { lair, probability }
            }),
            CasteTag::LairCharacteristic { .. } => token.parse_single(&values, |characteristic| {
                CasteTag::LairCharacteristic { characteristic }
            }),
            CasteTag::LairHunterSpeech { .. } => token.parse_single(&values, |speech_file| {
                CasteTag::LairHunterSpeech { speech_file }
            }),

            CasteTag::LaysUnusualEggs { .. } => {
                token.parse_labeled_vector(&values, |item, material| CasteTag::LaysUnusualEggs {
                    item,
                    material,
                })
            }
            CasteTag::Ligaments { .. } => {
                token.parse_vector_with_tail(&values, |material, healing_rate| {
                    CasteTag::Ligaments {
                        material,
                        healing_rate,
                    }
                })
            }
            CasteTag::LitterSize { .. } => {
                token.parse_array(&values, |[min, max]| CasteTag::LitterSize { min, max })
            }
            CasteTag::LowLightVision { .. } => {
                token.parse_single(&values, |vision| CasteTag::LowLightVision { vision })
            }
            CasteTag::MannerismFingers { .. } => token.parse_array(&values, |[finger, fingers]| {
                CasteTag::MannerismFingers { finger, fingers }
            }),
            CasteTag::MannerismNose { .. } => {
                token.parse_single(&values, |nose| CasteTag::MannerismNose { nose })
            }
            CasteTag::MannerismEar { .. } => {
                token.parse_single(&values, |ear| CasteTag::MannerismEar { ear })
            }
            CasteTag::MannerismHead { .. } => {
                token.parse_single(&values, |head| CasteTag::MannerismHead { head })
            }
            CasteTag::MannerismEyes { .. } => {
                token.parse_single(&values, |eyes| CasteTag::MannerismEyes { eyes })
            }
            CasteTag::MannerismMouth { .. } => {
                token.parse_single(&values, |mouth| CasteTag::MannerismMouth { mouth })
            }
            CasteTag::MannerismHair { .. } => {
                token.parse_single(&values, |hair| CasteTag::MannerismHair { hair })
            }
            CasteTag::MannerismKnuckles { .. } => {
                token.parse_single(&values, |knuckles| CasteTag::MannerismKnuckles { knuckles })
            }
            CasteTag::MannerismLips { .. } => {
                token.parse_single(&values, |lips| CasteTag::MannerismLips { lips })
            }
            CasteTag::MannerismCheek { .. } => {
                token.parse_single(&values, |cheek| CasteTag::MannerismCheek { cheek })
            }
            CasteTag::MannerismNails { .. } => {
                token.parse_single(&values, |nails| CasteTag::MannerismNails { nails })
            }
            CasteTag::MannerismFeet { .. } => {
                token.parse_single(&values, |feet| CasteTag::MannerismFeet { feet })
            }
            CasteTag::MannerismArms { .. } => {
                token.parse_single(&values, |arms| CasteTag::MannerismArms { arms })
            }
            CasteTag::MannerismHands { .. } => {
                token.parse_single(&values, |hands| CasteTag::MannerismHands { hands })
            }
            CasteTag::MannerismTongue { .. } => {
                token.parse_single(&values, |tongue| CasteTag::MannerismTongue { tongue })
            }
            CasteTag::MannerismLeg { .. } => {
                token.parse_single(&values, |leg| CasteTag::MannerismLeg { leg })
            }
            CasteTag::MaxAge { .. } => {
                token.parse_array(&values, |[min, max]| CasteTag::MaxAge { min, max })
            }
            CasteTag::MentalAttributeCapPercentage { .. } => {
                token.parse_labeled_array(&values, |attribute, [percentage]| {
                    CasteTag::MentalAttributeCapPercentage {
                        attribute,
                        percentage,
                    }
                })
            }
            CasteTag::MentalAttributeRange { .. } => token
                .parse_labeled_array(&values, |attribute, ranges| {
                    CasteTag::MentalAttributeRange { attribute, ranges }
                }),
            CasteTag::MentalAttributeRate { .. } => token.parse_labeled_array(
                &values,
                |attribute,
                 [
                    improvement_cost,
                    decay_rate_unused,
                    decay_rate_rusty,
                    decay_rate_demotion,
                ]| CasteTag::MentalAttributeRate {
                    attribute,
                    improvement_cost,
                    decay_rate_unused,
                    decay_rate_rusty,
                    decay_rate_demotion,
                },
            ),
            CasteTag::Milkable { .. } => {
                token.parse_vector_with_tail(&values, |material, frequency| CasteTag::Milkable {
                    material,
                    frequency,
                })
            }
            CasteTag::ModValue { .. } => {
                token.parse_single(&values, |value| CasteTag::ModValue { value })
            }
            CasteTag::Name { .. } => {
                token.parse_array(&values, |[singular, plural, adjective]| CasteTag::Name {
                    singular,
                    plural,
                    adjective,
                })
            }
            CasteTag::NaturalSkill { .. } => {
                token.parse_labeled_array(&values, |skill, [level]| CasteTag::NaturalSkill {
                    skill,
                    level,
                })
            }
            CasteTag::OdorLevel { .. } => {
                token.parse_single(&values, |odor_level| CasteTag::OdorLevel { odor_level })
            }
            CasteTag::OdorString { .. } => {
                token.parse_single(&values, |odor_string| CasteTag::OdorString { odor_string })
            }
            CasteTag::Orientation { .. } => token.parse_labeled_array(
                &values,
                |caste, [disinterested_chance, casual_chance, strong_chance]| {
                    CasteTag::Orientation {
                        caste,
                        disinterested_chance,
                        casual_chance,
                        strong_chance,
                    }
                },
            ),
            CasteTag::PenetratePower { .. } => token.parse_single(&values, |penetrate_power| {
                CasteTag::PenetratePower { penetrate_power }
            }),
            CasteTag::Personality { .. } => {
                token.parse_labeled_array(&values, |personality_trait, [low, median, high]| {
                    CasteTag::Personality {
                        personality_trait,
                        low,
                        median,
                        high,
                    }
                })
            }
            CasteTag::PetValue { .. } => {
                token.parse_single(&values, |pet_value| CasteTag::PetValue { pet_value })
            }
            CasteTag::PetValueDivisor { .. } => {
                token.parse_single(&values, |divisor| CasteTag::PetValueDivisor { divisor })
            }
            CasteTag::PhysicalAttributeCapPercentage { .. } => {
                token.parse_labeled_array(&values, |attribute, [percentage]| {
                    CasteTag::PhysicalAttributeCapPercentage {
                        attribute,
                        percentage,
                    }
                })
            }
            CasteTag::PhysicalAttributeRange { .. } => token
                .parse_labeled_array(&values, |attribute, ranges| {
                    CasteTag::PhysicalAttributeRange { attribute, ranges }
                }),
            CasteTag::PhysicalAttributeRate { .. } => token.parse_labeled_array(
                &values,
                |attribute,
                 [
                    improvement_cost,
                    decay_rate_unused,
                    decay_rate_rusty,
                    decay_rate_demotion,
                ]| Self::PhysicalAttributeRate {
                    attribute,
                    improvement_cost,
                    decay_rate_unused,
                    decay_rate_rusty,
                    decay_rate_demotion,
                },
            ),
            CasteTag::PlusBodyPartGroup { .. } => {
                token.parse_vector(&values, |selector| CasteTag::PlusBodyPartGroup { selector })
            }
            CasteTag::PopulationRatio { .. } => {
                token.parse_single(&values, |pop_ratio| CasteTag::PopulationRatio { pop_ratio })
            }
            CasteTag::ProfessionName { .. } => {
                token.parse_array(&values, |[profession, singular, plural]| {
                    CasteTag::ProfessionName {
                        profession,
                        singular,
                        plural,
                    }
                })
            }
            CasteTag::ProneToRage { .. } => {
                token.parse_single(&values, |rage_chance| CasteTag::ProneToRage { rage_chance })
            }
            CasteTag::Pus { .. } => token.parse_vector_with_tail(&values, |material, state| {
                CasteTag::Pus { material, state }
            }),
            CasteTag::RelativeSize { .. } => {
                token.parse_vector_with_tail(&values, |selector, relative_size| {
                    CasteTag::RelativeSize {
                        selector,
                        relative_size,
                    }
                })
            }
            CasteTag::Remains { .. } => token.parse_array(&values, |[singular, plural]| {
                CasteTag::Remains { singular, plural }
            }),
            CasteTag::RemainsColor { .. } => token.parse_single(&values, |remains_color| {
                CasteTag::RemainsColor { remains_color }
            }),
            CasteTag::RetractIntoBodyPart { .. } => token.parse_array(
                &values,
                |[
                    body_part_selector,
                    body_part,
                    second_person,
                    third_person,
                    second_person_cancel,
                    third_person_cancel,
                ]| CasteTag::RetractIntoBodyPart {
                    body_part_selector,
                    body_part,
                    second_person,
                    third_person,
                    second_person_cancel,
                    third_person_cancel,
                },
            ),
            CasteTag::RootAround { .. } => token.parse_array(
                &values,
                |[
                    body_part,
                    body_part_selection,
                    second_person_verb,
                    third_person_verb,
                ]| CasteTag::RootAround {
                    body_part_selector: vec![body_part, body_part_selection],
                    second_person_verb,
                    third_person_verb,
                },
            ),
            CasteTag::Secretion { .. } => token
                .parse_vector(&values, |mut args| {
                    // Safety check: Material(1+) + State(1) + Selector(2) + Tissue(1) + Trigger(1) = 6 min
                    if args.len() < 6 {
                        tracing::warn!(
                            "Secretion tag missing required arguments: {}/6 {:?}",
                            args.len(),
                            args
                        );
                        return None;
                    }

                    // 1. Pop the fixed tail arguments (Order is LIFO: Last In, First Out)
                    let trigger = args.pop().unwrap_or_default(); // EXERTION
                    let tissue_layer = args.pop().unwrap_or_default(); // SKIN

                    // We assume the selector is always 2 tokens (e.g. BY_CATEGORY:ALL)
                    let bp_target = args.pop().unwrap_or_default(); // ALL
                    let bp_selector = args.pop().unwrap_or_default(); // BY_CATEGORY
                    let body_part_selector = vec![bp_selector, bp_target];

                    let material_state = args.pop().unwrap_or_default(); // LIQUID

                    // 2. Whatever is left is the Material (e.g. ["LOCAL_CREATURE_MAT", "SWEAT"])
                    // We join it back into a string to match your struct definition
                    let material = args;

                    Some(CasteTag::Secretion {
                        material,
                        material_state,
                        body_part_selector,
                        tissue_layer,
                        trigger,
                    })
                })
                .flatten(),
            CasteTag::SenseCreatureClass { .. } => token.parse_array(
                &values,
                |[creature_class, tile, foreground, background, brightness]| {
                    CasteTag::SenseCreatureClass {
                        creature_class,
                        tile,
                        foreground: foreground.parse::<u32>().unwrap_or_default(),
                        background: background.parse::<u32>().unwrap_or_default(),
                        brightness: brightness.parse::<u32>().unwrap_or_default(),
                    }
                },
            ),
            CasteTag::SetBodyPartGroup { .. } => token
                .parse_vector(&values, |body_part_selector| CasteTag::SetBodyPartGroup {
                    body_part_selector,
                }),
            CasteTag::SkillLearnRate { .. } => {
                token.parse_labeled_array(&values, |skill, [rate]| CasteTag::SkillLearnRate {
                    skill,
                    rate,
                })
            }
            CasteTag::SkillLearnRates { .. } => {
                token.parse_single(&values, |rate| CasteTag::SkillLearnRates { rate })
            }
            CasteTag::SkillRate { .. } => token.parse_labeled_array(
                &values,
                |skill,
                 [
                    improvement_rate,
                    decay_rate_unused,
                    decay_rate_rusty,
                    decay_rate_demotion,
                ]| CasteTag::SkillRate {
                    skill,
                    improvement_rate,
                    decay_rate_unused,
                    decay_rate_rusty,
                    decay_rate_demotion,
                },
            ),
            CasteTag::SkillRates { .. } => token.parse_array(
                &values,
                |[
                    improvement_rate,
                    decay_rate_unused,
                    decay_rate_rusty,
                    decay_rate_demotion,
                ]| CasteTag::SkillRates {
                    improvement_rate,
                    decay_rate_unused,
                    decay_rate_rusty,
                    decay_rate_demotion,
                },
            ),
            CasteTag::SkillRustRate { .. } => token.parse_labeled_array(
                &values,
                |skill, [decay_rate_unused, decay_rate_rusty, decay_rate_demotion]| {
                    CasteTag::SkillRustRate {
                        skill,
                        decay_rate_unused,
                        decay_rate_rusty,
                        decay_rate_demotion,
                    }
                },
            ),
            CasteTag::SkillRustRates { .. } => token.parse_array(
                &values,
                |[decay_rate_unused, decay_rate_rusty, decay_rate_demotion]| {
                    CasteTag::SkillRustRates {
                        decay_rate_unused,
                        decay_rate_rusty,
                        decay_rate_demotion,
                    }
                },
            ),
            CasteTag::SlainSpeech { .. } => {
                token.parse_single(&values, |speech_file| CasteTag::SlainSpeech { speech_file })
            }
            CasteTag::SoldierTile { .. } => {
                token.parse_single(&values, |tile| CasteTag::SoldierTile { tile })
            }
            CasteTag::SoldierAltTile { .. } => {
                token.parse_single(&values, |tile| CasteTag::SoldierAltTile { tile })
            }
            CasteTag::Sound { .. } => {
                // Check if there are enough arguments to parse
                if values.len() < 6 {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse sound: not enough arguments: {}/6 '{:?}'",
                        values.len(),
                        values
                    );
                    return None;
                }
                let requires_breathing = values.len() == 7;

                let sound_type = (values.first().unwrap_or(&"")).to_string();
                let Ok(sound_range) = (values.get(1).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse sound: sound range: {values:?}"
                    );
                    return None;
                };
                let Ok(sound_interval) = (values.get(2).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse sound: sound interval: {values:?}"
                    );
                    return None;
                };
                let breathing_bump = usize::from(requires_breathing);

                let third_person = (values.get(3 + breathing_bump).unwrap_or(&"")).to_string();
                let first_person = (values.get(4 + breathing_bump).unwrap_or(&"")).to_string();
                let out_of_sight = (values.get(5 + breathing_bump).unwrap_or(&"")).to_string();
                Some(Self::Sound {
                    sound_type,
                    sound_range,
                    sound_interval,
                    requires_breathing,
                    third_person,
                    first_person,
                    out_of_sight,
                })
            }
            CasteTag::SpecificFood { .. } => {
                if values.len() < 2 {
                    tracing::warn!(
                        "Not enough args for SpecificFood: {}/2: {:?}",
                        &values.len(),
                        &values
                    );
                    return None;
                }

                let object_type_key = values[0];
                let identifier = values[1].to_string();

                // Validated lookup
                let food_type = OBJECT_TOKEN_MAP.get(object_type_key).or_else(|| {
                    tracing::warn!("SpecificFood: Unknown object type: {}", object_type_key);
                    None
                })?;

                Some(CasteTag::SpecificFood {
                    food_type: *food_type,
                    identifier,
                })
            }
            CasteTag::SyndromeDilutionFactor { .. } => {
                token.parse_labeled_array(&values, |syndrome, [percentage]| {
                    CasteTag::SyndromeDilutionFactor {
                        syndrome,
                        percentage,
                    }
                })
            }
            CasteTag::Tendons { .. } => {
                token.parse_vector_with_tail(&values, |material, healing_rate| CasteTag::Tendons {
                    material,
                    healing_rate,
                })
            }
            CasteTag::Tile { .. } => token.parse_single(&values, |tile| CasteTag::Tile { tile }),
            CasteTag::TissueLayer { .. } => {
                token.parse_vector(&values, |mut args: Vec<String>| {
                    // We need at least 3 arguments (Selector, Part, Tissue)
                    // If less than 3, we pad with empty strings to avoid panicing
                    // (If we knew default values, we could provide them here.)
                    let tissue = if args.len() >= 3 {
                        args.remove(2)
                    } else {
                        String::new()
                    };
                    let body_part = if args.len() >= 2 {
                        args.remove(1)
                    } else {
                        String::new()
                    };
                    let body_part_selection = if !args.is_empty() {
                        args.remove(0)
                    } else {
                        String::new()
                    };

                    // Whatever is left in `args` is the positioning
                    let positioning = args;

                    CasteTag::TissueLayer {
                        body_part_selector: vec![body_part, body_part_selection],
                        tissue,
                        positioning,
                    }
                })
            }
            CasteTag::TissueLayerUnder { .. } => {
                token.parse_array(&values, |[body_part_selector, body_part, tissue]| {
                    CasteTag::TissueLayerUnder {
                        body_part_selector,
                        body_part,
                        tissue,
                    }
                })
            }
            CasteTag::TradeCapacity { .. } => {
                token.parse_single(&values, |capacity| CasteTag::TradeCapacity { capacity })
            }
            CasteTag::VerminBite { .. } => {
                token.parse_vector_with_tail(&values, |body: Vec<String>, state| {
                    let mut iter = body.into_iter();

                    // 1. Pop 'chance' (use token.parse_value to handle "NONE" -> 0 safely)
                    let chance = iter.next().and_then(|s| token.parse_value(&s)).unwrap_or(0);

                    // 2. Pop 'verb'
                    let verb = iter.next().unwrap_or_default();

                    // 3. Collect the rest as 'material'
                    let material: Vec<String> = iter.collect();

                    CasteTag::VerminBite {
                        chance,
                        verb,
                        material,
                        material_state: state,
                    }
                })
            }
            CasteTag::ViewRange { .. } => {
                token.parse_single(&values, |view_range| CasteTag::ViewRange { view_range })
            }
            CasteTag::VisionArc { .. } => {
                token.parse_array(&values, |[binocular, non_binocular]| CasteTag::VisionArc {
                    binocular,
                    non_binocular,
                })
            }
            CasteTag::Webber { .. } => {
                token.parse_vector(&values, |material| CasteTag::Webber { material })
            }
        }
    }
}
