//! Material template definition

use dfraw_parser_proc_macros::{Cleanable, IsEmpty};
use uuid::Uuid;

use crate::{
    Material, metadata::RawMetadata, tokens::ObjectType,
    utilities::generate_object_id_using_raw_metadata,
};

/// A struct representing a material template
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
pub struct MaterialTemplate {
    pub identifier: String,
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
}

impl MaterialTemplate {
    /// Create a new empty material template
    ///
    /// # Returns
    ///
    /// A new empty material template
    #[must_use]
    pub fn empty() -> Self {
        Self {
            metadata: RawMetadata::default()
                .with_object_type(ObjectType::MaterialTemplate)
                .with_hidden(true),
            ..Self::default()
        }
    }
    /// Create a new material template
    ///
    /// # Arguments
    ///
    /// * `identifier`: The identifier of the material template
    /// * `metadata`: The metadata of the material template
    ///
    /// # Returns
    ///
    /// A new material template
    #[must_use]
    pub fn new(identifier: &str, metadata: &RawMetadata) -> Self {
        Self {
            identifier: String::from(identifier),
            metadata: metadata.clone(),
            object_id: generate_object_id_using_raw_metadata(
                identifier,
                ObjectType::MaterialTemplate,
                metadata,
            ),
            ..Self::default()
        }
    }
}
