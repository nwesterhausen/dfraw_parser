//! The tags for the tree parser

/// The tokens for the tree parser
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
pub enum TreeTag {
    /// A tree
    Tree,
    /// The name of the tree
    TrunkName,
    /// The maximum height of the trunk
    MaxTrunkHeight,
    /// The maximum diameter of the trunk
    MaxTrunkDiameter,
    /// The period of the trunk
    TrunkPeriod,
    /// The period of the trunk width
    TrunkWidthPeriod,
    /// The name of the branches
    BranchName,
    /// The density of the branches
    BranchDensity,
    /// The radius of the branches
    BranchRadius,
    /// The name of the heavy branches
    HeavyBranchesName,
    /// The density of the heavy branches
    HeavyBranchDensity,
    /// The radius of the heavy branches
    HeavyBranchRadius,
    /// The branching of the heavy branches
    TrunkBranching,
    /// The name of the roots
    RootName,
    /// The density of the roots
    RootDensity,
    /// The radius of the roots
    RootRadius,
    /// The name of the twigs
    TwigsName,
    /// Twigs are placed on the side of the branches
    TwigsSideBranches,
    /// Twigs are placed above the branches
    TwigsAboveBranches,
    /// Twigs are placed below the branches
    TwigsBelowBranches,
    /// Twigs are placed on the side of heavy branches
    TwigsSideHeavyBranches,
    /// Twigs are placed above heavy branches
    TwigsAboveHeavyBranches,
    /// Twigs are placed below heavy branches
    TwigsBelowHeavyBranches,
    /// Twigs are placed on the side of the trunk
    TwigsSideTrunk,
    /// Twigs are placed above the trunk
    TwigsAboveTrunk,
    /// Twigs are placed below the trunk
    TwigsBelowTrunk,
    /// The name of the tree canopy
    CapName,
    /// The period of the tree canopy
    CapPeriod,
    /// The radius of the tree canopy
    CapRadius,
    /// The tile to use for the tree
    TreeTile,
    /// The tile to use for a dead tree
    DeadTreeTile,
    /// The tile to use for a sapling
    SaplingTile,
    /// The tile to use for a dead sapling
    DeadSaplingTile,
    /// The color of the tree
    TreeColor,
    /// The color of a dead tree
    DeadTreeColor,
    /// The color of a sapling
    SaplingColor,
    /// The color of a dead sapling
    DeadSaplingColor,
    /// The level at which the spling will drown (in water)
    SaplingDrownLevel,
    /// The level at which the tree will drown (in water)
    TreeDrownLevel,
    // Actual Tags
    /// The tree has a rounded cap-hood like a giant mushroom. This severely stunts a tree's maximum height (known bug)
    TreeHasMushroomCap,
    /// Uses the standard names for the tree components (roots, trunk, branches, etc.)
    StandardTileNames,
    /// Makes young versions of the tree be called "[tree name] sapling"; otherwise, they are called "young [tree name]".
    Sapling,
    /// An unknown tree token
    #[default]
    Unknown,
}
