//! Material template definition

use dfraw_parser_proc_macros::{Cleanable, IsEmpty};
use uuid::Uuid;

use crate::{
    Material, metadata::RawMetadata, tokens::ObjectType, traits::RawObject,
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
    identifier: String,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    metadata: Option<RawMetadata>,
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
    material: Material,
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
            metadata: Some(
                RawMetadata::default()
                    .with_object_type(ObjectType::MaterialTemplate)
                    .with_hidden(true),
            ),
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
            metadata: Some(metadata.clone()),
            object_id: generate_object_id_using_raw_metadata(
                identifier,
                ObjectType::MaterialTemplate,
                metadata,
            ),
            ..Self::default()
        }
    }
}

#[typetag::serde]
impl RawObject for MaterialTemplate {
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
    fn get_name(&self) -> &str {
        &self.identifier
    }

    fn get_metadata(&self) -> RawMetadata {
        self.metadata.as_ref().map_or_else(
            || {
                tracing::warn!(
                    "Metadata is missing for MaterialTemplate {}",
                    self.get_object_id()
                );
                RawMetadata::default()
                    .with_object_type(ObjectType::MaterialTemplate)
                    .with_hidden(true)
            },
            std::clone::Clone::clone,
        )
    }

    fn get_object_id(&self) -> Uuid {
        self.object_id
    }

    fn parse_tag(&mut self, key: &str, value: &str) {
        self.material.parse_tag(key, value);
    }
    fn get_type(&self) -> ObjectType {
        ObjectType::MaterialTemplate
    }
    fn get_module_object_id(&self) -> Uuid {
        match &self.metadata {
            Some(meta) => meta.get_module_object_id(),
            None => Uuid::nil(),
        }
    }
}
