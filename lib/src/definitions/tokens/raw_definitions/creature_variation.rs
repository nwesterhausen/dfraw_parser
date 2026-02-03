//! String token to parsed tag map for creature variation tokens.

use crate::tokens::CreatureVariationToken;

/// A map of all the creature variation tags to strings
pub static CREATURE_VARIATION_TOKENS: phf::Map<&'static str, CreatureVariationToken> = phf::phf_map! {
    "CV_NEW_TAG" => CreatureVariationToken::NewTag,
    "CV_ADD_TAG" => CreatureVariationToken::AddTag,
    "CV_REMOVE_TAG" => CreatureVariationToken::RemoveTag,
    "CV_CONVERT_TAG" => CreatureVariationToken::ConvertTag,
    "CVCT_MASTER" => CreatureVariationToken::ConvertTagMaster,
    "CVCT_TARGET" => CreatureVariationToken::ConvertTagTarget,
    "CVCT_REPLACEMENT" => CreatureVariationToken::ConvertTagReplacement,
    "CV_NEW_CTAG" => CreatureVariationToken::ConditionalNewTag,
    "CV_ADD_CTAG" => CreatureVariationToken::ConditionalAddTag,
    "CV_REMOVE_CTAG" => CreatureVariationToken::ConditionalRemoveTag,
    "CV_CONVERT_CTAG" => CreatureVariationToken::ConditionalConvertTag,
};
