//! String token to parsed tag map for material type tokens.

use crate::tokens::MaterialTypeToken;

/// Mapping of material type tokens to strings
pub static MATERIAL_TYPE_TOKENS: phf::Map<&'static str, MaterialTypeToken> = phf::phf_map! {
  "INORGANIC" => MaterialTypeToken::Inorganic,
  "STONE" => MaterialTypeToken::Stone,
  "METAL" => MaterialTypeToken::Metal,
  "COAL" => MaterialTypeToken::Coal,
  "CREATURE_MAT" => MaterialTypeToken::CreatureMaterial,
  "LOCAL_CREATURE_MAT" => MaterialTypeToken::LocalCreatureMaterial,
  "PLANT_MAT" => MaterialTypeToken::PlantMaterial,
  "LOCAL_PLANT_MAT" => MaterialTypeToken::LocalPlantMaterial,
  "GET_MATERIAL_FROM_REAGENT" => MaterialTypeToken::GetMaterialFromReagent,

  // Special "Hardcoded" Materials
  "AMBER" => MaterialTypeToken::Amber,
  "CORAL" => MaterialTypeToken::Coral,
  "GLASS_GREEN" => MaterialTypeToken::GlassGreen,
  "GLASS_CLEAR" => MaterialTypeToken::GlassClear,
  "GLASS_CRYSTAL" => MaterialTypeToken::GlassCrystal,
  "WATER" => MaterialTypeToken::Water,
  "POTASH" => MaterialTypeToken::Potash,
  "ASH" => MaterialTypeToken::Ash,
  "PEARLASH" => MaterialTypeToken::PearlAsh,
  "LYE" => MaterialTypeToken::Lye,
  "MUD" => MaterialTypeToken::Mud,
  "VOMIT" => MaterialTypeToken::Vomit,
  "SALT" => MaterialTypeToken::Salt,
  "FILTH_B" => MaterialTypeToken::FilthB,
  "FILTH_Y" => MaterialTypeToken::FilthY,
  "UNKNOWN_SUBSTANCE" => MaterialTypeToken::UnknownSubstance,
  "GRIME" => MaterialTypeToken::Grime,
};
