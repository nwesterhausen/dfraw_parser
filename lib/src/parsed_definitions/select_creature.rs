//! Parsed `SelectCreature` definition
use dfraw_parser_proc_macros::{Cleanable, IsEmpty};
use uuid::Uuid;

use crate::{
    metadata::RawMetadata, tokens::ObjectType, utilities::generate_object_id_using_raw_metadata,
};

/// A struct representing a creature selection
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
pub struct SelectCreature {
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

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub tags: Vec<String>,
}
impl SelectCreature {
    /// Create a new `SelectCreature`
    ///
    /// # Arguments
    ///
    /// * `identifier` - The identifier of the creature
    /// * `metadata` - The metadata of the creature
    ///
    /// # Returns
    ///
    /// A new `SelectCreature`
    #[must_use]
    pub fn new(identifier: &str, metadata: &RawMetadata) -> Self {
        Self {
            identifier: String::from(identifier),
            metadata: metadata.clone(),
            object_id: generate_object_id_using_raw_metadata(
                identifier,
                ObjectType::SelectCreature,
                metadata,
            ),
            ..Self::default()
        }
    }
    /// Create a new empty `SelectCreature`
    ///
    /// # Returns
    ///
    /// A new empty `SelectCreature`
    #[must_use]
    pub fn empty() -> Self {
        Self {
            metadata: RawMetadata::default()
                .with_object_type(ObjectType::SelectCreature)
                .with_hidden(true),
            ..Self::default()
        }
    }
}
