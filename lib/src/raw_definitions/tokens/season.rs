//! String token to parsed tag map for season tokens.

use crate::tokens::SeasonToken;

/// Mapping of position tokens to strings
pub static SEASON_TOKENS: phf::Map<&'static str, SeasonToken> = phf::phf_map! {
  "SPRING" => SeasonToken::Spring,
  "SUMMER" => SeasonToken::Summer,
  "AUTUMN" => SeasonToken::Autumn,
  "WINTER" => SeasonToken::Winter,
};
