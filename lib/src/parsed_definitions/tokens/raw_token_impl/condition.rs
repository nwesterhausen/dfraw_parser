// dfraw_parser\lib\src\utilities\condition_tag_lookup.rs

use crate::raw_definitions::tokens::CONDITION_TOKENS;
use crate::tokens::ConditionToken;
use crate::traits::RawToken;
use std::collections::HashMap;
use std::mem::{Discriminant, discriminant};
use std::sync::OnceLock;

impl RawToken for ConditionToken {
    fn get_key(&self) -> Option<&'static str> {
        // Lazily initialized static reverse map: Discriminant -> token string
        static REVERSE_MAP: OnceLock<HashMap<Discriminant<ConditionToken>, &'static str>> =
            OnceLock::new();

        let map = REVERSE_MAP.get_or_init(|| {
            let mut m = HashMap::new();
            // Populate from the existing PHF map of token -> enum variant
            for (key, tag_template) in &CONDITION_TOKENS {
                m.insert(discriminant(tag_template), *key);
            }
            m
        });

        map.get(&discriminant(self)).copied()
    }
}
