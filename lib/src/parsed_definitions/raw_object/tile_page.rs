use std::path::PathBuf;

use tracing::warn;
use uuid::Uuid;

use crate::{
    TilePage,
    custom_types::Dimensions,
    metadata::RawMetadata,
    raw_definitions::TILE_PAGE_TOKENS,
    tokens::{ObjectType, TilePageToken},
    traits::RawObject,
};

#[typetag::serde]
impl RawObject for TilePage {
    fn get_metadata(&self) -> RawMetadata {
        self.metadata.clone()
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
    fn get_type(&self) -> ObjectType {
        ObjectType::TilePage
    }
    fn parse_tag(&mut self, key: &str, value: &str) {
        match TILE_PAGE_TOKENS.get(key).unwrap_or(&TilePageToken::Unknown) {
            TilePageToken::File => {
                let relative_path: PathBuf = value.split('/').collect();
                let raw_path = PathBuf::from(self.metadata.get_raw_file_path());
                self.file = raw_path.parent().unwrap_or(&raw_path).join(relative_path);
            }
            TilePageToken::TileDim => {
                self.tile_dim = Dimensions::from_token(value);
            }
            TilePageToken::PageDim => {
                self.page_dim = Dimensions::from_token(value);
            }
            TilePageToken::Unknown => {
                warn!(
                    "Failed to parse {} as TilePageTag for {}",
                    key,
                    self.get_object_id()
                );
            }
        }
    }
    fn get_object_id(&self) -> Uuid {
        self.object_id
    }
    fn get_name(&self) -> &str {
        &self.identifier
    }
}
