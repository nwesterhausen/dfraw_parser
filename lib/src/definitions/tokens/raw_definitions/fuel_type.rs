//! String token to parsed tag map for fuel type tokens.

use crate::tokens::FuelTypeToken;

/// Mapping of fuel type tokens to strings
pub static FUEL_TYPE_TOKENS: phf::Map<&'static str, FuelTypeToken> = phf::phf_map! {
  "COAL" => FuelTypeToken::Charcoal,
  "COKE" => FuelTypeToken::Coke,
  "NO_MATGLOSS" => FuelTypeToken::NoMaterialGloss,
};
