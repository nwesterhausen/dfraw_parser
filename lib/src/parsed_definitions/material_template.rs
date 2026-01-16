//! Material template definition

use crate::{
    material::Material,
    metadata::{ObjectType, RawMetadata},
    traits::{RawObject, Searchable},
    utilities::{build_object_id_from_pieces, clean_search_vec},
};

/// A struct representing a material template
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct MaterialTemplate {
    identifier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RawMetadata>,
    object_id: String,
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
            object_id: build_object_id_from_pieces(
                metadata,
                identifier,
                &ObjectType::MaterialTemplate,
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
    /// A new material template with all empty or default values removed.
    #[must_use]
    pub fn cleaned(&self) -> Self {
        let mut cleaned = self.clone();

        if let Some(metadata) = &cleaned.metadata
            && metadata.is_hidden()
        {
            cleaned.metadata = None;
        }

        cleaned
    }
}

#[typetag::serde]
impl RawObject for MaterialTemplate {
    fn get_searchable_tokens(&self) -> Vec<&str> {
        Vec::new()
    }
    fn is_empty(&self) -> bool {
        self.identifier.is_empty()
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

    fn get_object_id(&self) -> &str {
        &self.object_id
    }

    fn parse_tag(&mut self, key: &str, value: &str) {
        self.material.parse_tag(key, value);
    }
    fn get_type(&self) -> &ObjectType {
        &ObjectType::MaterialTemplate
    }
    fn clean_self(&mut self) {
        *self = self.cleaned();
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
