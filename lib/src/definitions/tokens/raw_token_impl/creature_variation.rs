use crate::tokens::CreatureVariationToken;
use crate::tokens::raw_definitions::CREATURE_VARIATION_TOKENS;
use crate::traits::RawToken;
use std::collections::HashMap;
use std::mem::{Discriminant, discriminant};
use std::sync::OnceLock;

/// Provides a reverse lookup from `CreatureVariationTag` enum variants back to
/// their original token strings (e.g. `"CV_NEW_TAG"`).
///
/// This mirrors the pattern used by other tag lookup utilities: populate a
/// lazily-initialized `HashMap` keyed by the enum `Discriminant` and cache it in
/// a `OnceLock` for fast subsequent lookups.
impl RawToken for CreatureVariationToken {
    fn get_key(&self) -> Option<&'static str> {
        static REVERSE_MAP: OnceLock<HashMap<Discriminant<CreatureVariationToken>, &'static str>> =
            OnceLock::new();

        let map = REVERSE_MAP.get_or_init(|| {
            let mut m = HashMap::new();
            // Populate from the existing PHF map of token -> enum variant
            for (key, tag_template) in &CREATURE_VARIATION_TOKENS {
                m.insert(discriminant(tag_template), *key);
            }
            m
        });

        map.get(&discriminant(self)).copied()
    }
}
