//! The parsing implementation for `CasteTag`

use crate::{
    raw_definitions::{CASTE_TOKENS, OBJECT_TOKEN_MAP},
    tokens::CasteToken,
    traits::{TagOperations, TokenParser},
};

impl TagOperations for CasteToken {
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
            | CasteToken::CannotBreatheAir => {
                // Emits a warning if there are values when we expect none.
                // Using it in the trait allows us to adjust it if needed in the future.
                token.parse_flag(&values, token.clone())
            }

            CasteToken::AltTile { .. } => {
                token.parse_single(&values, |tile| CasteToken::AltTile { tile })
            }
            CasteToken::ApplyCreatureVariation { .. } => {
                token.parse_labeled_vector(&values, |id, args| CasteToken::ApplyCreatureVariation {
                    id,
                    args,
                })
            }
            CasteToken::Attack { .. } => token.parse_labeled_vector(&values, |verb, selector| {
                CasteToken::Attack { verb, selector }
            }),
            CasteToken::AttackTrigger { .. } => {
                token.parse_array(&values, |[population, exported_wealth, created_wealth]| {
                    CasteToken::AttackTrigger {
                        population,
                        exported_wealth,
                        created_wealth,
                    }
                })
            }
            CasteToken::Baby { .. } => token.parse_single(&values, |age| CasteToken::Baby { age }),
            CasteToken::BabyName { .. } => token.parse_array(&values, |[singular, plural]| {
                CasteToken::BabyName { singular, plural }
            }),
            CasteToken::BeachFrequency { .. } => {
                token.parse_single(&values, |frequency| CasteToken::BeachFrequency { frequency })
            }
            CasteToken::Blood { .. } => token.parse_vector_with_tail(&values, |material, state| {
                CasteToken::Blood { material, state }
            }),
            CasteToken::Body { .. } => {
                token.parse_vector(&values, |body_parts| CasteToken::Body { body_parts })
            }
            CasteToken::BodyAppearanceModifier { .. } => token
                .parse_labeled_array(&values, |attribute, values| {
                    CasteToken::BodyAppearanceModifier { attribute, values }
                }),
            CasteToken::BodyDetailPlan { .. } => {
                token.parse_labeled_vector(&values, |body_plan, arguments| {
                    CasteToken::BodyDetailPlan {
                        body_plan,
                        arguments,
                    }
                })
            }
            CasteToken::BodySize { .. } => token.parse_array(&values, |[year, days, size]| {
                CasteToken::BodySize { year, days, size }
            }),
            CasteToken::BodyGloss { .. } => {
                token.parse_single(&values, |gloss| CasteToken::BodyGloss { gloss })
            }
            CasteToken::BodyPartAddType { .. } => token.parse_single(&values, |body_part_type| {
                CasteToken::BodyPartAddType { body_part_type }
            }),
            CasteToken::BodyPartAppearanceModifier { .. } => token
                .parse_labeled_array(&values, |quality, spread| {
                    CasteToken::BodyPartAppearanceModifier { quality, spread }
                }),
            CasteToken::BodyPartRemoveType { .. } => token.parse_single(&values, |body_part_type| {
                CasteToken::BodyPartRemoveType { body_part_type }
            }),
            CasteToken::BuildingDestroyer { .. } => {
                token.parse_single(&values, |int_value: u32| CasteToken::BuildingDestroyer {
                    door_and_furniture_focused: int_value == 1,
                })
            }
            CasteToken::CanDoInteraction { .. } => token.parse_single(&values, |interaction| {
                CasteToken::CanDoInteraction { interaction }
            }),
            CasteToken::ChangeBodySizePercent { .. } => token.parse_single(&values, |percent| {
                CasteToken::ChangeBodySizePercent { percent }
            }),
            CasteToken::Child { .. } => token.parse_single(&values, |age| CasteToken::Child { age }),
            CasteToken::ChildName { .. } => token.parse_array(&values, |[singular, plural]| {
                CasteToken::ChildName { singular, plural }
            }),
            CasteToken::ClutchSize { .. } => {
                token.parse_array(&values, |[min, max]| CasteToken::ClutchSize { min, max })
            }
            CasteToken::Color { .. } => {
                token.parse_array(&values, |[foreground, background, brightness]| {
                    CasteToken::Color {
                        foreground,
                        background,
                        brightness,
                    }
                })
            }
            CasteToken::CreatureClass { .. } => {
                token.parse_single(&values, |class| CasteToken::CreatureClass { class })
            }
            CasteToken::CreatureVariationAddTag { .. } => {
                token.parse_single(&values, |tag| CasteToken::CreatureVariationAddTag { tag })
            }
            CasteToken::CreatureVariationRemoveTag { .. } => {
                token.parse_single(&values, |tag| CasteToken::CreatureVariationRemoveTag { tag })
            }
            CasteToken::Description { .. } => {
                token.parse_single(&values, |description| CasteToken::Description { description })
            }
            CasteToken::Difficulty { .. } => {
                token.parse_single(&values, |difficulty| CasteToken::Difficulty { difficulty })
            }
            CasteToken::ExtraButcherObjectItem { .. } => {
                token.parse_labeled_vector(&values, |item, material| {
                    CasteToken::ExtraButcherObjectItem { item, material }
                })
            }
            CasteToken::ExtraButcherObjectShape { .. } => {
                token.parse_single(&values, |shape| CasteToken::ExtraButcherObjectShape { shape })
            }
            CasteToken::EggMaterial { .. } => {
                token.parse_vector_with_tail(&values, |material, state| CasteToken::EggMaterial {
                    material,
                    state,
                })
            }
            CasteToken::EggSize { .. } => {
                token.parse_single(&values, |size| CasteToken::EggSize { size })
            }
            CasteToken::ExtraButcherObject { .. } => {
                token.parse_labeled_vector(&values, |object_type, arguments| {
                    CasteToken::ExtraButcherObject {
                        object_type,
                        arguments,
                    }
                })
            }
            CasteToken::Extract { .. } => {
                token.parse_single(&values, |material| CasteToken::Extract { material })
            }
            CasteToken::FixedTemp { .. } => {
                token.parse_single(&values, |temperature| CasteToken::FixedTemp { temperature })
            }
            CasteToken::Gait { .. } => {
                token.parse_vector(&values, |gait_values| CasteToken::Gait { gait_values })
            }
            CasteToken::GeneralMaterialForceMultiplier { .. } => token
                .parse_array(&values, |[value_a, value_b]| {
                    CasteToken::GeneralMaterialForceMultiplier { value_a, value_b }
                }),
            CasteToken::GlowColor { .. } => {
                token.parse_array(&values, |[foreground, background, brightness]| {
                    CasteToken::GlowColor {
                        foreground,
                        background,
                        brightness,
                    }
                })
            }
            CasteToken::GlowTile { .. } => {
                token.parse_single(&values, |tile| CasteToken::GlowTile { tile })
            }
            CasteToken::Gnawer { .. } => {
                token.parse_single(&values, |verb| CasteToken::Gnawer { verb })
            }
            CasteToken::GobbleVerminClass { .. } => token.parse_single(&values, |vermin_class| {
                CasteToken::GobbleVerminClass { vermin_class }
            }),
            CasteToken::GobbleVerminCreature { .. } => {
                token.parse_array(&values, |[vermin_creature, vermin_caste]| {
                    CasteToken::GobbleVerminCreature {
                        vermin_creature,
                        vermin_caste,
                    }
                })
            }
            CasteToken::GrassTrample { .. } => {
                token.parse_single(&values, |trample| CasteToken::GrassTrample { trample })
            }
            CasteToken::GravitateBodySize { .. } => {
                token.parse_single(&values, |target| CasteToken::GravitateBodySize { target })
            }
            CasteToken::Grazer { .. } => {
                token.parse_single(&values, |grazer| CasteToken::Grazer { grazer })
            }
            CasteToken::Habit { .. } => {
                token.parse_single(&values, |habit| CasteToken::Habit { habit })
            }
            CasteToken::HabitNumber { .. } => {
                token.parse_single(&values, |number| CasteToken::HabitNumber { number })
            }
            CasteToken::Homeotherm { .. } => {
                token.parse_single(&values, |temperature| CasteToken::Homeotherm { temperature })
            }
            CasteToken::InteractionDetail { .. } => {
                token.parse_labeled_vector(&values, |label, args| CasteToken::InteractionDetail {
                    label,
                    args,
                })
            }
            CasteToken::ItemCorpse { .. } => token.parse_labeled_vector(&values, |item, material| {
                CasteToken::ItemCorpse { item, material }
            }),
            CasteToken::ItemCorpseQuality { .. } => {
                token.parse_single(&values, |quality| CasteToken::ItemCorpseQuality { quality })
            }
            CasteToken::Lair { .. } => token.parse_labeled_array(&values, |lair, [probability]| {
                CasteToken::Lair { lair, probability }
            }),
            CasteToken::LairCharacteristic { .. } => token.parse_single(&values, |characteristic| {
                CasteToken::LairCharacteristic { characteristic }
            }),
            CasteToken::LairHunterSpeech { .. } => token.parse_single(&values, |speech_file| {
                CasteToken::LairHunterSpeech { speech_file }
            }),

            CasteToken::LaysUnusualEggs { .. } => {
                token.parse_labeled_vector(&values, |item, material| CasteToken::LaysUnusualEggs {
                    item,
                    material,
                })
            }
            CasteToken::Ligaments { .. } => {
                token.parse_vector_with_tail(&values, |material, healing_rate| {
                    CasteToken::Ligaments {
                        material,
                        healing_rate,
                    }
                })
            }
            CasteToken::LitterSize { .. } => {
                token.parse_array(&values, |[min, max]| CasteToken::LitterSize { min, max })
            }
            CasteToken::LowLightVision { .. } => {
                token.parse_single(&values, |vision| CasteToken::LowLightVision { vision })
            }
            CasteToken::MannerismFingers { .. } => token.parse_array(&values, |[finger, fingers]| {
                CasteToken::MannerismFingers { finger, fingers }
            }),
            CasteToken::MannerismNose { .. } => {
                token.parse_single(&values, |nose| CasteToken::MannerismNose { nose })
            }
            CasteToken::MannerismEar { .. } => {
                token.parse_single(&values, |ear| CasteToken::MannerismEar { ear })
            }
            CasteToken::MannerismHead { .. } => {
                token.parse_single(&values, |head| CasteToken::MannerismHead { head })
            }
            CasteToken::MannerismEyes { .. } => {
                token.parse_single(&values, |eyes| CasteToken::MannerismEyes { eyes })
            }
            CasteToken::MannerismMouth { .. } => {
                token.parse_single(&values, |mouth| CasteToken::MannerismMouth { mouth })
            }
            CasteToken::MannerismHair { .. } => {
                token.parse_single(&values, |hair| CasteToken::MannerismHair { hair })
            }
            CasteToken::MannerismKnuckles { .. } => {
                token.parse_single(&values, |knuckles| CasteToken::MannerismKnuckles { knuckles })
            }
            CasteToken::MannerismLips { .. } => {
                token.parse_single(&values, |lips| CasteToken::MannerismLips { lips })
            }
            CasteToken::MannerismCheek { .. } => {
                token.parse_single(&values, |cheek| CasteToken::MannerismCheek { cheek })
            }
            CasteToken::MannerismNails { .. } => {
                token.parse_single(&values, |nails| CasteToken::MannerismNails { nails })
            }
            CasteToken::MannerismFeet { .. } => {
                token.parse_single(&values, |feet| CasteToken::MannerismFeet { feet })
            }
            CasteToken::MannerismArms { .. } => {
                token.parse_single(&values, |arms| CasteToken::MannerismArms { arms })
            }
            CasteToken::MannerismHands { .. } => {
                token.parse_single(&values, |hands| CasteToken::MannerismHands { hands })
            }
            CasteToken::MannerismTongue { .. } => {
                token.parse_single(&values, |tongue| CasteToken::MannerismTongue { tongue })
            }
            CasteToken::MannerismLeg { .. } => {
                token.parse_single(&values, |leg| CasteToken::MannerismLeg { leg })
            }
            CasteToken::MaxAge { .. } => {
                token.parse_array(&values, |[min, max]| CasteToken::MaxAge { min, max })
            }
            CasteToken::MentalAttributeCapPercentage { .. } => {
                token.parse_labeled_array(&values, |attribute, [percentage]| {
                    CasteToken::MentalAttributeCapPercentage {
                        attribute,
                        percentage,
                    }
                })
            }
            CasteToken::MentalAttributeRange { .. } => token
                .parse_labeled_array(&values, |attribute, ranges| {
                    CasteToken::MentalAttributeRange { attribute, ranges }
                }),
            CasteToken::MentalAttributeRate { .. } => token.parse_labeled_array(
                &values,
                |attribute,
                 [
                    improvement_cost,
                    decay_rate_unused,
                    decay_rate_rusty,
                    decay_rate_demotion,
                ]| CasteToken::MentalAttributeRate {
                    attribute,
                    improvement_cost,
                    decay_rate_unused,
                    decay_rate_rusty,
                    decay_rate_demotion,
                },
            ),
            CasteToken::Milkable { .. } => {
                token.parse_vector_with_tail(&values, |material, frequency| CasteToken::Milkable {
                    material,
                    frequency,
                })
            }
            CasteToken::ModValue { .. } => {
                token.parse_single(&values, |value| CasteToken::ModValue { value })
            }
            CasteToken::Name { .. } => {
                token.parse_array(&values, |[singular, plural, adjective]| CasteToken::Name {
                    singular,
                    plural,
                    adjective,
                })
            }
            CasteToken::NaturalSkill { .. } => {
                token.parse_labeled_array(&values, |skill, [level]| CasteToken::NaturalSkill {
                    skill,
                    level,
                })
            }
            CasteToken::OdorLevel { .. } => {
                token.parse_single(&values, |odor_level| CasteToken::OdorLevel { odor_level })
            }
            CasteToken::OdorString { .. } => {
                token.parse_single(&values, |odor_string| CasteToken::OdorString { odor_string })
            }
            CasteToken::Orientation { .. } => token.parse_labeled_array(
                &values,
                |caste, [disinterested_chance, casual_chance, strong_chance]| {
                    CasteToken::Orientation {
                        caste,
                        disinterested_chance,
                        casual_chance,
                        strong_chance,
                    }
                },
            ),
            CasteToken::PenetratePower { .. } => token.parse_single(&values, |penetrate_power| {
                CasteToken::PenetratePower { penetrate_power }
            }),
            CasteToken::Personality { .. } => {
                token.parse_labeled_array(&values, |personality_trait, [low, median, high]| {
                    CasteToken::Personality {
                        personality_trait,
                        low,
                        median,
                        high,
                    }
                })
            }
            CasteToken::PetValue { .. } => {
                token.parse_single(&values, |pet_value| CasteToken::PetValue { pet_value })
            }
            CasteToken::PetValueDivisor { .. } => {
                token.parse_single(&values, |divisor| CasteToken::PetValueDivisor { divisor })
            }
            CasteToken::PhysicalAttributeCapPercentage { .. } => {
                token.parse_labeled_array(&values, |attribute, [percentage]| {
                    CasteToken::PhysicalAttributeCapPercentage {
                        attribute,
                        percentage,
                    }
                })
            }
            CasteToken::PhysicalAttributeRange { .. } => token
                .parse_labeled_array(&values, |attribute, ranges| {
                    CasteToken::PhysicalAttributeRange { attribute, ranges }
                }),
            CasteToken::PhysicalAttributeRate { .. } => token.parse_labeled_array(
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
            CasteToken::PlusBodyPartGroup { .. } => {
                token.parse_vector(&values, |selector| CasteToken::PlusBodyPartGroup { selector })
            }
            CasteToken::PopulationRatio { .. } => {
                token.parse_single(&values, |pop_ratio| CasteToken::PopulationRatio { pop_ratio })
            }
            CasteToken::ProfessionName { .. } => {
                token.parse_array(&values, |[profession, singular, plural]| {
                    CasteToken::ProfessionName {
                        profession,
                        singular,
                        plural,
                    }
                })
            }
            CasteToken::ProneToRage { .. } => {
                token.parse_single(&values, |rage_chance| CasteToken::ProneToRage { rage_chance })
            }
            CasteToken::Pus { .. } => token.parse_vector_with_tail(&values, |material, state| {
                CasteToken::Pus { material, state }
            }),
            CasteToken::RelativeSize { .. } => {
                token.parse_vector_with_tail(&values, |selector, relative_size| {
                    CasteToken::RelativeSize {
                        selector,
                        relative_size,
                    }
                })
            }
            CasteToken::Remains { .. } => token.parse_array(&values, |[singular, plural]| {
                CasteToken::Remains { singular, plural }
            }),
            CasteToken::RemainsColor { .. } => token.parse_single(&values, |remains_color| {
                CasteToken::RemainsColor { remains_color }
            }),
            CasteToken::RetractIntoBodyPart { .. } => token.parse_array(
                &values,
                |[
                    body_part_selector,
                    body_part,
                    second_person,
                    third_person,
                    second_person_cancel,
                    third_person_cancel,
                ]| CasteToken::RetractIntoBodyPart {
                    body_part_selector,
                    body_part,
                    second_person,
                    third_person,
                    second_person_cancel,
                    third_person_cancel,
                },
            ),
            CasteToken::RootAround { .. } => token.parse_array(
                &values,
                |[
                    body_part,
                    body_part_selection,
                    second_person_verb,
                    third_person_verb,
                ]| CasteToken::RootAround {
                    body_part_selector: vec![body_part, body_part_selection],
                    second_person_verb,
                    third_person_verb,
                },
            ),
            CasteToken::Secretion { .. } => token
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

                    Some(CasteToken::Secretion {
                        material,
                        material_state,
                        body_part_selector,
                        tissue_layer,
                        trigger,
                    })
                })
                .flatten(),
            CasteToken::SenseCreatureClass { .. } => token.parse_array(
                &values,
                |[creature_class, tile, foreground, background, brightness]| {
                    CasteToken::SenseCreatureClass {
                        creature_class,
                        tile,
                        foreground: foreground.parse::<u32>().unwrap_or_default(),
                        background: background.parse::<u32>().unwrap_or_default(),
                        brightness: brightness.parse::<u32>().unwrap_or_default(),
                    }
                },
            ),
            CasteToken::SetBodyPartGroup { .. } => token
                .parse_vector(&values, |body_part_selector| CasteToken::SetBodyPartGroup {
                    body_part_selector,
                }),
            CasteToken::SkillLearnRate { .. } => {
                token.parse_labeled_array(&values, |skill, [rate]| CasteToken::SkillLearnRate {
                    skill,
                    rate,
                })
            }
            CasteToken::SkillLearnRates { .. } => {
                token.parse_single(&values, |rate| CasteToken::SkillLearnRates { rate })
            }
            CasteToken::SkillRate { .. } => token.parse_labeled_array(
                &values,
                |skill,
                 [
                    improvement_rate,
                    decay_rate_unused,
                    decay_rate_rusty,
                    decay_rate_demotion,
                ]| CasteToken::SkillRate {
                    skill,
                    improvement_rate,
                    decay_rate_unused,
                    decay_rate_rusty,
                    decay_rate_demotion,
                },
            ),
            CasteToken::SkillRates { .. } => token.parse_array(
                &values,
                |[
                    improvement_rate,
                    decay_rate_unused,
                    decay_rate_rusty,
                    decay_rate_demotion,
                ]| CasteToken::SkillRates {
                    improvement_rate,
                    decay_rate_unused,
                    decay_rate_rusty,
                    decay_rate_demotion,
                },
            ),
            CasteToken::SkillRustRate { .. } => token.parse_labeled_array(
                &values,
                |skill, [decay_rate_unused, decay_rate_rusty, decay_rate_demotion]| {
                    CasteToken::SkillRustRate {
                        skill,
                        decay_rate_unused,
                        decay_rate_rusty,
                        decay_rate_demotion,
                    }
                },
            ),
            CasteToken::SkillRustRates { .. } => token.parse_array(
                &values,
                |[decay_rate_unused, decay_rate_rusty, decay_rate_demotion]| {
                    CasteToken::SkillRustRates {
                        decay_rate_unused,
                        decay_rate_rusty,
                        decay_rate_demotion,
                    }
                },
            ),
            CasteToken::SlainSpeech { .. } => {
                token.parse_single(&values, |speech_file| CasteToken::SlainSpeech { speech_file })
            }
            CasteToken::SoldierTile { .. } => {
                token.parse_single(&values, |tile| CasteToken::SoldierTile { tile })
            }
            CasteToken::SoldierAltTile { .. } => {
                token.parse_single(&values, |tile| CasteToken::SoldierAltTile { tile })
            }
            CasteToken::Sound { .. } => {
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
            CasteToken::SpecificFood { .. } => {
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

                Some(CasteToken::SpecificFood {
                    food_type: *food_type,
                    identifier,
                })
            }
            CasteToken::SyndromeDilutionFactor { .. } => {
                token.parse_labeled_array(&values, |syndrome, [percentage]| {
                    CasteToken::SyndromeDilutionFactor {
                        syndrome,
                        percentage,
                    }
                })
            }
            CasteToken::Tendons { .. } => {
                token.parse_vector_with_tail(&values, |material, healing_rate| CasteToken::Tendons {
                    material,
                    healing_rate,
                })
            }
            CasteToken::Tile { .. } => token.parse_single(&values, |tile| CasteToken::Tile { tile }),
            CasteToken::TissueLayer { .. } => {
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

                    CasteToken::TissueLayer {
                        body_part_selector: vec![body_part, body_part_selection],
                        tissue,
                        positioning,
                    }
                })
            }
            CasteToken::TissueLayerUnder { .. } => {
                token.parse_array(&values, |[body_part_selector, body_part, tissue]| {
                    CasteToken::TissueLayerUnder {
                        body_part_selector,
                        body_part,
                        tissue,
                    }
                })
            }
            CasteToken::TradeCapacity { .. } => {
                token.parse_single(&values, |capacity| CasteToken::TradeCapacity { capacity })
            }
            CasteToken::VerminBite { .. } => {
                token.parse_vector_with_tail(&values, |body: Vec<String>, state| {
                    let mut iter = body.into_iter();

                    // 1. Pop 'chance' (use token.parse_value to handle "NONE" -> 0 safely)
                    let chance = iter.next().and_then(|s| token.parse_value(&s)).unwrap_or(0);

                    // 2. Pop 'verb'
                    let verb = iter.next().unwrap_or_default();

                    // 3. Collect the rest as 'material'
                    let material: Vec<String> = iter.collect();

                    CasteToken::VerminBite {
                        chance,
                        verb,
                        material,
                        material_state: state,
                    }
                })
            }
            CasteToken::ViewRange { .. } => {
                token.parse_single(&values, |view_range| CasteToken::ViewRange { view_range })
            }
            CasteToken::VisionArc { .. } => {
                token.parse_array(&values, |[binocular, non_binocular]| CasteToken::VisionArc {
                    binocular,
                    non_binocular,
                })
            }
            CasteToken::Webber { .. } => {
                token.parse_vector(&values, |material| CasteToken::Webber { material })
            }
        }
    }
}
