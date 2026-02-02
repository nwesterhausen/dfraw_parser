use tracing::warn;
use uuid::Uuid;

use crate::{
    Entity, Position,
    metadata::RawMetadata,
    raw_definitions::POSITION_TOKENS,
    tokens::{EntityToken, ObjectType},
    traits::{RawObject, TagOperations as _},
};

#[typetag::serde]
impl RawObject for Entity {
    fn get_object_id(&self) -> Uuid {
        self.object_id
    }
    fn get_metadata(&self) -> RawMetadata {
        self.metadata.clone()
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
    fn get_name(&self) -> &str {
        self.get_identifier()
    }
    fn get_type(&self) -> ObjectType {
        ObjectType::Entity
    }
    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn parse_tag(&mut self, key: &str, value: &str) {
        if POSITION_TOKENS.get(key).is_some() {
            // Tags should be attached to the last Position in the list
            if let Some(position) = self.positions.last_mut() {
                position.parse_tag(key, value);
                return;
            }
            // If there is no position, create one with unknown name..
            let mut position = Position::new("unknown".into());
            position.parse_tag(key, value);
            self.positions.push(position);
            return;
        }

        let Some(token) = EntityToken::parse(key, value) else {
            warn!("EntityToken::parse failed to parse {key}:{value}");
            return;
        };

        self.tokens.push(token);
    }
}
