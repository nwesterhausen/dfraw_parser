use std::str::FromStr;

use crate::{
    tokens::{BiomeToken, raw_definitions::BIOME_TOKENS},
    traits::TagOperations,
};

impl TagOperations for BiomeToken {
    fn parse(key: &str, _: &str) -> Option<Self> {
        let Some(token) = BIOME_TOKENS.get(key) else {
            tracing::error!("BiomeToken::parse_token: unknown token: {}", key);
            return None;
        };
        Some(*token)
    }
}

impl FromStr for BiomeToken {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s
            .strip_prefix('[')
            .unwrap_or(s)
            .strip_suffix(']')
            .unwrap_or(s);

        match Self::parse(trimmed, "") {
            Some(token) => Ok(token),
            None => Err(format!("BiomeToken unable to parse {s}")),
        }
    }
}
