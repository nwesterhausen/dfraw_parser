//! String token to parsed tag map for plant part tokens.

use crate::tokens::PlantPartToken;

/// Mapping of plant part tokens to strings
pub static PLANT_PART_TOKENS: phf::Map<&'static str, PlantPartToken> = phf::phf_map! {
  "TWIGS" => PlantPartToken::Twigs,
  "BRANCHES" => PlantPartToken::Branches,
  "BRANCHES_AND_TWIGS" => PlantPartToken::BranchesAndTwigs,
  "LIGHT_BRANCHES" => PlantPartToken::Branches,
  "LIGHT_BRANCHES_AND_TWIGS" => PlantPartToken::BranchesAndTwigs,
  "ALL_BRANCHES_AND_TWIGS" => PlantPartToken::AllBranchesAndTwigs,
  "HEAVY_BRANCHES" => PlantPartToken::HeavyBranches,
  "DIRECTED_BRANCHES" => PlantPartToken::HeavyBranches,
  "HEAVY_BRANCHES_AND_TRUNK" => PlantPartToken::HeavyBranchesAndTrunk,
  "DIRECTED_BRANCHES_AND_TRUNK" => PlantPartToken::HeavyBranchesAndTrunk,
  "TRUNK" => PlantPartToken::Trunk,
  "ROOTS" => PlantPartToken::Roots,
  "CAP" => PlantPartToken::Cap,
  "SAPLING" => PlantPartToken::Sapling,
};
