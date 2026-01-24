//! The tokens for the seasons

use crate::traits::IsEmpty;

/// The tokens for the seasons
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Default,
    specta::Type,
    Copy,
    strum_macros::EnumIter,
)]
pub enum SeasonToken {
    /// The spring season
    Spring,
    /// The summer season
    Summer,
    /// The autumn season
    Autumn,
    /// The winter season
    Winter,
    /// An unknown season
    #[default]
    Unknown,
}

impl std::fmt::Display for SeasonToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl IsEmpty for SeasonToken {
    fn is_empty(&self) -> bool {
        self == &Self::Unknown
    }
}
