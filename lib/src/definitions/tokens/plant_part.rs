//! Plant part tags

/// Parts of a plant
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
pub enum PlantPartToken {
    /// Twigs
    Twigs,
    /// Branches
    Branches,
    /// Branches and twigs
    BranchesAndTwigs,
    /// All branches and twigs
    AllBranchesAndTwigs,
    /// Heavy branches
    HeavyBranches,
    /// Heavy branches and twigs
    HeavyBranchesAndTrunk,
    /// Trunk
    Trunk,
    /// Roots
    Roots,
    /// Cap (canopy)
    Cap,
    /// Sapling
    Sapling,
    /// An unknown part of the plant
    #[default]
    Unknown,
}

impl std::fmt::Display for PlantPartToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
