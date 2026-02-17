use std::str::FromStr;

use crate::{
    tokens::{CreatureEffectToken, raw_definitions::CREATURE_EFFECT_TOKENS},
    traits::TagOperations,
};

impl TagOperations for CreatureEffectToken {
    fn parse(key: &str, _: &str) -> Option<Self> {
        let Some(token) = CREATURE_EFFECT_TOKENS.get(key) else {
            tracing::error!("CreatureEffectToken::parse_token: unknown token: {}", key);
            return None;
        };
        Some(*token)
    }
}

impl FromStr for CreatureEffectToken {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s
            .strip_prefix('[')
            .unwrap_or(s)
            .strip_suffix(']')
            .unwrap_or(s);

        match Self::parse(trimmed, "") {
            Some(token) => Ok(token),
            None => Err(format!("CreatureEffectToken unable to parse {s}")),
        }
    }
}
