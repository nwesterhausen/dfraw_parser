use std::str::FromStr;

use crate::{
    tokens::{PlantGrowthToken, raw_definitions::PLANT_GROWTH_TOKENS},
    traits::TagOperations,
};

impl TagOperations for PlantGrowthToken {
    fn parse(key: &str, _: &str) -> Option<Self> {
        let Some(token) = PLANT_GROWTH_TOKENS.get(key) else {
            tracing::error!("PlantGrowthToken::parse_token: unknown token: {}", key);
            return None;
        };
        Some(*token)
    }
}

impl FromStr for PlantGrowthToken {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s
            .strip_prefix('[')
            .unwrap_or(s)
            .strip_suffix(']')
            .unwrap_or(s);

        match Self::parse(trimmed, "") {
            Some(token) => Ok(token),
            None => Err(format!("PlantGrowthToken unable to parse {s}")),
        }
    }
}
