use crate::raw_definitions::tokens::BIOME_TOKENS;
use crate::tokens::BiomeToken;
use std::collections::HashMap;
use std::mem::{Discriminant, discriminant};
use std::sync::OnceLock;

impl BiomeToken {
    /// Retrieves the original string token key for this tag (e.g., "MOUNTAIN").
    /// Uses a cached reverse-lookup map for O(1) performance.
    pub fn get_key(&self) -> Option<&'static str> {
        // Lazily-initialized static reverse map: Discriminant<BiomeTag> -> &'static str
        static REVERSE_MAP: OnceLock<HashMap<Discriminant<BiomeToken>, &'static str>> =
            OnceLock::new();

        let map = REVERSE_MAP.get_or_init(|| {
            let mut m = HashMap::new();
            // Populate the reverse map from the existing PHF token map
            for (key, tag_template) in &BIOME_TOKENS {
                m.insert(discriminant(tag_template), *key);
            }
            m
        });

        // Lookup the token string by this enum variant's discriminant
        map.get(&discriminant(self)).copied()
    }
}
