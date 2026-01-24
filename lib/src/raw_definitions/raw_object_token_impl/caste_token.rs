use crate::Creature;
use crate::raw_definitions::tokens::CASTE_TOKENS;
use crate::tokens::CasteToken;
use crate::traits::RawObjectToken;
use std::collections::HashMap;
use std::mem::{Discriminant, discriminant};
use std::sync::OnceLock;

#[typetag::serialize]
impl RawObjectToken<Creature> for CasteToken {
    fn is_within(&self, object: &Creature) -> bool {
        for caste in object.get_castes() {
            if caste.get_tags().contains(self) {
                return true;
            }
        }
        false
    }
    fn get_key(&self) -> Option<&'static str> {
        // 1. Define a static storage for the reverse map
        static REVERSE_MAP: OnceLock<HashMap<Discriminant<CasteToken>, &'static str>> =
            OnceLock::new();

        // 2. Initialize it lazily (only runs once)
        let map = REVERSE_MAP.get_or_init(|| {
            let mut m = HashMap::new();
            // Iterate the existing PHF map
            for (key, tag_template) in &CASTE_TOKENS {
                // Key: The Enum Variant (Discriminant)
                // Value: The String Token (e.g., "FLIER")
                m.insert(discriminant(tag_template), *key);
            }
            m
        });

        // 3. Lookup the key using the discriminant of 'self'
        map.get(&discriminant(self)).copied()
    }
}
