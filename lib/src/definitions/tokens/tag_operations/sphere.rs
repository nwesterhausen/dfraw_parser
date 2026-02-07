use std::str::FromStr;

use crate::{
    tokens::{SphereToken, raw_definitions::SPHERE_TOKENS},
    traits::TagOperations,
};

impl TagOperations for SphereToken {
    fn parse(key: &str, _: &str) -> Option<Self> {
        let Some(token) = SPHERE_TOKENS.get(key) else {
            tracing::error!("SphereToken::parse_token: unknown token: {}", key);
            return None;
        };
        Some(*token)
    }
}

impl FromStr for SphereToken {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s
            .strip_prefix('[')
            .unwrap_or(s)
            .strip_suffix(']')
            .unwrap_or(s);

        match Self::parse(trimmed, "") {
            Some(token) => Ok(token),
            None => Err(format!("SphereToken unable to parse {s}")),
        }
    }
}
