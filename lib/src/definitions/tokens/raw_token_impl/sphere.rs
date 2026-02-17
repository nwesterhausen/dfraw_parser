use crate::tokens::SphereToken;
use crate::tokens::raw_definitions::SPHERE_TOKENS;
use crate::traits::RawToken;
use std::collections::HashMap;
use std::mem::{Discriminant, discriminant};
use std::sync::OnceLock;

impl RawToken for SphereToken {
    fn get_key(&self) -> Option<&'static str> {
        // Lazily-initialized static reverse map: Discriminant<SphereToken> -> &'static str
        static REVERSE_MAP: OnceLock<HashMap<Discriminant<SphereToken>, &'static str>> =
            OnceLock::new();

        let map = REVERSE_MAP.get_or_init(|| {
            let mut m = HashMap::new();
            // Populate the reverse map from the existing PHF token map
            for (key, tag_template) in &SPHERE_TOKENS {
                m.insert(discriminant(tag_template), *key);
            }
            m
        });

        // Lookup the token string by this enum variant's discriminant
        map.get(&discriminant(self)).copied()
    }
}
