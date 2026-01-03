//! Parsed `SelectCreature` definition

use crate::{
    metadata::{ObjectType, RawMetadata},
    traits::{searchable::clean_search_vec, RawObject, Searchable},
    utilities::build_object_id_from_pieces,
};

/// A struct representing a creature selection
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SelectCreature {
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RawMetadata>,
    identifier: String,
    object_id: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
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
            object_id: build_object_id_from_pieces(
                metadata,
                identifier,
                &ObjectType::SelectCreature,
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

    /// Function to "clean" the raw. This is used to remove any empty list or strings,
    /// and to remove any default values. By "removing" it means setting the value to None.
    ///
    /// This also will remove the metadata if `is_metadata_hidden` is true.
    ///
    /// Steps for all "Option" fields:
    /// - Set any metadata to None if `is_metadata_hidden` is true.
    /// - Set any empty string to None.
    /// - Set any empty list to None.
    /// - Set any default values to None.
    ///
    /// # Returns
    ///
    /// A cleaned `SelectCreature`
    #[must_use]
    pub fn cleaned(&self) -> Self {
        let mut cleaned = self.clone();

        if let Some(metadata) = &cleaned.metadata {
            if metadata.is_hidden() {
                cleaned.metadata = None;
            }
        }

        cleaned
    }
}

#[typetag::serde]
impl RawObject for SelectCreature {
    fn get_searchable_tokens(&self) -> Vec<&str> {
        Vec::new()
    }
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
    fn clean_self(&mut self) {
        *self = self.cleaned();
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
    fn get_name(&self) -> &str {
        &self.identifier
    }
    fn is_empty(&self) -> bool {
        false
    }
    fn get_type(&self) -> &ObjectType {
        &ObjectType::SelectCreature
    }

    fn parse_tag(&mut self, key: &str, value: &str) {
        self.tags.push(format!("{key}:{value}"));
    }

    fn get_object_id(&self) -> &str {
        self.object_id.as_str()
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
