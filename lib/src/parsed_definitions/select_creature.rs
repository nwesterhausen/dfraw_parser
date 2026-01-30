//! Parsed `SelectCreature` definition
use dfraw_parser_proc_macros::{Cleanable, IsEmpty};
use uuid::Uuid;

use crate::{
    metadata::RawMetadata, tokens::ObjectType, traits::RawObject,
    utilities::generate_object_id_using_raw_metadata,
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
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    metadata: Option<RawMetadata>,
    identifier: String,
    /// A generated id that is used to uniquely identify this object.
    ///
    /// This is deterministic based on the following:
    /// * The raw's `identifier`
    /// * The raw's [`ObjectType`]
    /// * [`RawModuleLocation`] where the raw was found
    /// * The containing module's `numeric_version`
    ///
    /// See [`crate::utilities::generate_object_id`]
    object_id: Uuid,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    tags: Vec<String>,
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
            metadata: Some(metadata.clone()),
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
            metadata: Some(
                RawMetadata::default()
                    .with_object_type(ObjectType::SelectCreature)
                    .with_hidden(true),
            ),
            ..Self::default()
        }
    }
}

#[typetag::serde]
impl RawObject for SelectCreature {
    fn get_metadata(&self) -> RawMetadata {
        self.metadata.as_ref().map_or_else(
            || {
                tracing::warn!(
                    "Metadata is missing for SelectCreature: {}",
                    self.identifier
                );
                RawMetadata::default()
                    .with_object_type(ObjectType::SelectCreature)
                    .with_hidden(true)
            },
            std::clone::Clone::clone,
        )
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
    fn get_name(&self) -> &str {
        &self.identifier
    }
    fn get_type(&self) -> ObjectType {
        ObjectType::SelectCreature
    }

    fn parse_tag(&mut self, key: &str, value: &str) {
        self.tags.push(format!("{key}:{value}"));
    }

    fn get_object_id(&self) -> Uuid {
        self.object_id
    }
    fn get_module_object_id(&self) -> Uuid {
        match &self.metadata {
            Some(meta) => meta.get_module_object_id(),
            None => Uuid::nil(),
        }
    }
}
