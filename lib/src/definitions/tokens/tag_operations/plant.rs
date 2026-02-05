use std::str::FromStr;

use crate::{
    tokens::{PlantToken, raw_definitions::PLANT_TOKENS},
    traits::TagOperations,
};

impl TagOperations for PlantToken {
    fn parse(key: &str, _: &str) -> Option<Self> {
        let Some(token) = PLANT_TOKENS.get(key) else {
            tracing::error!("PlantToken::parse_token: unknown token: {}", key);
            return None;
        };
        Some(*token)
    }
}

impl FromStr for PlantToken {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s
            .strip_prefix('[')
            .unwrap_or(s)
            .strip_suffix(']')
            .unwrap_or(s);

        match Self::parse(trimmed, "") {
            Some(token) => Ok(token),
            None => Err(format!("PlantToken unable to parse {s}")),
        }
    }
}
