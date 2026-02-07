//! A module for the `TilePage` object.

use std::path::PathBuf;

use dfraw_parser_proc_macros::{Cleanable, IsEmpty};
use uuid::Uuid;

use crate::{
    custom_types::Dimensions, metadata::RawMetadata, tokens::ObjectType,
    utilities::generate_object_id_using_raw_metadata,
};

/// A struct representing a `TilePage` object.
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
pub struct TilePage {
    pub metadata: RawMetadata,
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

    pub file: PathBuf,
    pub tile_dim: Dimensions,
    pub page_dim: Dimensions,
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
            metadata: RawMetadata::default()
                .with_object_type(ObjectType::TilePage)
                .with_hidden(true),
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
            metadata: metadata.clone(),
            object_id: generate_object_id_using_raw_metadata(
                identifier,
                ObjectType::TilePage,
                metadata,
            ),
            ..Self::default()
        }
    }
}
