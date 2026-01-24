//! String token to parsed tag map for plant growth tokens.

use crate::tokens::PlantGrowthToken;

/// Mapping of growth tokens to strings
pub static PLANT_GROWTH_TOKENS: phf::Map<&'static str, PlantGrowthToken> = phf::phf_map! {
  "GROWTH" => PlantGrowthToken::Growth,
  "GROWTH_NAME" => PlantGrowthToken::GrowthName,
  "GROWTH_ITEM" => PlantGrowthToken::GrowthItem,
  "GROWTH_HOST_TILE" => PlantGrowthToken::GrowthHostTile,
  "GROWTH_TRUNK_HEIGHT_PERC" => PlantGrowthToken::GrowthTrunkHeightPercent,
  "GROWTH_DENSITY" => PlantGrowthToken::GrowthDensity,
  "GROWTH_TIMING" => PlantGrowthToken::GrowthTiming,
  "GROWTH_PRINT" => PlantGrowthToken::GrowthPrint,
  "GROWTH_HAS_SEED" => PlantGrowthToken::GrowthHasSeed,
  "GROWTH_DROPS_OFF" => PlantGrowthToken::GrowthDropsOff,
  "GROWTH_DROPS_OFF_NO_CLOUD" => PlantGrowthToken::GrowthDropsOffNoCloud,
};
