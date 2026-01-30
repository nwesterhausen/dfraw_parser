use std::collections::HashMap;
use std::mem::{Discriminant, discriminant};
use std::sync::OnceLock;

use crate::raw_definitions::OBJECT_TOKEN_MAP;
use crate::tokens::ObjectType;

impl ObjectType {
    /// Retrieves the original string token key for this tag (e.g., "PET_VALUE").
    /// Uses a cached reverse-lookup map for O(1) performance.
    pub fn get_key(&self) -> Option<&'static str> {
        // 1. Define a static storage for the reverse map
        static REVERSE_MAP: OnceLock<HashMap<Discriminant<ObjectType>, &'static str>> =
            OnceLock::new();

        // 2. Initialize it lazily (only runs once)
        let map = REVERSE_MAP.get_or_init(|| {
            let mut m = HashMap::new();
            // Iterate the existing PHF map
            for (key, tag_template) in &OBJECT_TOKEN_MAP {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_key_reverse_lookup() {
        // Test 1: Verify a standard type that definitely exists in the map
        let creature_key = ObjectType::Creature.get_key();
        assert_eq!(
            creature_key,
            Some("CREATURE"),
            "Should correctly map ObjectType::Creature to 'CREATURE'"
        );

        // Test 2: Verify another standard type
        let plant_key = ObjectType::Plant.get_key();
        assert_eq!(
            plant_key,
            Some("PLANT"),
            "Should correctly map ObjectType::Plant to 'PLANT'"
        );
    }

    #[test]
    fn test_get_key_missing_types() {
        // Test 3: Verify that internal types (which likely aren't in the token map) return None
        // 'Unknown' is usually 99 and not in the PHF map.
        let unknown_key = ObjectType::Unknown.get_key();
        assert_eq!(
            unknown_key, None,
            "Internal types like Unknown should return None"
        );
    }

    #[test]
    fn test_caching_behavior() {
        // Test 4: Ensure calling it multiple times works (verifies OnceLock logic)
        let key1 = ObjectType::Inorganic.get_key();
        let key2 = ObjectType::Inorganic.get_key();

        assert_eq!(key1, key2);
        assert_eq!(key1, Some("INORGANIC"));
    }
}
