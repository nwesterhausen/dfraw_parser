//! Plant graphic template tag

/// The graphic of the tile
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    specta::Type,
    Copy,
    strum_macros::EnumIter,
)]
#[serde(rename_all = "camelCase")]
pub enum PlantGraphicTemplateTag {
    /// The standard leaves
    StandardLeaves,
    /// The standard fruit 1
    StandardFruit1,
    /// The standard fruit 2
    StandardFruit2,
    /// The standard fruit 3
    StandardFruit3,
    /// The standard fruit 4
    StandardFruit4,
    /// The standard flowers 1
    StandardFlowers1,
    /// The standard flowers 2
    StandardFlowers2,
    /// The standard flowers 3
    StandardFlowers3,
    /// The standard flowers 4
    StandardFlowers4,
}
