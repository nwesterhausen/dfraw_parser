//! The tokens for the shrubs

/// The tokens for the shrubs
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
pub enum ShrubToken {
    /// The spring season
    Spring,
    /// The summer season
    Summer,
    /// The autumn season
    Autumn,
    /// The winter season
    Winter,
    /// The growth duration
    GrowDuration,
    /// The value of the shrub
    Value,
    /// The tile used for the shrub once it is picked
    PickedTile,
    /// The tile used for the shrub once it is dead and picked
    DeadPickedTile,
    /// The tile used for the shrub
    ShrubTile,
    /// The tile used for the dead shrub
    DeadShrubTile,
    /// The cluster size the shrubs will spawn in
    ClusterSize,
    /// The color of the shrub once it is picked
    PickedColor,
    /// The color of the shrub once it is dead and picked
    DeadPickedColor,
    /// The color of the shrub
    ShrubColor,
    /// The color of the dead shrub
    DeadShrubColor,
    /// The depth level the shrub will drown at
    ShrubDrownLevel,
    /// The shrub can be brewed
    Drink,
    /// The shrub can be milled
    Mill,
    /// The shrub can be spun
    Thread,
    /// The shrub has seeds
    Seed,
    /// The shrub can have a liquid extracted from it using a still and vial
    ExtractStillVial,
    /// The shrub can have a liquid extracted from it using a vial alone
    ExtractVial,
    /// The shrub can have a liquid extracted from it using a barrel
    ExtractBarrel,
    /// An unknown token
    #[default]
    Unknown,
}

impl std::fmt::Display for ShrubToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
