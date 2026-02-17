//! String token to parsed tag map for plant growth type tokens.

use crate::tokens::PlantGrowthTypeToken;

/// Mapping of growth type tokens to strings
pub static PLANT_GROWTH_TYPE_TOKENS: phf::Map<&'static str, PlantGrowthTypeToken> = phf::phf_map! {
  "LEAVES" => PlantGrowthTypeToken::Leaves,
  "SPATHES" => PlantGrowthTypeToken::Spathes,
  "FRUIT" => PlantGrowthTypeToken::Fruit,
  "FLOWERS" => PlantGrowthTypeToken::Flowers,
  "NUT" => PlantGrowthTypeToken::Nut,
  "SEED_CATKINS" => PlantGrowthTypeToken::SeedCatkins,
  "POLLEN_CATKINS" => PlantGrowthTypeToken::PollenCatkins,
  "CONE" => PlantGrowthTypeToken::Cone,
  "SEED_CONE" => PlantGrowthTypeToken::SeedCone,
  "POLLEN_CONE" => PlantGrowthTypeToken::PollenCone,
  "FEATHERS" => PlantGrowthTypeToken::Feathers,
  "EGGS" => PlantGrowthTypeToken::Eggs,
  "POD" => PlantGrowthTypeToken::Pod,
  "NONE" => PlantGrowthTypeToken::None,
};
