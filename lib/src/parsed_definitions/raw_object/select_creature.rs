use uuid::Uuid;

use crate::{SelectCreature, metadata::RawMetadata, tokens::ObjectType, traits::RawObject};

#[typetag::serde]
impl RawObject for SelectCreature {
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
        ObjectType::SelectCreature
    }

    fn parse_tag(&mut self, key: &str, value: &str) {
        self.tags.push(format!("{key}:{value}"));
    }

    fn get_object_id(&self) -> Uuid {
        self.object_id
    }
}
