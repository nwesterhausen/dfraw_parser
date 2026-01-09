use crate::Creature;
use crate::raw_definitions::tokens::creature::CREATURE_TOKENS;
use crate::tags::CreatureTag;
use crate::traits::RawObjectToken;
use std::collections::HashMap;
use std::mem::{Discriminant, discriminant};
use std::sync::OnceLock;

#[typetag::serialize]
impl RawObjectToken<Creature> for CreatureTag {
    fn is_within(&self, object: &Creature) -> bool {
        object.get_tags().contains(self)
    }

    fn get_key(&self) -> Option<&'static str> {
        // Static lazy-initialized reverse map from enum discriminant -> token string
        static REVERSE_MAP: OnceLock<HashMap<Discriminant<CreatureTag>, &'static str>> =
            OnceLock::new();

        let map = REVERSE_MAP.get_or_init(|| {
            let mut m = HashMap::new();
            // Iterate the existing token map and populate reverse lookup
            for (key, tag_template) in &CREATURE_TOKENS {
                m.insert(discriminant(tag_template), *key);
            }
            m
        });

        // Lookup the token string by this enum variant's discriminant
        map.get(&discriminant(self)).copied()
    }
}
