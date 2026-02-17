use std::str::FromStr;

use crate::{
    tokens::{PlantPartToken, raw_definitions::PLANT_PART_TOKENS},
    traits::TagOperations,
};

impl TagOperations for PlantPartToken {
    fn parse(key: &str, _: &str) -> Option<Self> {
        let Some(token) = PLANT_PART_TOKENS.get(key) else {
            tracing::error!("PlantPartToken::parse_token: unknown token: {}", key);
            return None;
        };
        Some(*token)
    }
}

impl FromStr for PlantPartToken {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s
            .strip_prefix('[')
            .unwrap_or(s)
            .strip_suffix(']')
            .unwrap_or(s);

        match Self::parse(trimmed, "") {
            Some(token) => Ok(token),
            None => Err(format!("PlantPartToken unable to parse {s}")),
        }
    }
}
