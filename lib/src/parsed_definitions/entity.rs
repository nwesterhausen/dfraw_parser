//! Contains the Entity struct and implementations.

use dfraw_parser_proc_macros::{Cleanable, IsEmpty};
use uuid::Uuid;

use crate::{
    Position,
    metadata::RawMetadata,
    tokens::{EntityToken, ObjectType},
    utilities::generate_object_id_using_raw_metadata,
};

/// A struct representing an Entity object.
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    Default,
    specta::Type,
    PartialEq,
    IsEmpty,
    Cleanable,
)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    /// The metadata for this [`Entity`]
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub metadata: Option<RawMetadata>,
    /// The identifier and name of the civilizaiton
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
    /// The tokens defining this [`Entity`]
    pub tokens: Vec<EntityToken>,
    /// The positions defined in this [`Entity`]
    pub positions: Vec<Position>,
}

impl Entity {
    /// Function to create a new empty Entity.
    ///
    /// # Returns
    ///
    /// * `Entity` - The new empty Entity.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }
    /// Function to create a new Entity.
    ///
    /// # Parameters
    ///
    /// * `identifier` - The identifier for the Entity.
    /// * `metadata` - The metadata for the Entity.
    ///
    /// # Returns
    ///
    /// * `Entity` - The new Entity.
    #[must_use]
    pub fn new(identifier: &str, metadata: &RawMetadata) -> Self {
        Self {
            identifier: String::from(identifier),
            metadata: Some(metadata.clone()),
            object_id: generate_object_id_using_raw_metadata(
                identifier,
                ObjectType::Entity,
                metadata,
            ),
            ..Default::default()
        }
    }

    pub fn get_tokens(&self) -> Vec<EntityToken> {
        self.tokens.clone()
    }
}
