//! String token to parsed tag map for growth tag tokens.

use crate::tokens::GrowthToken;

/// Map of growth tag tags to their string representation.
pub static GROWTH_TOKENS: phf::Map<&'static str, GrowthToken> = phf::phf_map! {
  "GROWTH_FRUIT" => GrowthToken::Fruit,
  // [GROWTH_1:GRASS_FLOWERS:0:1]
  "GROWTH_1" => GrowthToken::Growth1,
  // [GROWTH_2:GRASS_FLOWERS:1:1]
  "GROWTH_2" => GrowthToken::Growth2,
  // [GROWTH_3:GRASS_FLOWERS:2:1]
  "GROWTH_3" => GrowthToken::Growth3,
  // [GROWTH_4:GRASS_FLOWERS:3:1]
  "GROWTH_4" => GrowthToken::Growth4,
};
