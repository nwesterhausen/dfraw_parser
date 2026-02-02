//! A module for the creature variation definition.

use dfraw_parser_proc_macros::{Cleanable, IsEmpty};
use uuid::Uuid;

use crate::{
    metadata::RawMetadata,
    tokens::{CreatureVariationRuleToken, CreatureVariationToken, ObjectType},
    utilities::generate_object_id_using_raw_metadata,
};

/// A creature variation.
#[allow(clippy::module_name_repetitions)]
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    Default,
    specta::Type,
    PartialEq,
    Eq,
    IsEmpty,
    Cleanable,
)]
#[serde(rename_all = "camelCase")]
pub struct CreatureVariation {
    /// Common Raw file Things
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub metadata: RawMetadata,
    pub identifier: String,
    /// A generated id that is used to uniquely identify this object.
    ///
    /// This is deterministic based on the following:
    /// * The raw's `identifier`
    /// * The raw's [`ObjectType`]
    /// * [`RawModuleLocation`] where the raw was found
    /// * The containing module's `numeric_version`
    ///
    /// See [`crate::utilities::generate_object_id`]
    pub object_id: Uuid,

    /// Creature variations are basically just a set of simple tag actions which are applied to
    /// the creature which is being modified. The tags are applied in order EXCEPT for the convert
    /// tags which are applied in a reverse order.
    pub rules: Vec<CreatureVariationRuleToken>,

    /// The raw tags making up the creature variation
    pub tags: Vec<(CreatureVariationToken, String)>,

    /// A creature variation can define any number of arguments which can be used in the rules.
    /// These arguments replace instances of `!ARGn` in the rules. Use `apply_arguments` to apply
    /// a set of arguments to a creature variation (and get a very specific variation back). Use
    /// `apply_to_creature` to apply the variation to a creature (it also takes arguments and will
    /// apply them to the variation before applying the variation to the creature).
    pub argument_count: usize,
}

impl CreatureVariation {
    /// Create a new creature variation with the given identifier.
    ///
    /// # Arguments
    ///
    /// * `metadata` - The metadata for the creature variation.
    /// * `identifier` - The identifier for the creature variation.
    ///
    /// # Returns
    ///
    /// A new creature variation with the given identifier.
    #[must_use]
    pub fn new(identifier: &str, metadata: &RawMetadata) -> Self {
        Self {
            metadata: metadata.clone(),
            identifier: identifier.to_string(),
            object_id: generate_object_id_using_raw_metadata(
                identifier,
                ObjectType::CreatureVariation,
                metadata,
            ),
            rules: Vec::new(),
            tags: Vec::new(),
            argument_count: 0,
        }
    }
    /// Whether the creature variation is empty.
    ///
    /// # Returns
    ///
    /// `true` if the creature variation is empty, `false` otherwise.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            metadata: RawMetadata::default()
                .with_object_type(ObjectType::CreatureVariation)
                .with_hidden(true),
            identifier: String::new(),
            object_id: Uuid::nil(),
            rules: Vec::new(),
            tags: Vec::new(),
            argument_count: 0,
        }
    }
    /// Get the rules for the creature variation.
    ///
    /// # Returns
    ///
    /// `&Vec<Rule>` - The rules for the creature variation.
    #[must_use]
    pub const fn get_rules(&self) -> &Vec<CreatureVariationRuleToken> {
        &self.rules
    }
    /// Get the conversion rules for the creature variation.
    ///
    /// # Returns
    ///
    /// `Vec<&Rule>` - The conversion rules for the creature variation.
    #[must_use]
    pub fn get_convert_rules(&self) -> Vec<&CreatureVariationRuleToken> {
        self.rules
            .iter()
            .filter(|r| {
                matches!(
                    r,
                    CreatureVariationRuleToken::ConvertTag { .. }
                        | CreatureVariationRuleToken::ConditionalConvertTag { .. }
                )
            })
            .collect()
    }

    #[must_use]
    pub fn get_tags(&self) -> Vec<(CreatureVariationToken, String)> {
        self.tags.clone()
    }
}
