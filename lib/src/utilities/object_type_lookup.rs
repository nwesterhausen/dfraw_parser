use std::collections::HashMap;
use std::mem::{discriminant, Discriminant};
use std::sync::OnceLock;

use crate::metadata::{ObjectType, OBJECT_TOKEN_MAP};

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
