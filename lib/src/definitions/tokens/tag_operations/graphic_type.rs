use std::str::FromStr;

use crate::{
    tokens::{GraphicTypeToken, raw_definitions::GRAPHIC_TYPE_TOKENS},
    traits::TagOperations,
};

impl TagOperations for GraphicTypeToken {
    fn parse(key: &str, _: &str) -> Option<Self> {
        let Some(token) = GRAPHIC_TYPE_TOKENS.get(key) else {
            tracing::error!("GraphicTypeToken::parse_token: unknown token: {}", key);
            return None;
        };
        Some(*token)
    }
}

impl FromStr for GraphicTypeToken {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s
            .strip_prefix('[')
            .unwrap_or(s)
            .strip_suffix(']')
            .unwrap_or(s);

        match Self::parse(trimmed, "") {
            Some(token) => Ok(token),
            None => Err(format!("GraphicTypeToken unable to parse {s}")),
        }
    }
}
