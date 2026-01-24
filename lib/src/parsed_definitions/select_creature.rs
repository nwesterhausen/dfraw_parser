//! Parsed `SelectCreature` definition
use dfraw_parser_proc_macros::{Cleanable, IsEmpty};
use uuid::Uuid;

use crate::{
    metadata::RawMetadata,
    tokens::ObjectType,
    traits::{RawObject, Searchable},
    utilities::{clean_search_vec, generate_object_id_using_raw_metadata},
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

impl Searchable for SelectCreature {
    fn get_search_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();

        vec.push(self.get_identifier().to_string());
        vec.push(format!("{:?}", self.get_type()));
        vec.push("selectCreature".to_string());

        clean_search_vec(vec.as_slice())
    }
}
