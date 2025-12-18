//! Contains the `CreatureEffectTag` enum and associated functions.

/// An enum representing a creature effect tag.
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
pub enum CreatureEffectTag {
    // Negative Effects
    /// Afflicts the targeted body part with intense pain. If no target is specified this applies to all body parts.
    Pain,
    /// Causes the targeted body part to swell up. Extreme swelling may lead to necrosis.
    Swelling,
    /// Causes pus to ooze from the afflicted body part.
    Oozing,
    /// Causes the targeted body part to undergo bruising.
    Bruising,
    /// Covers the targeted body part with blisters.
    Blisters,
    /// Causes numbness in the affected body part, blocking pain. Extreme numbness may lead to sensory nerve damage.
    /// If no target is specified this applies to all body parts.
    Numbness,
    /// Causes complete paralysis of the affected body part. Paralysis on a limb may lead to motor nerve damage.
    /// If no target is specified this causes total paralysis, which can lead to suffocation of smaller creatures.
    Paralysis,
    /// Causes the Fever condition.
    Fever,
    /// Causes the targeted body part to start bleeding, with heavy enough bleeding resulting in the death of the sufferer.
    /// Some conditions seem to cause bleeding to be fatal no matter how weak.
    Bleeding,
    /// This effect results in the sufferer periodically coughing blood, which stains the tile they're on and requires cleanup.
    /// It doesn't appear to be lethal, but may cause minor bleeding damage.
    CoughingBlood,
    /// This effect results in the sufferer periodically vomiting blood, which stains the tile they're on and requires cleanup.
    /// It doesn't appear to be lethal, but may cause minor bleeding damage.
    VomitingBlood,
    /// Causes the Nausea condition, and heavy vomiting. Can eventually lead to dehydration and death.
    Nausea,
    /// Renders the creature unconscious.
    Unconsciousness,
    /// Causes the targeted body part to rot, with associated tissue damage, miasma emission and bleeding.
    /// The victim slowly bleeds to death if the wound is not treated. Badly necrotic limbs will require amputation.
    Necrosis,
    /// An organ afflicted with this effect is rendered inoperable.
    /// E.g., if both lungs are impaired the creature can't breathe and will suffocate. This token only affects organs, not limbs.
    ImpairFunction,
    /// Causes the Drowsiness condition
    Drowsiness,
    /// Inflicts the Dizziness condition, occasional fainting and a general slowdown in movement and work speed.
    Dizziness,
    // Healing Effects
    /// Decreases the severity of pain produced by wounds or syndrome effects on the targeted body part.
    /// The SEV value probably controls by how much the pain is decreased.
    ReducePain,
    /// Decreases the severity of swelling on the targeted body part.
    ReduceSwelling,
    /// Decreases the severity of any paralysis effects on the targeted body part.
    ReduceParalysis,
    /// Decreases the severity of any dizziness the creature has.
    ReduceDizziness,
    /// Decreases the severity of any nausea the creature has.
    ReduceNausea,
    /// Decreases the severity of any fever the creature has.
    ReduceFever,
    /// Decreases the severity of the bleeding of any wounds or syndrome effects on the targeted body part.
    /// The SEV value probably controls by how much the bleeding is decreased.
    StopBleeding,
    /// Closes any wounds on the targeted body part with speed depending on the SEV value.
    CloseOpenWounds,
    /// Probably decreases the severity of the infection from infected wounds over time.
    CureInfection,
    /// Heals the tissues of the targeted body part with speed depending on the SEV value.
    HealTissues,
    /// Heals the nerves of the targeted body part with speed depending on the SEV value.
    HealNerves,
    /// Causes missing body parts to regrow. SEV controls how quickly body parts are regrown.
    RegrowParts,
    // Special Effects
    /// Add a tag
    AddTag,
    /// Remove a tag
    RemoveTag,
    /// Display name of the effect
    DisplayName,
    /// Display tile of the effect
    DisplayTile,
    /// Whether the tile flashes
    FlashTile,
    /// Physical attribute change
    PhysAttChange,
    /// Mental attribute change
    MentAttChange,
    /// Speed change
    SpeedChange,
    /// Skill roll adjustment
    SkillRollAdjust,
    /// Body appearance modifier
    BodyAppearanceModifier,
    /// Body part appearance modifier
    BodyPartAppearanceModifier,
    /// Body transformation
    BodyTransformation,
    /// Material force multiplier
    MaterialForceMultiplier,
    /// Can do an interaction
    CanDoInteraction,
    /// Can do a special attack interaction
    SpecialAttackInteraction,
    /// Can do a body mat interaction
    BodyMatInteraction,
    /// Can sense creatures of a class
    SenseCreatureClass,
    /// Feel emotion
    FeelEmotion,
    /// Changes the personality of the creature
    ChangePersonality,
    /// Erratic behavior
    ErraticBehavior,
    /// Unknown
    #[default]
    Unknown,
}

//Todo: add triggers
