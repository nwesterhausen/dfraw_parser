use std::str::FromStr;

use crate::{
    tokens::{CreatureVariationToken, raw_definitions::CREATURE_VARIATION_TOKENS},
    traits::TagOperations,
};

impl TagOperations for CreatureVariationToken {
    fn parse(key: &str, _: &str) -> Option<Self> {
        let Some(token) = CREATURE_VARIATION_TOKENS.get(key) else {
            tracing::error!(
                "CreatureVariationToken::parse_token: unknown token: {}",
                key
            );
            return None;
        };
        Some(*token)
    }
}

impl FromStr for CreatureVariationToken {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s
            .strip_prefix('[')
            .unwrap_or(s)
            .strip_suffix(']')
            .unwrap_or(s);

        match Self::parse(trimmed, "") {
            Some(token) => Ok(token),
            None => Err(format!("CreatureVariationToken unable to parse {s}")),
        }
    }
}
