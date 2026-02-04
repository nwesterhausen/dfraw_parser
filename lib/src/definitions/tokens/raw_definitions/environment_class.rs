//! String token to parsed tag map for environment class tokens.

use crate::tokens::EnvironmentClassToken;

/// Map of environment classes to their string representation.
pub static ENVIRONMENT_CLASS_TOKENS: phf::Map<&'static str, EnvironmentClassToken> = phf::phf_map! {
  "ALL_STONE" => EnvironmentClassToken::AllStone,
  "IGNEOUS_ALL" => EnvironmentClassToken::IgneousAll,
  "IGNEOUS_EXTRUSIVE" => EnvironmentClassToken::IgneousExtrusive,
  "IGNEOUS_INTRUSIVE" => EnvironmentClassToken::IgneousIntrusive,
  "SOIL" => EnvironmentClassToken::Soil,
  "SOIL_SAND" => EnvironmentClassToken::SoilSand,
  "SOIL_OCEAN" => EnvironmentClassToken::SoilOcean,
  "SEDIMENTARY" => EnvironmentClassToken::Sedimentary,
  "METAMORPHIC" => EnvironmentClassToken::Metamorphic,
  "ALLUVIAL" => EnvironmentClassToken::Alluvial,
};
