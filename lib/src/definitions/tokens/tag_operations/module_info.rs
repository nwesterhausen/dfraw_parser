use std::str::FromStr;

use crate::{
    tokens::{ModuleInfoToken, raw_definitions::MODULE_INFO_TOKENS},
    traits::{TagOperations, TokenParser as _},
};

impl TagOperations for ModuleInfoToken {
    fn parse(key: &str, value: &str) -> Option<Self> {
        let Some(token) = MODULE_INFO_TOKENS.get(key) else {
            tracing::error!("parse_token: unknown token: {key}");
            return None;
        };

        // No module info tokens use arrays of values, so we directly use `value`
        match token {
            ModuleInfoToken::NumericVersion { .. } => token.parse_single(&[value], |version| {
                ModuleInfoToken::NumericVersion { version }
            }),
            ModuleInfoToken::DisplayedVersion { .. } => token.parse_single(&[value], |version| {
                ModuleInfoToken::DisplayedVersion { version }
            }),
            ModuleInfoToken::EarliestCompatibleNumericVersion { .. } => token
                .parse_single(&[value], |version| {
                    ModuleInfoToken::EarliestCompatibleNumericVersion { version }
                }),
            ModuleInfoToken::EarliestCompatibleDisplayedVersion { .. } => token
                .parse_single(&[value], |version| {
                    ModuleInfoToken::EarliestCompatibleDisplayedVersion { version }
                }),
            ModuleInfoToken::Author { .. } => {
                token.parse_single(&[value], |name| ModuleInfoToken::Author { name })
            }
            ModuleInfoToken::Name { .. } => {
                token.parse_single(&[value], |name| ModuleInfoToken::Author { name })
            }
            ModuleInfoToken::Description { .. } => token.parse_single(&[value], |description| {
                ModuleInfoToken::Description { description }
            }),
            ModuleInfoToken::RequiresId { .. } => {
                token.parse_single(&[value], |id| ModuleInfoToken::RequiresId { id })
            }
            ModuleInfoToken::RequiresIdBeforeMe { .. } => {
                token.parse_single(&[value], |id| ModuleInfoToken::RequiresId { id })
            }
            ModuleInfoToken::RequiresIdAfterMe { .. } => {
                token.parse_single(&[value], |id| ModuleInfoToken::RequiresId { id })
            }
            ModuleInfoToken::ConflictsWithId { .. } => {
                token.parse_single(&[value], |id| ModuleInfoToken::RequiresId { id })
            }
            ModuleInfoToken::Unknown => None,
        }
    }
}

impl FromStr for ModuleInfoToken {
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
                None => Err(format!("ModuleInfoToken unable to parse {s}")),
            }
        } else {
            match Self::parse(trimmed, "") {
                Some(token) => Ok(token),
                None => Err(format!("ModuleInfoToken unable to parse {s}")),
            }
        }
    }
}
