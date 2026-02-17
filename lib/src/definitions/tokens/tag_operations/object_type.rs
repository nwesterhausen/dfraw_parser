use std::str::FromStr;

use crate::{
    tokens::{ObjectType, raw_definitions::OBJECT_TOKEN_MAP},
    traits::TagOperations,
};

impl TagOperations for ObjectType {
    fn parse(key: &str, _: &str) -> Option<Self> {
        let Some(token) = OBJECT_TOKEN_MAP.get(key) else {
            tracing::error!("ObjectType::parse_token: unknown token: {}", key);
            return None;
        };
        Some(*token)
    }
}

impl FromStr for ObjectType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s
            .strip_prefix('[')
            .unwrap_or(s)
            .strip_suffix(']')
            .unwrap_or(s);

        match Self::parse(trimmed, "") {
            Some(token) => Ok(token),
            None => Err(format!("ObjectType unable to parse {s}")),
        }
    }
}
