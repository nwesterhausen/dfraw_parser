//! String token to parsed tag map for inorganic tokens.

use crate::tokens::InorganicToken;

/// Map of inorganic tokens to their string representation.
pub static INORGANIC_TOKENS: phf::Map<&'static str, InorganicToken> = phf::phf_map! {
  "WAFERS" => InorganicToken::Wafers,
  "DEEP_SPECIAL" => InorganicToken::DeepSpecial,
  "METAL_ORE" => InorganicToken::MetalOre,
  "THREAD_METAL" => InorganicToken::ThreadMetal,
  "DEEP_SURFACE" => InorganicToken::DeepSurface,
  "AQUIFER" => InorganicToken::Aquifer,
  "METAMORPHIC" => InorganicToken::Metamorphic,
  "SEDIMENTARY" => InorganicToken::Sedimentary,
  "SOIL" => InorganicToken::Soil,
  "SOIL_OCEAN" => InorganicToken::SoilOcean,
  "SOIL_SAND" => InorganicToken::SoilSand,
  "SEDIMENTARY_OCEAN_SHALLOW" => InorganicToken::SedimentaryOceanShallow,
  "SEDIMENTARY_OCEAN_DEEP" => InorganicToken::SedimentaryOceanDeep,
  "IGNEOUS_EXTRUSIVE" => InorganicToken::IgneousExtrusive,
  "IGNEOUS_INTRUSIVE" => InorganicToken::IgneousIntrusive,
  "ENVIRONMENT" => InorganicToken::Environment,
  "ENVIRONMENT_SPEC" => InorganicToken::EnvironmentSpecific,
  "LAVA" => InorganicToken::Lava,
  "SPECIAL" => InorganicToken::Special,
  "GENERATED" => InorganicToken::Generated,
  "DIVINE" => InorganicToken::Divine,
  "SPHERE" => InorganicToken::Sphere,
};
