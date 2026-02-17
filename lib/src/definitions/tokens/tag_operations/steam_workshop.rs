use std::str::FromStr;

use crate::{
    tokens::{SteamWorkshopToken, raw_definitions::STEAM_WORKSHOP_TOKENS},
    traits::{TagOperations, TokenParser},
};

impl TagOperations for SteamWorkshopToken {
    fn parse(key: &str, value: &str) -> Option<Self> {
        let Some(token) = STEAM_WORKSHOP_TOKENS.get(key) else {
            tracing::error!("parse_token: unknown token: {key}");
            return None;
        };

        // Only a single token uses multiple values, so we split `value` in that arm.
        match token {
            SteamWorkshopToken::Title { .. } => {
                token.parse_single(&[value], |title| SteamWorkshopToken::Title { title })
            }
            SteamWorkshopToken::Description { .. } => token.parse_single(&[value], |description| {
                SteamWorkshopToken::Description { description }
            }),
            SteamWorkshopToken::Tag { .. } => {
                token.parse_single(&[value], |tag| SteamWorkshopToken::Tag { tag })
            }
            SteamWorkshopToken::KeyValueTag { .. } => {
                let values: Vec<&str> = value.split(":").collect();
                token.parse_key_value(&values, |key, value| SteamWorkshopToken::KeyValueTag {
                    key,
                    value,
                })
            }
            SteamWorkshopToken::Metadata { .. } => token.parse_single(&[value], |metadata| {
                SteamWorkshopToken::Metadata { metadata }
            }),
            SteamWorkshopToken::Changelog { .. } => token.parse_single(&[value], |changes| {
                SteamWorkshopToken::Changelog { changes }
            }),
            SteamWorkshopToken::FileId { .. } => {
                token.parse_single(&[value], |id| SteamWorkshopToken::FileId { id })
            }
            SteamWorkshopToken::Unknown => None,
        }
    }
}

impl FromStr for SteamWorkshopToken {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s
            .strip_prefix('[')
            .unwrap_or(s)
            .strip_suffix(']')
            .unwrap_or(s);

        if let Some((key, value)) = trimmed.split_once(':') {
            match Self::parse(key, value) {
                Some(token) => Ok(token),
                None => Err(format!("SteamWorkshopToken unable to parse {s}")),
            }
        } else {
            match Self::parse(trimmed, "") {
                Some(token) => Ok(token),
                None => Err(format!("SteamWorkshopToken unable to parse {s}")),
            }
        }
    }
}
