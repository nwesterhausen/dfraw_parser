use std::str::FromStr;

use crate::{
    custom_types::Dimensions,
    tokens::{ConditionToken, raw_definitions::CONDITION_TOKENS},
    traits::{TagOperations, TokenParser},
};

impl TagOperations for ConditionToken {
    fn parse(key: &str, value: &str) -> Option<Self>
    where
        Self: Sized,
    {
        let Some(token) = CONDITION_TOKENS.get(key) else {
            tracing::error!("CasteToken::parse_token: unknown token: {}", key);
            return None;
        };

        // Split values into an array if possible, using an empty one if no values exist.
        let values: Vec<&str> = if value.is_empty() {
            Vec::new()
        } else {
            value.split(':').collect()
        };

        match token {
            ConditionToken::HaulCountMin { .. } => {
                token.parse_single(&values, |count| ConditionToken::HaulCountMin { count })
            }
            ConditionToken::HaulCountMax { .. } => {
                token.parse_single(&values, |count| ConditionToken::HaulCountMax { count })
            }
            ConditionToken::BodyPart { .. } => {
                token.parse_labeled_vector(&values, |specifier, selector| {
                    ConditionToken::BodyPart {
                        specifier,
                        selector,
                    }
                })
            }
            ConditionToken::BodySizeMin { .. } => {
                token.parse_single(&values, |size| ConditionToken::BodySizeMin { size })
            }
            ConditionToken::BodySizeMax { .. } => {
                token.parse_single(&values, |size| ConditionToken::BodySizeMax { size })
            }
            ConditionToken::SyndromeClass { .. } => token.parse_single(&values, |syndrome| {
                ConditionToken::SyndromeClass { syndrome }
            }),
            ConditionToken::TissueLayer { .. } => {
                token.parse_labeled_vector(&values, |specifier, selector| {
                    ConditionToken::TissueLayer {
                        specifier,
                        selector,
                    }
                })
            }
            ConditionToken::RandomPartIndex { .. } => {
                token.parse_labeled_array(&values, |identifier, [index, range]| {
                    ConditionToken::RandomPartIndex {
                        identifier,
                        index,
                        range,
                    }
                })
            }
            ConditionToken::TissueMayHaveColor { .. } => token.parse_vector(&values, |colors| {
                ConditionToken::TissueMayHaveColor { colors }
            }),
            ConditionToken::TissueMinLength { .. } => {
                token.parse_single(&values, |length| ConditionToken::TissueMinLength { length })
            }
            ConditionToken::TissueMaxLength { .. } => {
                token.parse_single(&values, |length| ConditionToken::TissueMaxLength { length })
            }
            ConditionToken::TissueMinDensity { .. } => token.parse_single(&values, |density| {
                ConditionToken::TissueMinDensity { density }
            }),
            ConditionToken::TissueMaxDensity { .. } => token.parse_single(&values, |density| {
                ConditionToken::TissueMaxDensity { density }
            }),
            ConditionToken::TissueMayHaveShaping { .. } => token.parse_single(&values, |shaping| {
                ConditionToken::TissueMayHaveShaping { shaping }
            }),
            ConditionToken::TissueSwap { .. } => {
                if let Some((key_value, tile_data)) = values.split_first_chunk::<2>() {
                    token
                        .parse_key_value(key_value, |condition, value| {
                            token.parse_labeled_array(tile_data, |tile_page_identifier, [x, y]| {
                                ConditionToken::TissueSwap {
                                    condition: condition.clone(),
                                    value,
                                    tile_page_identifier,
                                    tile_position: Dimensions::from_xy(x, y),
                                }
                            })
                        })
                        .flatten()
                } else {
                    None
                }
            }
            ConditionToken::ItemQuality { .. } => {
                token.parse_single(&values, |quality| ConditionToken::ItemQuality { quality })
            }
            ConditionToken::LayerSet { .. } => {
                token.parse_single(&values, |condition| ConditionToken::LayerSet { condition })
            }
            ConditionToken::LayerSetPalette { .. } => {
                token.parse_single(&values, |name| ConditionToken::LayerSetPalette { name })
            }
            ConditionToken::LayerSetPaletteFile { .. } => {
                token.parse_single(&values, |path| ConditionToken::LayerSetPaletteFile { path })
            }
            ConditionToken::LayerSetPaletteDefault { .. } => token
                .parse_single(&values, |default_row| {
                    ConditionToken::LayerSetPaletteDefault { default_row }
                }),
            ConditionToken::LayerGroupBodyPart { .. } => {
                token.parse_labeled_vector(&values, |specifier, selector| {
                    ConditionToken::LayerGroupBodyPart {
                        specifier,
                        selector,
                    }
                })
            }
            ConditionToken::ItemWorn { .. } => token.parse_labeled_vector_with_tail(
                &values,
                |specifier, selector, item_identifier| ConditionToken::ItemWorn {
                    specifier,
                    selector,
                    item_identifier,
                },
            ),
            ConditionToken::None
            | ConditionToken::Portrait
            | ConditionToken::Condition
            | ConditionToken::Default
            | ConditionToken::ChildPrime
            | ConditionToken::BabyPrime
            | ConditionToken::Animated
            | ConditionToken::Corpse
            | ConditionToken::ListIcon
            | ConditionToken::CdiListIcon
            | ConditionToken::TrainedHunter
            | ConditionToken::TrainedWar
            | ConditionToken::Skeleton
            | ConditionToken::SkeletonWithSkull
            | ConditionToken::Zombie
            | ConditionToken::Necromancer
            | ConditionToken::Male
            | ConditionToken::Female
            | ConditionToken::Baby
            | ConditionToken::VampireCursed
            | ConditionToken::Ghoul
            | ConditionToken::DisturbedDead
            | ConditionToken::Remains
            | ConditionToken::TaxEscort
            | ConditionToken::LawEnforcement
            | ConditionToken::Adventurer
            | ConditionToken::Glow
            | ConditionToken::GlowLeftGone
            | ConditionToken::GlowRightGone
            | ConditionToken::GlowChild
            | ConditionToken::Egg
            | ConditionToken::Vermin
            | ConditionToken::VerminAlt
            | ConditionToken::SwarmSmall
            | ConditionToken::SwarmMedium
            | ConditionToken::SwarmLarge
            | ConditionToken::LightVermin
            | ConditionToken::LightVerminAlt
            | ConditionToken::LightSwarmSmall
            | ConditionToken::LightSwarmMedium
            | ConditionToken::LightSwarmLarge
            | ConditionToken::Hive
            | ConditionToken::NotArtifact
            | ConditionToken::CraftedArtifact
            | ConditionToken::Crop
            | ConditionToken::Seed
            | ConditionToken::Picked
            | ConditionToken::Shrub
            | ConditionToken::Sapling
            | ConditionToken::CropSprout
            | ConditionToken::CropL
            | ConditionToken::CropM
            | ConditionToken::CropR
            | ConditionToken::ShrubDead
            | ConditionToken::Child
            | ConditionToken::NotChild
            | ConditionToken::Class
            | ConditionToken::BodyPartAppearanceModifierRange
            | ConditionToken::BodyPartPresent
            | ConditionToken::BodyPartScarred
            | ConditionToken::Ghost
            | ConditionToken::TissueMinCurly
            | ConditionToken::TissueMaxCurly
            | ConditionToken::TissueNotShaped
            | ConditionToken::Layer
            | ConditionToken::BodyUpper
            | ConditionToken::CopyOfTemplate
            | ConditionToken::LayerGroup
            | ConditionToken::EndLayerGroup
            | ConditionToken::ShutOffIfItemPresent
            | ConditionToken::Caste
            | ConditionToken::Dye
            | ConditionToken::NotDyed
            | ConditionToken::MaterialFlag
            | ConditionToken::MaterialType
            | ConditionToken::UsePalette
            | ConditionToken::UseStandardPaletteFromItem
            | ConditionToken::ProfessionCategory
            | ConditionToken::Hammerman
            | ConditionToken::MasterHammerman
            | ConditionToken::Spearman
            | ConditionToken::MasterSpearman
            | ConditionToken::Wrestler
            | ConditionToken::MasterWrestler
            | ConditionToken::Axeman
            | ConditionToken::MasterAxeman
            | ConditionToken::Swordsman
            | ConditionToken::MasterSwordsman
            | ConditionToken::Maceman
            | ConditionToken::MasterMaceman
            | ConditionToken::Pikeman
            | ConditionToken::MasterPikeman
            | ConditionToken::Recruit
            | ConditionToken::Thief
            | ConditionToken::MasterThief
            | ConditionToken::Lasher
            | ConditionToken::MasterLasher
            | ConditionToken::MonsterSlayer
            | ConditionToken::Crossbowman
            | ConditionToken::MasterCrossbowman
            | ConditionToken::Bowman
            | ConditionToken::MasterBowman
            | ConditionToken::Blowgunman
            | ConditionToken::MasterBlowgunman
            | ConditionToken::BeastHunter
            | ConditionToken::Scout
            | ConditionToken::Ranger
            | ConditionToken::Hunter
            | ConditionToken::Sage
            | ConditionToken::Scholar
            | ConditionToken::Philosopher
            | ConditionToken::Mathematician
            | ConditionToken::Historian
            | ConditionToken::Astronomer
            | ConditionToken::Naturalist
            | ConditionToken::Chemist
            | ConditionToken::Geographer
            | ConditionToken::Scribe
            | ConditionToken::Bookbinder
            | ConditionToken::Performer
            | ConditionToken::Poet
            | ConditionToken::Bard
            | ConditionToken::Dancer => token.parse_flag(&values, token.clone()),
        }
    }
}

impl FromStr for ConditionToken {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s
            .strip_prefix('[')
            .unwrap_or(s)
            .strip_suffix(']')
            .unwrap_or(s);

        if let Some((key, value)) = trimmed.split_once(':') {
            match Self::parse(key, value) {
                Some(token) => Ok(token),
                None => Err(format!("ConditionToken unable to parse {s}")),
            }
        } else {
            match Self::parse(trimmed, "") {
                Some(token) => Ok(token),
                None => Err(format!("ConditionToken unable to parse {s}")),
            }
        }
    }
}
