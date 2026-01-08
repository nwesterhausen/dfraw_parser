//! Syndrome tags.

/// Represents the tokens that can be used in a syndrome definition.
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
pub enum SyndromeTag {
    /// Used to specify the name of the syndrome as it appears in-game. Names don't have to be unique;
    /// It's perfectly acceptable to have multiple syndromes with identical names.
    Name,
    /// Can be included to create a syndrome class and assign the syndrome to it, for use with the `IT_CANNOT_HAVE_SYNDROME_CLASS`
    /// interaction token. Can be specified more than once to assign the syndrome to multiple classes.
    ///
    /// Other syndromes can also be assigned to the same class.
    Class,
    /// If the syndrome is tied to a material, the injection of this material into a creature's bloodstream will cause it to contract
    /// the syndrome if this token is included. Injection can be carried out as part of a creature attack via `SPECIALATTACK_INJECT_EXTRACT`,
    /// or by piercing the flesh of a creature with an item that has been contaminated with the material. Thus, this token can be used as a
    /// more specific alternative to `SYN_CONTACT` for syndromes intended to be administered by envenomed weapons.
    Injected,
    /// If the syndrome is tied to a material, creatures who come into contact with this material will contract the syndrome
    /// if this token is included in the syndrome definition. Methods of getting a material contaminant onto a creature's body include:
    /// - secretions
    /// - liquid projectiles
    /// - vapor and dust clouds
    /// - puddles and dust piles
    /// - freakish rain
    /// - unprotected contact with an infected creature (punching/wrestling/colliding)
    /// - equipped or hauled items melting
    /// - being struck with a contaminated item
    Contact,
    ///If the syndrome is tied to a material, creatures who inhale the material will contract the syndrome if this token is included.
    /// Materials can only be inhaled in their gaseous state, which is attainable by boiling, or in the form of a `TRAILING_GAS_FLOW`,
    /// `UNDIRECTED_GAS` or `WEATHER_CREEPING_GAS`. Creatures can also be made to leak gaseous tissue when damaged.
    Inhaled,
    /// If the syndrome is tied to a material, creatures who eat or drink substances comprising, containing or contaminated with this
    /// material will contract the syndrome if this token is included. This includes prepared meals when any of the constituent
    /// ingredients contains the material in question.
    ///
    /// This also applies to grazing creatures which happen to munch on a grass that has an ingestion-triggered syndrome tied to any of
    /// its constituent materials.
    Ingested,
    /// If this is included, only creatures which belong to the specified creature class (as well as creatures which pass the
    /// `SYN_AFFECTED_CREATURE` check if this is included) will be able to contract the syndrome. This token can be specified multiple times
    /// per syndrome, in which case creatures which have at least one matching class will be considered susceptible.
    ///
    /// If `SYN_IMMUNE_CLASS` and/or `SYN_IMMUNE_CREATURE` are included, creatures which fail these checks will be unable to contract the syndrome
    /// even if they pass this class check.
    AffectedClass,
    /// If this is included, creatures which belong to the specified creature class will be unable to contract the syndrome. This token can be
    /// specified multiple times per syndrome, in which case creatures with at least one matching class will be considered immune
    /// (unless overridden by `SYN_AFFECTED_CREATURE`).
    ImmuneClass,
    /// If this is included, only the specified creature (and, if `SYN_AFFECTED_CLASS` is included, also creatures which pass this check as explained above)
    /// will be able to contract the syndrome. This token can be used multiple times per syndrome. If used alongside `SYN_IMMUNE_CLASS`, the specified
    /// creature will be able to contract the syndrome regardless of this class check.
    ///
    /// `DWARF:FEMALE` is an example of a valid `creature:caste` combination.
    ///
    /// `ALL` can be used in place of a specific caste so as to indicate that this applies to all castes of the specified creature.
    AffectedCreature,
    /// If this is included, the specified creature will be unable to contract the syndrome (even if it matches `SYN_AFFECTED_CLASS`).
    /// It can be specified multiple times per syndrome. As above, `ALL` can be used in place of a specific caste.
    ImmuneCreature,
    /// Syndrome concentration is essentially a quantity which impacts the severity of the syndrome's relevant effects. The higher the syndrome's concentration,
    /// the greater its severity. When a syndrome is contracted, the value specified in `amount` is its initial concentration level.
    ///
    /// As described above, if a creature is exposed to a syndrome with a particular `SYN_IDENTIFIER` when already possessing an active syndrome with the same identifier,
    /// then this later syndrome isn't contracted, instead contributing to the original syndrome's concentration as indicated by its `SYN_CONCENTRATION_ADDED` token, if present.
    /// The syndrome in question will increase the original syndrome's concentration by `amount` whenever the creature is exposed to it, until its specified `max` concentration
    /// is reached by the original syndrome, causing subsequent exposure to this particular syndrome to do nothing (that is, until the original syndrome ends, at which point
    /// a new one may be contracted normally). Should the creature be exposed to a different syndrome with the same identifier and a higher `max` value, the concentration will
    /// of course increase further.
    ///
    /// Example: `SYN_CONCENTRATION_ADDED:amount:max`
    ConcentrationAdded,
    /// Prevents creatures from being admitted to hospital for problems arising directly as a result of the syndrome's effects, no matter how bad they get.
    NoHospital,
    /// This token can be included to give a syndrome an identifier which can be shared between multiple syndromes. Only one identifier may be specified per syndrome.
    ///
    /// Syndrome identifiers can be used in conjunction with the `SYNDROME_DILUTION_FACTOR` creature token to alter a creature’s innate resistance to the relevant
    /// effects of any syndromes that possess the specified identifier. For example, every alcoholic beverage in unmodded games comes with its own copy of an intoxicating
    /// syndrome, each of which has a `[SYN_IDENTIFIER:INEBRIATION]` token. All dwarves have `[SYNDROME_DILUTION_FACTOR:INEBRIATION:150]`, which decreases the severity of
    /// any effects derived from a syndrome with the INEBRIATION identifier, thus enabling them to better handle all forms of alcohol.
    Identifier,
    /// Unknown as default.
    #[default]
    Unknown,
}

impl std::fmt::Display for SyndromeTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
