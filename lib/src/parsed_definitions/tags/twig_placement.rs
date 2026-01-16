//! Twig placement tags

use crate::traits::IsEmpty;

/// The placement of twigs on a tree
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
pub enum TwigPlacementTag {
    /// Twigs are placed on the side of the tree
    SideBranches,
    /// Twigs are placed above the tree
    AboveBranches,
    /// Twigs are placed below the tree
    BelowBranches,
    /// Twigs are placed on the side of heavy branches
    SideHeavyBranches,
    /// Twigs are placed above heavy branches
    AboveHeavyBranches,
    /// Twigs are placed below heavy branches
    BelowHeavyBranches,
    /// Twigs are placed on the side of the trunk
    SideTrunk,
    /// Twigs are placed above the trunk
    AboveTrunk,
    /// Twigs are placed below the trunk
    BelowTrunk,
    /// An unknown twig placement
    #[default]
    Unknown,
}

impl std::fmt::Display for TwigPlacementTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl IsEmpty for TwigPlacementTag {
    fn is_empty(&self) -> bool {
        self == &Self::Unknown
    }
}
