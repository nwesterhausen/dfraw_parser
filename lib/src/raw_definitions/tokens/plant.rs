//! String token to parsed tag map for plant tokens.

use crate::tokens::PlantToken;

/// Mapping of plant tokens to strings
pub static PLANT_TOKENS: phf::Map<&'static str, PlantToken> = phf::phf_map! {
    "NAME" => PlantToken::NameSingular,
    "NAME_PLURAL" => PlantToken::NamePlural,
    "ADJ" => PlantToken::NameAdjective,
    "ALL_NAMES" => PlantToken::AllNames,
    "PREFSTRING" => PlantToken::PrefString,
    "MATERIAL" => PlantToken::Material,
    "USE_MATERIAL" => PlantToken::UseMaterial,
    "BASIC_MAT" => PlantToken::BasicMaterial,
    "USE_MATERIAL_TEMPLATE" => PlantToken::UseMaterialTemplate,
    "UNDERGROUND_DEPTH" => PlantToken::UndergroundDepth,
    "GOOD" => PlantToken::Good,
    "EVIL" => PlantToken::Evil,
    "SAVAGE" => PlantToken::Savage,
    "FREQUENCY" => PlantToken::Frequency,
    "WET" => PlantToken::Wet,
    "DRY" => PlantToken::Dry,
    "BIOME" => PlantToken::Biome,
};
