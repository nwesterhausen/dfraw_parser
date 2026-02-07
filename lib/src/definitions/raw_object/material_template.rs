use uuid::Uuid;

use crate::{MaterialTemplate, metadata::RawMetadata, tokens::ObjectType, traits::RawObject};

#[typetag::serde]
impl RawObject for MaterialTemplate {
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
    fn get_name(&self) -> &str {
        &self.identifier
    }

    fn get_metadata(&self) -> RawMetadata {
        self.metadata.clone()
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
