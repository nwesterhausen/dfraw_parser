//! Contains the Position struct and implementation (for government positions)
use dfraw_parser_proc_macros::{Cleanable, IsEmpty};
use tracing::warn;

use crate::{tokens::PositionToken, traits::TagOperations as _};

/// Represents a position in the government of an entity
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    Default,
    specta::Type,
    PartialEq,
    Eq,
    IsEmpty,
    Cleanable,
)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    /// The identifier for the position
    pub identifier: String,
    /// The tokens defining this position
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    pub tokens: Vec<PositionToken>,
}

impl Position {
    /// Creates a new Position struct with the given identifier
    ///
    /// # Arguments
    ///
    /// * `identifier` - The identifier of the position
    #[must_use]
    pub fn new(identifier: String) -> Self {
        Self {
            identifier,
            ..Default::default()
        }
    }
    /// Parses a tag and value into the position
    ///
    /// # Arguments
    ///
    /// * `key` - The tag to parse
    /// * `value` - The value to parse
    pub fn parse_tag(&mut self, key: &str, value: &str) {
        let Some(token) = PositionToken::parse(key, value) else {
            warn!("PositionToken::parse failed to parse {key}:{value}");
            return;
        };

        self.tokens.push(token);
    }
}
