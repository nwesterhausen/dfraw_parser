//! The tokens for the seasons

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
pub enum SeasonTag {
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

impl std::fmt::Display for SeasonTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
