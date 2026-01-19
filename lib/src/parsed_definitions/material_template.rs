//! Material template definition

use dfraw_parser_proc_macros::{Cleanable, IsEmpty};
use uuid::Uuid;

use crate::{
    Material,
    metadata::RawMetadata,
    tags::ObjectType,
    traits::{RawObject, Searchable},
    utilities::{clean_search_vec, generate_object_id_using_raw_metadata},
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
    fn get_searchable_tokens(&self) -> Vec<&str> {
        Vec::new()
    }
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
}

impl Searchable for MaterialTemplate {
    fn get_search_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();

        vec.push(self.get_identifier().to_string());
        vec.push(format!("{:?}", self.get_type()));
        vec.extend(self.material.get_search_vec());
        vec.push("template".to_string());

        clean_search_vec(vec.as_slice())
    }
}
