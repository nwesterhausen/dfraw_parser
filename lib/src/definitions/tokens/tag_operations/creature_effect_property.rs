use std::str::FromStr;

use crate::{
    tokens::{CreatureEffectPropertyToken, raw_definitions::CREATURE_EFFECT_PROPERTY_TOKENS},
    traits::TagOperations,
};

impl TagOperations for CreatureEffectPropertyToken {
    fn parse(key: &str, _: &str) -> Option<Self> {
        let Some(token) = CREATURE_EFFECT_PROPERTY_TOKENS.get(key) else {
            tracing::error!(
                "CreatureEffectPropertyToken::parse_token: unknown token: {}",
                key
            );
            return None;
        };
        Some(*token)
    }
}

impl FromStr for CreatureEffectPropertyToken {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s
            .strip_prefix('[')
            .unwrap_or(s)
            .strip_suffix(']')
            .unwrap_or(s);

        match Self::parse(trimmed, "") {
            Some(token) => Ok(token),
            None => Err(format!("CreatureEffectPropertyToken unable to parse {s}")),
        }
    }
}
