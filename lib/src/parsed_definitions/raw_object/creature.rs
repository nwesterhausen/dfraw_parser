use std::collections::HashSet;

use tracing::warn;
use uuid::Uuid;

use crate::{
    Caste, Creature,
    metadata::{NumericToken, RawMetadata},
    raw_definitions::CASTE_TOKENS,
    tokens::{CasteToken, CreatureToken, ObjectType},
    traits::{NumericTokenTransform as _, RawObject, RawToken, TagOperations as _},
};

#[typetag::serde]
impl RawObject for Creature {
    fn get_metadata(&self) -> RawMetadata {
        self.metadata.as_ref().map_or_else(
            || {
                warn!(
                    "Creature::get_metadata: ({}) metadata is None",
                    self.identifier
                );
                RawMetadata::default()
                    .with_object_type(ObjectType::Creature)
                    .with_hidden(true)
            },
            std::clone::Clone::clone,
        )
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
    fn get_type(&self) -> ObjectType {
        ObjectType::Creature
    }
    #[allow(clippy::too_many_lines)]
    fn parse_tag(&mut self, key: &str, value: &str) {
        if CASTE_TOKENS.contains_key(key) {
            if let Some(caste) = self.castes.last_mut() {
                caste.parse_tag(key, value);
                return;
            }
            // Create an unknown caste to parse it instead of missing the token
            let mut caste = Caste::new("unknown");
            caste.parse_tag(key, value);
            self.castes.push(caste);
            return;
        }

        let Some(token) = CreatureToken::parse(key, value) else {
            warn!("CreatureToken::parse: failed to parse {key}:{value}");
            return;
        };

        self.tokens.push(token);
    }
    fn get_object_id(&self) -> Uuid {
        self.object_id
    }
    fn get_name(&self) -> &str {
        self.tokens
            .iter()
            .find_map(|token| match token {
                CreatureToken::Name { name } => Some(name.get_singular()),
                _ => None,
            })
            // If find_map returns None, return the identifier instead
            .unwrap_or(&self.identifier)
    }
    fn get_searchable_tokens(&self) -> Vec<&str> {
        let mut tokens = HashSet::new();

        for token in CreatureToken::FLAG_TOKENS {
            if self.has_tag(token) {
                tokens.insert(RawToken::get_key(token).unwrap_or_default());
            }
        }

        for caste in &self.castes {
            for token in CasteToken::FLAG_TOKENS {
                if caste.has_tag(token) {
                    tokens.insert(RawToken::get_key(token).unwrap_or_default());
                }
            }
        }

        tokens.into_iter().collect()
    }
    fn get_numeric_flags(&self) -> Vec<NumericToken> {
        let mut tokens = Vec::new();

        // 1. Collect from Creature Tags
        for token in &self.tokens {
            tokens.extend(token.as_numeric_tokens());
        }

        // 2. Collect from Caste Tags
        for caste in &self.castes {
            for tag in caste.get_tags() {
                tokens.extend(tag.as_numeric_tokens());
            }
        }

        tokens
    }
    fn get_module_object_id(&self) -> Uuid {
        match &self.metadata {
            Some(meta) => meta.get_module_object_id(),
            None => Uuid::nil(),
        }
    }
}
