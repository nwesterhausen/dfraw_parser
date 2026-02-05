use std::str::FromStr;

use crate::{
    tokens::{CreatureToken, raw_definitions::CREATURE_TOKENS},
    traits::{TagOperations, TokenParser as _},
};

impl TagOperations for CreatureToken {
    fn parse(key: &str, value: &str) -> Option<Self> {
        // Implement the logic for parsing the token from the key and value.
        // Create a new `CreatureTag` instance and return it, or return `None` if the token could not be parsed.

        let Some(token) = CREATURE_TOKENS.get(key) else {
            tracing::warn!("parse_token: unknown token: {key}");
            return None;
        };

        // Split values into an array if possible, using an empty one if no values exist.
        let values: Vec<&str> = if value.is_empty() {
            Vec::new()
        } else {
            value.split(':').collect()
        };

        match token {
            CreatureToken::ApplyCurrentCreatureVariation
            | CreatureToken::ArtificialHiveable
            | CreatureToken::DoesNotExist
            | CreatureToken::EquipmentWagon
            | CreatureToken::Evil
            | CreatureToken::Fanciful
            | CreatureToken::Generated
            | CreatureToken::Good
            | CreatureToken::GoToEnd
            | CreatureToken::GoToStart
            | CreatureToken::LargeRoaming
            | CreatureToken::LocalPopsControllable
            | CreatureToken::LocalPopsProduceHeroes
            | CreatureToken::LooseClusters
            | CreatureToken::Mundane
            | CreatureToken::Savage
            | CreatureToken::Ubiquitous
            | CreatureToken::Utterances
            | CreatureToken::VerminEater
            | CreatureToken::VerminFish
            | CreatureToken::VerminGrounder
            | CreatureToken::VerminRotter
            | CreatureToken::VerminSoil
            | CreatureToken::VerminSoilColony
            | CreatureToken::Unknown
            | CreatureToken::MatesToBreed
            | CreatureToken::TwoGenders
            | CreatureToken::AllCastesAlive
            | CreatureToken::SmallRace
            | CreatureToken::OccursAsEntityRace
            | CreatureToken::Equipment => {
                // Emits a warning if there are values when we expect none.
                // Using it in the trait allows us to adjust it if needed in the future.
                token.parse_flag(&values, token.clone())
            }
            CreatureToken::AltTile { .. } => {
                token.parse_single(&values, |character| CreatureToken::AltTile { character })
            }
            CreatureToken::ApplyCreatureVariation { .. } => {
                token.parse_labeled_vector(&values, |id, args| {
                    CreatureToken::ApplyCreatureVariation { id, args }
                })
            }
            CreatureToken::Biome { .. } => {
                token.parse_single(&values, |id| CreatureToken::Biome { id })
            }
            CreatureToken::Caste { .. } => {
                token.parse_single(&values, |name| CreatureToken::Caste { name })
            }
            CreatureToken::ChangeFrequencyPercent { .. } => token
                .parse_single(&values, |percent| CreatureToken::ChangeFrequencyPercent {
                    percent,
                }),
            CreatureToken::ClusterNumber { .. } => token.parse_array(&values, |[min, max]| {
                CreatureToken::ClusterNumber { min, max }
            }),
            CreatureToken::CopyTagsFrom { .. } => {
                token.parse_single(&values, |creature| CreatureToken::CopyTagsFrom { creature })
            }
            CreatureToken::CreatureSoldierTile { .. } => token.parse_single(&values, |character| {
                CreatureToken::CreatureSoldierTile { character }
            }),
            CreatureToken::CreatureTile { .. } => token.parse_single(&values, |character| {
                CreatureToken::CreatureTile { character }
            }),
            CreatureToken::Color { .. } => {
                token.parse_single(&values, |color| CreatureToken::Color { color })
            }
            CreatureToken::Frequency { .. } => {
                token.parse_single(&values, |frequency| CreatureToken::Frequency { frequency })
            }
            CreatureToken::GeneralBabyName { .. } => {
                token.parse_single(&values, |name| CreatureToken::GeneralBabyName { name })
            }
            CreatureToken::GeneralChildName { .. } => {
                token.parse_single(&values, |name| CreatureToken::GeneralChildName { name })
            }
            CreatureToken::GlowColor { .. } => {
                token.parse_single(&values, |color| CreatureToken::GlowColor { color })
            }
            CreatureToken::GlowTile { .. } => {
                token.parse_single(&values, |character| CreatureToken::GlowTile { character })
            }
            CreatureToken::GoToTag { .. } => {
                token.parse_single(&values, |tag| CreatureToken::GoToTag { tag })
            }
            CreatureToken::HarvestProduct { .. } => {
                // Check if there are at least 3 arguments
                if values.len() < 3 {
                    tracing::warn!(
                        "not enough arguments for HarvestProduct: {}/3: {:?}",
                        values.len(),
                        &values
                    );
                    return None;
                }

                let Ok(number) = (*values.first().unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: HarvestProduct failed to parse number value in position 0: {values:?}"
                    );
                    return None;
                };
                let Ok(time) = (*values.get(1).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: HarvestProduct failed to parse time value in position 1: {values:?}"
                    );
                    return None;
                };

                let item_tokens: Vec<String> = values
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect();
                Some(Self::HarvestProduct {
                    number,
                    time,
                    item_tokens,
                })
            }
            CreatureToken::Name { .. } => {
                token.parse_single(&values, |name| CreatureToken::Name { name })
            }
            CreatureToken::PlusMaterial { .. } => {
                token.parse_single(&values, |material| CreatureToken::PlusMaterial { material })
            }
            CreatureToken::PopulationNumber { .. } => token.parse_array(&values, |[min, max]| {
                CreatureToken::PopulationNumber { min, max }
            }),
            CreatureToken::PrefString { .. } => token.parse_single(&values, |pref_string| {
                CreatureToken::PrefString { pref_string }
            }),
            CreatureToken::ProfessionName { .. } => {
                token.parse_array(&values, |[id, name, plural_name]| {
                    CreatureToken::ProfessionName {
                        id,
                        name,
                        plural_name,
                    }
                })
            }
            CreatureToken::RemoveMaterial { .. } => token.parse_single(&values, |material| {
                CreatureToken::RemoveMaterial { material }
            }),
            CreatureToken::RemoveTissue { .. } => {
                token.parse_single(&values, |tissue| CreatureToken::RemoveTissue { tissue })
            }
            CreatureToken::SelectAdditionalCaste { .. } => token.parse_single(&values, |caste| {
                CreatureToken::SelectAdditionalCaste { caste }
            }),
            CreatureToken::SelectCaste { .. } => {
                token.parse_single(&values, |caste| CreatureToken::SelectCaste { caste })
            }
            CreatureToken::SelectMaterial { .. } => token.parse_single(&values, |material| {
                CreatureToken::SelectMaterial { material }
            }),
            CreatureToken::SelectTissue { .. } => {
                token.parse_single(&values, |tissue| CreatureToken::SelectTissue { tissue })
            }
            CreatureToken::SlainSpeech { .. } => token.parse_single(&values, |slain_speech| {
                CreatureToken::SlainSpeech { slain_speech }
            }),
            CreatureToken::SmellTrigger { .. } => token.parse_single(&values, |smell_trigger| {
                CreatureToken::SmellTrigger { smell_trigger }
            }),
            CreatureToken::SoldierAltTile { .. } => {
                token.parse_single(&values, |tile| CreatureToken::SoldierAltTile { tile })
            }
            CreatureToken::SourceHfid { .. } => {
                token.parse_single(&values, |hfid| CreatureToken::SourceHfid { hfid })
            }
            CreatureToken::Sphere { .. } => {
                token.parse_single(&values, |sphere| CreatureToken::Sphere { sphere })
            }
            CreatureToken::Tissue { .. } => {
                token.parse_single(&values, |name| CreatureToken::Tissue { name })
            }
            CreatureToken::TriggerableGroup { .. } => token.parse_array(&values, |[min, max]| {
                CreatureToken::TriggerableGroup { min, max }
            }),
            CreatureToken::UndergroundDepth { .. } => token.parse_array(&values, |[min, max]| {
                CreatureToken::UndergroundDepth { min, max }
            }),
            CreatureToken::UseCaste { .. } => {
                token.parse_array(&values, |[caste, original_caste]| CreatureToken::UseCaste {
                    caste,
                    original_caste,
                })
            }
            CreatureToken::UseMaterial { .. } => {
                token.parse_array(&values, |[material, original_material]| {
                    CreatureToken::UseMaterial {
                        material,
                        original_material,
                    }
                })
            }
            CreatureToken::UseMaterialTemplate { .. } => token
                .parse_array(&values, |[material, template]| {
                    CreatureToken::UseMaterialTemplate { material, template }
                }),
            CreatureToken::UseTissue { .. } => {
                token.parse_array(&values, |[tissue, original_tissue]| {
                    CreatureToken::UseTissue {
                        tissue,
                        original_tissue,
                    }
                })
            }
            CreatureToken::UseTissueTemplate { .. } => token
                .parse_array(&values, |[tissue, template]| {
                    CreatureToken::UseTissueTemplate { tissue, template }
                }),
        }
    }
}

impl FromStr for CreatureToken {
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
                None => Err(format!("CreatureToken unable to parse {s}")),
            }
        } else {
            match Self::parse(trimmed, "") {
                Some(token) => Ok(token),
                None => Err(format!("CreatureToken unable to parse {s}")),
            }
        }
    }
}
