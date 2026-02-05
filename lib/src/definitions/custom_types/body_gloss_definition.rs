use crate::custom_types::Name;

/// Body gloss is defined to give different names for different parts of the body.
///
/// e.g. `[BODYGLOSS:PAW:foot:paw:feet:paws]` becomes
/// `BodyGlossDefintion {
///     identifier: "PAW",
///     source_name: {Singular: "foot", Plural: "feet"},
///     target_name: {Singular: "paw", Plural: "paws"}
/// }`
pub struct BodyGlossDefinition {
    /// The identifier for this body gloss
    pub identifier: String,
    /// The name that should be replaced
    pub source_name: Name,
    /// The name to replace the `source_name` with
    pub target_name: Name,
}
