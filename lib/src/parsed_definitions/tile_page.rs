//! A module for the `TilePage` object.

use std::path::PathBuf;

use tracing::warn;

use crate::{
    dimensions::Dimensions,
    metadata::{ObjectType, RawMetadata},
    raw_definitions::TILE_PAGE_TOKENS,
    tags::TilePageTag,
    traits::{RawObject, Searchable},
    utilities::{build_object_id_from_pieces, clean_search_vec},
};

/// A struct representing a `TilePage` object.
#[allow(clippy::module_name_repetitions)]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct TilePage {
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RawMetadata>,
    identifier: String,
    object_id: String,

    file: PathBuf,
    tile_dim: Dimensions,
    page_dim: Dimensions,
}

impl TilePage {
    #[must_use]
    pub fn get_file_path(&self) -> PathBuf {
        self.file.clone()
    }
    #[must_use]
    pub fn get_tile_dimensions(&self) -> Dimensions {
        self.tile_dim
    }
    #[must_use]
    pub fn get_page_dimensions(&self) -> Dimensions {
        self.page_dim
    }
    /// Function to create a new empty `TilePage`.
    ///
    /// # Returns
    ///
    /// * `TilePage` - The new empty `TilePage`.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            metadata: Some(
                RawMetadata::default()
                    .with_object_type(ObjectType::TilePage)
                    .with_hidden(true),
            ),
            ..Self::default()
        }
    }
    /// Function to create a new `TilePage`.
    ///
    /// # Parameters
    ///
    /// * `identifier` - The identifier for the `TilePage`.
    /// * `metadata` - The metadata for the `TilePage`.
    ///
    /// # Returns
    ///
    /// * `TilePage` - The new `TilePage`.
    #[must_use]
    pub fn new(identifier: &str, metadata: &RawMetadata) -> Self {
        Self {
            identifier: String::from(identifier),
            metadata: Some(metadata.clone()),
            object_id: build_object_id_from_pieces(metadata, identifier, &ObjectType::TilePage),
            ..Self::default()
        }
    }
    /// Function to "clean" the creature. This is used to remove any empty list or strings,
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
    /// * `TilePage` - The cleaned `TilePage`.
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
impl RawObject for TilePage {
    fn get_metadata(&self) -> RawMetadata {
        self.metadata.as_ref().map_or_else(
            || {
                warn!("Metadata is missing for TilePage {}", self.get_object_id());
                RawMetadata::default()
                    .with_object_type(ObjectType::TilePage)
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
    fn is_empty(&self) -> bool {
        self.identifier.is_empty()
    }
    fn get_type(&self) -> &ObjectType {
        &ObjectType::TilePage
    }
    fn clean_self(&mut self) {
        *self = self.cleaned();
    }

    fn parse_tag(&mut self, key: &str, value: &str) {
        match TILE_PAGE_TOKENS.get(key).unwrap_or(&TilePageTag::Unknown) {
            TilePageTag::File => {
                let relative_path: PathBuf = value.split('/').collect();
                let mut raw_path = PathBuf::new();
                if let Some(metadata) = &self.metadata {
                    raw_path = PathBuf::from(metadata.get_raw_file_path());
                }
                self.file = raw_path.parent().unwrap_or(&raw_path).join(relative_path);
            }
            TilePageTag::TileDim => {
                self.tile_dim = Dimensions::from_token(value);
            }
            TilePageTag::PageDim => {
                self.page_dim = Dimensions::from_token(value);
            }
            TilePageTag::Unknown => {
                warn!(
                    "Failed to parse {} as TilePageTag for {}",
                    key,
                    self.get_object_id()
                );
            }
        }
    }
    fn get_searchable_tokens(&self) -> Vec<&str> {
        Vec::new()
    }
    fn get_object_id(&self) -> &str {
        &self.object_id
    }
}

impl Searchable for TilePage {
    fn get_search_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();

        vec.push(self.get_identifier().to_string());
        vec.push(format!("{:?}", self.get_type()));
        vec.push("tilePage".to_string());

        clean_search_vec(vec.as_slice())
    }
}
