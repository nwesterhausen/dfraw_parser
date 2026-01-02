//! The types of growths

/// The types of growths
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
pub enum PlantGrowthTypeTag {
    /// The growth is a leaf
    Leaves,
    /// The growth is a flower cluster
    Spathes,
    /// The growth is a fruit
    Fruit,
    /// The growth is a flower
    Flowers,
    /// The growth is a nut
    Nut,
    /// The growth is a seed catkin
    SeedCatkins,
    /// The growth is a pollen catkin
    PollenCatkins,
    /// The growth is a cone
    Cone,
    /// The growth is a seed cone
    SeedCone,
    /// The growth is a pollen cone
    PollenCone,
    /// The growth is a feather
    Feathers,
    /// The growth is an egg
    Eggs,
    /// The growth is a pod
    Pod,
    /// An unknown growth type
    #[default]
    None,
}
