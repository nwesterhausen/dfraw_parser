//! The color modification of the tile

use tracing::warn;

/// The color modification of the tile
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    specta::Type,
    strum_macros::EnumIter,
)]
#[serde(rename_all = "camelCase")]
pub enum ColorModificationTag {
    /// The color is as is
    #[default]
    AsIs,
}

impl ColorModificationTag {
    /// Parse a token into a `ColorModification`
    ///
    /// # Arguments
    ///
    /// * `token` - The token to parse
    ///
    /// # Returns
    ///
    /// The parsed `ColorModification`
    #[must_use]
    pub fn from_token(token: &str) -> Self {
        if token == "AS_IS" {
            Self::AsIs
        } else {
            warn!("Failed to parse {} as ColorModification", token);
            Self::default()
        }
    }
    /// Whether the `ColorModification` is the default value.
    ///
    /// # Returns
    ///
    /// True if the `ColorModification` is the default value, false otherwise.
    #[must_use]
    pub const fn is_default(&self) -> bool {
        matches!(self, Self::AsIs)
    }
}
