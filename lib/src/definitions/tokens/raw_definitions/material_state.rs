use crate::tokens::MaterialStateToken;

/// Mapping of fuel type tokens to strings
pub static MATERIAL_STATE_TOKENS: phf::Map<&'static str, MaterialStateToken> = phf::phf_map! {
    "SOLID" => MaterialStateToken::Solid,
    "LIQUID" => MaterialStateToken::Liquid,
    "GAS" => MaterialStateToken::Gas,
    "SOLID_POWDER" => MaterialStateToken::Powder,
    "POWDER" => MaterialStateToken::Powder,
    "SOLID_PASTE" => MaterialStateToken::Paste,
    "PASTE" => MaterialStateToken::Paste,
    "SOLID_PRESSED" => MaterialStateToken::Pressed,
    "PRESSED" => MaterialStateToken::Pressed,
    "ALL" => MaterialStateToken::All,
    "ALL_SOLID" => MaterialStateToken::AllSolid,
};
