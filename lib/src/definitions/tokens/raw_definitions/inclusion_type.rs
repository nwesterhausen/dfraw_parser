//! String token to parsed tag map for inclusion type tokens.

use crate::tokens::InclusionTypeToken;

/// Map of inclusion types to their string representation.
pub static INCLUSION_TYPE_TOKENS: phf::Map<&'static str, InclusionTypeToken> = phf::phf_map! {
  "CLUSTER" => InclusionTypeToken::Cluster,
  "CLUSTER_SMALL" => InclusionTypeToken::ClusterSmall,
  "CLUSTER_ONE" => InclusionTypeToken::ClusterOne,
  "VEIN" => InclusionTypeToken::Vein,
};
