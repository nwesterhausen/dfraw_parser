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
pub enum TilePageTag {
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
