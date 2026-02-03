//! String token to parsed tag map for creature effect property tokens.

use crate::tokens::CreatureEffectPropertyToken;

/// A map of creature effect properties to their strings.
pub static CREATURE_EFFECT_PROPERTY_TOKENS: phf::Map<&'static str, CreatureEffectPropertyToken> = phf::phf_map! {
    "SEV" => CreatureEffectPropertyToken::Severity,
    "PROB" => CreatureEffectPropertyToken::Probability,
    "RESISTABLE" => CreatureEffectPropertyToken::Resistible,
    "RESISTIBLE" => CreatureEffectPropertyToken::Resistible,
    "SIZE_DILUTES" => CreatureEffectPropertyToken::SizeDilutes,
    "SIZE_DELAYS" => CreatureEffectPropertyToken::SizeDelays,

    "LOCALIZED" => CreatureEffectPropertyToken::Localized,
    "VASCULAR_ONLY" => CreatureEffectPropertyToken::VascularOnly,
    "MUSCULAR_ONLY" => CreatureEffectPropertyToken::MuscularOnly,

    "BP" => CreatureEffectPropertyToken::BodyPart,
    "BY_CATEGORY" => CreatureEffectPropertyToken::ByCategory,
    "BY_TYPE" => CreatureEffectPropertyToken::ByType,
    "BY_TOKEN" => CreatureEffectPropertyToken::ByToken,

    "START" => CreatureEffectPropertyToken::Start,
    "PEAK" => CreatureEffectPropertyToken::Peak,
    "END" => CreatureEffectPropertyToken::End,

    "DWF_STRETCH" => CreatureEffectPropertyToken::DwfStretch,

    "ABRUPT" => CreatureEffectPropertyToken::Abrupt,
    "ABRUPT_END" => CreatureEffectPropertyToken::AbruptEnd,
    "ABRUPT_START" => CreatureEffectPropertyToken::AbruptStart,

    "CAN_BE_HIDDEN" => CreatureEffectPropertyToken::CanBeHidden,
    "PROBABILITY" => CreatureEffectPropertyToken::Probability,
};
