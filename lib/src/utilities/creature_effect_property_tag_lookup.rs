use crate::raw_definitions::tokens::CREATURE_EFFECT_PROPERTY_TOKENS;
use crate::tokens::CreatureEffectPropertyToken;
use std::collections::HashMap;
use std::mem::{Discriminant, discriminant};
use std::sync::OnceLock;

/// Utility impl to provide a reverse lookup from `CreatureEffectPropertyTag` enum
/// variants back to their original token string (e.g. `"SEV"`, `"PROB"`).
///
/// This mirrors the pattern used for other tag lookups: a lazily-initialized
/// static `HashMap` keyed by the enum `Discriminant` is populated from the
/// existing PHF token map and cached in a `OnceLock` for fast subsequent
/// lookups.
impl CreatureEffectPropertyToken {
    /// Returns the original token string for this variant, if available.
    ///
    /// Example: `CreatureEffectPropertyTag::Severity.get_key()` -> `Some("SEV")`
    pub fn get_key(&self) -> Option<&'static str> {
        static REVERSE_MAP: OnceLock<
            HashMap<Discriminant<CreatureEffectPropertyToken>, &'static str>,
        > = OnceLock::new();

        let map = REVERSE_MAP.get_or_init(|| {
            let mut m = HashMap::new();
            for (key, tag_template) in &CREATURE_EFFECT_PROPERTY_TOKENS {
                m.insert(discriminant(tag_template), *key);
            }
            m
        });

        map.get(&discriminant(self)).copied()
    }
}
