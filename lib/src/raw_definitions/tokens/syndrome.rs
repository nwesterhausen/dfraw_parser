//! String token to parsed tag map for syndrome tokens.

use crate::tokens::SyndromeToken;

/// Mapping of syndrome tokens to strings
pub static SYNDROME_TOKENS: phf::Map<&'static str, SyndromeToken> = phf::phf_map! {
    "SYN_NAME" => SyndromeToken::Name,
    "SYN_IDENTIFIER" => SyndromeToken::Identifier,
    "SYN_INJECTED" => SyndromeToken::Injected,
    "SYN_CONTACT" => SyndromeToken::Contact,
    "SYN_INHALED" => SyndromeToken::Inhaled,
    "SYN_INGESTED" => SyndromeToken::Ingested,
    "SYN_AFFECTED_CLASS" => SyndromeToken::AffectedClass,
    "SYN_IMMUNE_CLASS" => SyndromeToken::ImmuneClass,
    "SYN_AFFECTED_CREATURE" => SyndromeToken::AffectedCreature,
    "SYN_IMMUNE_CREATURE" => SyndromeToken::ImmuneCreature,
    "SYN_CONCENTRATION_ADDED" => SyndromeToken::ConcentrationAdded,
    "SYN_NO_HOSPITAL" => SyndromeToken::NoHospital,
    "SYN_CLASS" => SyndromeToken::Class,
};
