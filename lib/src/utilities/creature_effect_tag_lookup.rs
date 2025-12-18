use crate::raw_definitions::tokens::creature_effect::CREATURE_EFFECT_TOKENS;
use crate::tags::CreatureEffectTag;
use std::collections::HashMap;
use std::mem::{discriminant, Discriminant};
use std::sync::OnceLock;

impl CreatureEffectTag {
    /// Retrieves the original string token key for this tag (e.g., "CE_PAIN").
    /// Uses a cached reverse-lookup map for O(1) performance.
    pub fn get_key(&self) -> Option<&'static str> {
        // Static lazy-initialized reverse map from enum discriminant -> token string
        static REVERSE_MAP: OnceLock<HashMap<Discriminant<CreatureEffectTag>, &'static str>> =
            OnceLock::new();

        let map = REVERSE_MAP.get_or_init(|| {
            let mut m = HashMap::new();
            // Iterate the existing token map and populate reverse lookup
            for (key, tag_template) in &CREATURE_EFFECT_TOKENS {
                m.insert(discriminant(tag_template), *key);
            }
            m
        });

        // Lookup the token string by this enum variant's discriminant
        map.get(&discriminant(self)).copied()
    }
}
