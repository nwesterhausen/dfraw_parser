use crate::raw_definitions::INORGANIC_TOKENS;
use crate::tokens::InorganicToken;
use crate::traits::RawToken;
use std::collections::HashMap;
use std::mem::{Discriminant, discriminant};
use std::sync::OnceLock;

/// Utility impl to provide a reverse lookup from `EntityTag` enum variants back to
/// their original token string (if one exists).
///
/// This mirrors the pattern used by other tag lookup utilities: a lazily-initialized
/// static `HashMap` keyed by the enum `Discriminant` is populated from the
/// existing PHF token map and cached in a `OnceLock` for fast subsequent lookups.
impl RawToken for InorganicToken {
    fn get_key(&self) -> Option<&'static str> {
        static REVERSE_MAP: OnceLock<HashMap<Discriminant<InorganicToken>, &'static str>> =
            OnceLock::new();

        let map = REVERSE_MAP.get_or_init(|| {
            let mut m = HashMap::new();
            for (key, tag_template) in &INORGANIC_TOKENS {
                m.insert(discriminant(tag_template), *key);
            }
            m
        });

        map.get(&discriminant(self)).copied()
    }
}
