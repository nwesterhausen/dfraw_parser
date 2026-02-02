//! Parsed Inorganic object definition.
use dfraw_parser_proc_macros::{Cleanable, IsEmpty};
use uuid::Uuid;

use crate::{
    Material,
    metadata::RawMetadata,
    tokens::{EnvironmentClassToken, InclusionTypeToken, InorganicToken, ObjectType},
    utilities::generate_object_id_using_raw_metadata,
};

/// The raw representation of an inorganic object.
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
pub struct Inorganic {
    pub identifier: String,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub metadata: RawMetadata,
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
    pub material: Material,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub metal_ore_chance: Option<Vec<(String, u8)>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub thread_metal_chance: Option<Vec<(String, u8)>>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub environment_class: Option<EnvironmentClassToken>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub environment_inclusion_type: Option<InclusionTypeToken>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub environment_inclusion_frequency: Option<u32>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub environment_class_specific: Option<Vec<String>>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub tags: Option<Vec<InorganicToken>>,
}

impl Inorganic {
    /// Create a new empty Inorganic object.
    ///
    /// This is used for creating a new Inorganic object with the metadata set to hidden.
    ///
    /// # Returns
    ///
    /// A new Inorganic object with the metadata set to hidden.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            metadata: RawMetadata::default()
                .with_object_type(ObjectType::Inorganic)
                .with_hidden(true),
            ..Self::default()
        }
    }
    /// Create a new Inorganic object with the given identifier and metadata.
    ///
    /// The `object_id` is generated from the metadata's raw identifier and the identifier.
    ///
    /// # Arguments
    ///
    /// * `identifier` - The identifier for the inorganic object.
    /// * `metadata` - The metadata for the inorganic object.
    ///
    /// # Returns
    ///
    /// A new Inorganic object with the given identifier and metadata.
    #[must_use]
    pub fn new(identifier: &str, metadata: &RawMetadata) -> Self {
        Self {
            identifier: String::from(identifier),
            metadata: metadata.clone(),
            object_id: generate_object_id_using_raw_metadata(
                identifier,
                ObjectType::Inorganic,
                metadata,
            ),
            ..Self::default()
        }
    }
    #[must_use]
    pub fn get_tags(&self) -> Vec<InorganicToken> {
        if self.tags.is_none() {
            return Vec::new();
        }

        let mut ret_tags = Vec::new();
        if let Some(tags) = &self.tags {
            for tag in tags {
                ret_tags.push(*tag);
            }
        }
        ret_tags
    }

    /// Add a tag to the inorganic raw.
    ///
    /// This handles making sure the tags vector is initialized.
    ///
    /// # Arguments
    ///
    /// * `tag` - The tag to add to the inorganic raw.
    pub fn add_tag(&mut self, tag: InorganicToken) {
        if self.tags.is_none() {
            self.tags = Some(Vec::new());
        }
        if let Some(tags) = self.tags.as_mut() {
            tags.push(tag);
        } else {
            tracing::warn!(
                "Inorganic::add_tag: ({}) Failed to add tag {:?}",
                self.identifier,
                tag
            );
        }
    }

    /// Check whether the inorganic has the specified inorganic tag (found in the `tags` field).
    ///
    /// # Arguments
    ///
    /// * `tag`: The tag to check for.
    ///
    /// # Returns
    ///
    /// Returns true if the inorganic has the specified tag, and false otherwise.
    #[must_use]
    pub fn has_tag(&self, tag: &InorganicToken) -> bool {
        if let Some(tags) = &self.tags {
            for t in tags {
                if std::mem::discriminant(t) == std::mem::discriminant(tag) {
                    return true;
                }
            }
        }
        false
    }
}
