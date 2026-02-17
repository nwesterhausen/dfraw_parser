use crate::tokens::CreatureEffectToken;
use crate::tokens::raw_definitions::CREATURE_EFFECT_TOKENS;
use crate::traits::RawToken;
use std::collections::HashMap;
use std::mem::{Discriminant, discriminant};
use std::sync::OnceLock;

impl RawToken for CreatureEffectToken {
    fn get_key(&self) -> Option<&'static str> {
        // Static lazy-initialized reverse map from enum discriminant -> token string
        static REVERSE_MAP: OnceLock<HashMap<Discriminant<CreatureEffectToken>, &'static str>> =
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
