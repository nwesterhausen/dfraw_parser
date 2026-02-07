use crate::{
    custom_types::{Name, TissueShape},
    tokens::MaterialStateToken,
};

/// Tokens used to define tissue
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    PartialEq,
    Default,
    Eq,
    specta::Type,
    strum_macros::EnumIter,
)]
pub enum TissueToken {
    /// Name of the tissue
    Name { name: Name },
    /// Defines the tissue material.
    Material { material_token: String },
    /// The relative thickness of the tissue. A higher thickness is harder to penetrate, but raising a tissue's relative thickness decreases
    /// the thickness of all other tissues.
    RelativeThickness { thickness: u32 },
    /// Lower is faster. Omitting the token will result in a tissue that never heals.
    HealingRate { rate: u32 },
    /// Related to how much said tissue bleeds. Higher = More bleeding (Which is why the heart has the highest value.)
    Vascular { bleeding_rate: u32 },
    /// Related to how much pain your character will suffer when said tissue is damaged. Higher = More pain when damaged (Which is why the
    /// bone tissue has a much higher value than other tissues. A broken bone hurts a lot more than a flesh cut.)
    PainReceptors { sensitivity: u32 },
    /// The thickness of the tissue increases when character strength increases.
    ThickensOnStrength,
    /// Thickness of said tissue increases when the character eats and doesn't exercise sufficiently.
    ThickensOnEnergyStorage,
    /// The tissue contains arteries. Edged attacks have the chance to break an artery, increasing blood loss.
    Arteries,
    /// Simply, whether or not the tissue will be scarred once healed.
    Scars,
    /// Holds the body part together. A cut or a fracture disables the body part it's in.
    Structural,
    /// Any ligaments or tendons are part of this tissue. Vulnerable to edged attacks, damage disables the limb.
    ConnectiveTissueAnchor,
    /// The tissue will not heal, or heals slower, until it is set by a bone doctor.
    Settable,
    /// The broken tissue can be fixed with a cast or a splint to restore function while it heals.
    Splintable,
    /// The tissue performs some sort of special function (e.g. sight, hearing, breathing, etc.) An organ with such a function will stop
    /// working if a sufficient amount of damage is sustained by its `FUNCTIONAL` tissues. If an organ has no `FUNCTIONAL` tissues,
    /// it will stop working only if it is severed or destroyed entirely by heat or cold.
    Functional,
    /// If a creature has no functioning parts with the `THOUGHT` token, it will be unable to move or breathe.
    /// `NO_THOUGHT_CENTER_FOR_MOVEMENT` bypasses this limitation.
    Thought,
    /// Seems to affect where sensory or motor nerves are located, and whether damage to this tissue will render a limb useless.
    Muscular,
    /// Holds body parts together. A body part will not be severed unless all of its component tissues with
    /// the `CONNECTS` tag are severed.
    Connects,
    /// Causes tissue to sometimes severely bleed when damaged. This is independent of its `VASCULAR` value.
    MajorArteries,
    /// Tissue supplies the creature with heat insulation. Higher values result in more insulation.
    Insulation { value: u32 },
    /// The tissue is purely cosmetic
    Cosmetic,
    /// The tissue can be styled as per a tissue style (defined in an entity entry)
    Styleable,
    /// Specifies how the tissue is shaped
    Shape { shape: TissueShape },
    /// Tissue is implicitly attached to another tissue and will fall off if that tissue layer is destroyed.
    /// Used for hair and feathers, which are subordinate to skin.
    SubordinateToTissue { tissue: String },
    /// Sets/forces a default material state for the selected tissue.
    TissueMaterialState { state: MaterialStateToken },
    /// The selected tissue leaks out of the creature when the layers above it are pierced.
    Leaks,

    /// Used to smell - not used. This token is used in `[OBJECT:BODY]` tokens.
    Smell,
    /// Used to hearing - not used. This token is used in `[OBJECT:BODY]` tokens.
    Hear,
    /// Unknown - not used. Most likely related to flying, see `[FLIER]` in `[OBJECT:BODY]`.
    Flight,
    /// Used to breathing - not used. This token is used in `[OBJECT:BODY]` tokens.
    Breathe,
    /// Used to seeing - not used. This token is used in `[OBJECT:BODY]` tokens.
    Sight,
    /// Nervous function - not used. This token is used in `[OBJECT:BODY]` tokens.
    Nervous,
    /// Unknown token
    #[default]
    Unknown,
}
