//! String token to parsed tag map for shrub tokens.

use crate::tokens::ShrubToken;

/// The mapping of shrub tokens to their string representation
pub static SHRUB_TOKENS: phf::Map<&'static str, ShrubToken> = phf::phf_map! {
  "SPRING" => ShrubToken::Spring,
  "SUMMER" => ShrubToken::Summer,
  "AUTUMN" => ShrubToken::Autumn,
  "WINTER" => ShrubToken::Winter,
  "GROWDUR" => ShrubToken::GrowDuration,
  "VALUE" => ShrubToken::Value,
  "PICKED_TILE" => ShrubToken::PickedTile,
  "DEAD_PICKED_TILE" => ShrubToken::DeadPickedTile,
  "SHRUB_TILE" => ShrubToken::ShrubTile,
  "DEAD_SHRUB_TILE" => ShrubToken::DeadShrubTile,
  "CLUSTER_SIZE" => ShrubToken::ClusterSize,
  "PICKED_COLOR" => ShrubToken::PickedColor,
  "DEAD_PICKED_COLOR" => ShrubToken::DeadPickedColor,
  "SHRUB_COLOR" => ShrubToken::ShrubColor,
  "DEAD_SHRUB_COLOR" => ShrubToken::DeadShrubColor,
  "SHRUB_DROWN_LEVEL" => ShrubToken::ShrubDrownLevel,
  "DRINK" => ShrubToken::Drink,
  "MILL" => ShrubToken::Mill,
  "THREAD" => ShrubToken::Thread,
  "SEED" => ShrubToken::Seed,
  "EXTRACT_STILL_VIAL" => ShrubToken::ExtractStillVial,
  "EXTRACT_VIAL" => ShrubToken::ExtractVial,
  "EXTRACT_BARREL" => ShrubToken::ExtractBarrel,
};
