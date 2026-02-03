use tracing::warn;
use uuid::Uuid;

use crate::{Graphic, metadata::RawMetadata, tokens::ObjectType, traits::RawObject};

#[typetag::serde]
impl RawObject for Graphic {
    fn get_metadata(&self) -> RawMetadata {
        self.metadata.clone()
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
    fn get_name(&self) -> &str {
        &self.identifier
    }
    fn get_type(&self) -> ObjectType {
        ObjectType::Graphics
    }

    fn parse_tag(&mut self, key: &str, value: &str) {
        // Any tags should just be able to be handled by the sprite graphic, but it needs to call the right function
        warn!(
            "Graphics tag attempted parse with wrong method: {}:{} for {}",
            key,
            value,
            self.get_identifier()
        );
    }

    fn get_object_id(&self) -> Uuid {
        self.object_id
    }
}
