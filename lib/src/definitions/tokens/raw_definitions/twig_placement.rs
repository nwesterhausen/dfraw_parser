//! String token to parsed tag map for twig placement tokens.

use crate::tokens::TwigPlacementToken;

/// The mapping of tree tokens to their string representation
pub static TWIG_PLACEMENT_TOKENS: phf::Map<&'static str, TwigPlacementToken> = phf::phf_map! {
  "SIDE_BRANCHES" => TwigPlacementToken::SideBranches,
  "ABOVE_BRANCHES" => TwigPlacementToken::AboveBranches,
  "BELOW_BRANCHES" => TwigPlacementToken::BelowBranches,
  "SIDE_HEAVY_BRANCHES" => TwigPlacementToken::SideHeavyBranches,
  "ABOVE_HEAVY_BRANCHES" => TwigPlacementToken::AboveHeavyBranches,
  "BELOW_HEAVY_BRANCHES" => TwigPlacementToken::BelowHeavyBranches,
  "SIDE_TRUNK" => TwigPlacementToken::SideTrunk,
  "ABOVE_TRUNK" => TwigPlacementToken::AboveTrunk,
  "BELOW_TRUNK" => TwigPlacementToken::BelowTrunk,
};
