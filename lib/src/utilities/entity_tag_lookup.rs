use crate::raw_definitions::tokens::ENTITY_TOKENS;
use crate::tags::EntityTag;
use std::collections::HashMap;
use std::mem::{Discriminant, discriminant};
use std::sync::OnceLock;

/// Utility impl to provide a reverse lookup from `EntityTag` enum variants back to
/// their original token string (if one exists).
///
/// This mirrors the pattern used by other tag lookup utilities: a lazily-initialized
/// static `HashMap` keyed by the enum `Discriminant` is populated from the
/// existing PHF token map and cached in a `OnceLock` for fast subsequent lookups.
impl EntityTag {
    /// Returns the original token string for this variant, if available.
    ///
    /// Example:
    /// - `EntityTag::Creature.get_key()` -> `Some("CREATURE")`
    pub fn get_key(&self) -> Option<&'static str> {
        static REVERSE_MAP: OnceLock<HashMap<Discriminant<EntityTag>, &'static str>> =
            OnceLock::new();

        let map = REVERSE_MAP.get_or_init(|| {
            let mut m = HashMap::new();
            for (key, tag_template) in &ENTITY_TOKENS {
                m.insert(discriminant(tag_template), *key);
            }
            m
        });

        map.get(&discriminant(self)).copied()
    }
}
