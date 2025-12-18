//! Growth tags for tiles

/// The growth tag of the tile
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
pub enum GrowthTag {
    /// The tile is a fruit
    Fruit,
    /// The tile is growth-1
    Growth1,
    /// The tile is growth-2
    Growth2,
    /// The tile is growth-3
    Growth3,
    /// The tile is growth-4
    Growth4,
    /// The tile is "as-is"
    #[default]
    AsIs,
}
