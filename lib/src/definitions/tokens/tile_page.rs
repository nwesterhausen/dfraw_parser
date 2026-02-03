//! The tokens used to define the tile page

/// The tokens used to define the tile page
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
#[serde(rename_all = "camelCase")]
pub enum TilePageToken {
    /// The dimensions of the tile
    TileDim,
    /// The dimensions of the page
    PageDim,
    /// The file path
    File,
    /// An unknown token
    #[default]
    Unknown,
}

impl std::fmt::Display for TilePageToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
