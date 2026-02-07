use std::str::FromStr;

use crate::{
    tokens::{InorganicToken, raw_definitions::INORGANIC_TOKENS},
    traits::TagOperations,
};

impl TagOperations for InorganicToken {
    fn parse(key: &str, _: &str) -> Option<Self> {
        let Some(token) = INORGANIC_TOKENS.get(key) else {
            tracing::error!("InorganicToken::parse_token: unknown token: {}", key);
            return None;
        };
        Some(*token)
    }
}

impl FromStr for InorganicToken {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s
            .strip_prefix('[')
            .unwrap_or(s)
            .strip_suffix(']')
            .unwrap_or(s);

        match Self::parse(trimmed, "") {
            Some(token) => Ok(token),
            None => Err(format!("InorganicToken unable to parse {s}")),
        }
    }
}
