//! An enum representing a creature effect property tag.

use crate::traits::IsEmpty;

/// An enum representing a creature effect property tag.
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
pub enum CreatureEffectPropertyToken {
    /// The severity of the effect. Higher values appear to be worse, with SEV:1000 `CE_NECROSIS` causing a part to near-instantly become rotten.
    Severity,
    /// The probability of the effect actually manifesting in the victim, as a percentage. 100 means always, 1 means a 1 in 100 chance.
    Probability,
    ///(Optional) Determines if the effect can be hindered by the target creature's disease resistance attribute.
    /// Without this token, disease resistance is ignored. (yes, it's spelled incorrectly)
    Resistible,
    /// (Optional) This token presumably causes the severity of the effect to scale with the size of the creature compared
    /// to the size of the dose of contagion they received, but has yet to be extensively tested.
    SizeDilutes,
    /// (Optional) As above, this token has yet to be tested but presumably delays the onset of an effect according to the size of the victim.
    SizeDelays,
    /// (Optional; overrides BP tokens) This tag causes an effect to ignore all BP tokens and then forces the game to attempt to apply the effect to
    /// the limb that came into contact with the contagion - i.e. the part that was bitten by the creature injecting the syndrome material,
    /// or the one that was splattered by a contact contagion. If an effect can not be applied to the contacted limb (such as `IMPAIR_FUNCTION` on a non-organ)
    /// then this token makes the effect do nothing. This token also makes inhaled syndromes have no effect.
    Localized,
    /// (Optional) This effect only affects tissue layers with the VASCULAR token.
    VascularOnly,
    /// (Optional) This effect only affects tissue layers with the MUSCULAR token.
    MuscularOnly,
    /// (Optional; overridden by LOCALIZED) Specifies which body parts and tissues the effect is to be applied to. Not every effect requires a target!
    ///  For example, if you wanted to target the lungs of a creature, you would use `BP:BY_CATEGORY:LUNG:ALL`. The effect would act on all body parts
    /// within the creature with the CATEGORY tag LUNG and affect all tissue layers. For another example, say you wanted to cause the skin to rot off a creature -
    /// you could use `BP:BY_CATEGORY:ALL:SKIN`, targeting the SKIN tissue on all body parts. Multiple targets can be given in one effect by placing the BP tokens end to end.
    /// This is one of the most powerful and useful aspects of the syndrome system, as it allows you to selectively target body parts relevant to the contagion,
    /// like lungs for coal dust inhalation, or the eyes for exposure to an acid gas.
    BodyPart,
    /// `BY_CATEGORY:X` to target body parts with a matching `[CATEGORY:X]` body token (or `ALL` to affect everything)
    ByCategory,
    /// `BY_TYPE:X` to target body parts having a particular type (`UPPERBODY`, `LOWERBODY`, `HEAD`, `GRASP`, or `STANCE`)
    ByType,
    /// `BY_TOKEN:X` to target individual body parts by their ID as specified by the `[BP]` token of the body plan definition.
    ByToken,
    /// Determines the time after exposure, in ticks, when the effect starts. Required for all effects.
    Start,
    /// (Optional) Determines the time after exposure, in ticks, when the effect reaches its peak intensity.
    Peak,
    /// (Optional) Determines the time after exposure, in ticks, when the effect ends.
    End,
    /// (Optional) Multiplies the duration values of the effect by the specified amount in Fortress mode.
    DwfStretch,
    /// (Optional) Makes the effect begin immediately rather than ramping up.
    AbruptStart,
    /// (Optional) Makes the effect end immediately rather than ramping down.
    AbruptEnd,
    /// (Optional) Combination of `ABRUPT_START` and `ABRUPT_END`.
    Abrupt,
    /// (Optional) Can be hidden by a unit assuming a secret identity, such as a vampire.
    CanBeHidden,
    /// Unknown value for default.
    #[default]
    Unknown,
}

impl std::fmt::Display for CreatureEffectPropertyToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl IsEmpty for CreatureEffectPropertyToken {
    fn is_empty(&self) -> bool {
        self == &Self::Unknown
    }
}
