use crate::tokens::ModuleInfoToken;
use crate::tokens::raw_definitions::MODULE_INFO_TOKENS;
use crate::traits::RawToken;
use std::collections::HashMap;
use std::mem::{Discriminant, discriminant};
use std::sync::OnceLock;

impl RawToken for ModuleInfoToken {
    fn get_key(&self) -> Option<&'static str> {
        // Lazily-initialized static reverse map: Discriminant<BiomeTag> -> &'static str
        static REVERSE_MAP: OnceLock<HashMap<Discriminant<ModuleInfoToken>, &'static str>> =
            OnceLock::new();

        let map = REVERSE_MAP.get_or_init(|| {
            let mut m = HashMap::new();
            // Populate the reverse map from the existing PHF token map
            for (key, tag_template) in &MODULE_INFO_TOKENS {
                m.insert(discriminant(tag_template), *key);
            }
            m
        });

        // Lookup the token string by this enum variant's discriminant
        map.get(&discriminant(self)).copied()
    }

    fn to_raw_token(&self) -> String {
        let key = match self.get_key() {
            Some(key) => key,
            None => return String::new(),
        };

        match self {
            ModuleInfoToken::EarliestCompatibleNumericVersion { version }
            | ModuleInfoToken::NumericVersion { version } => format!("[{key}:{version}]"),
            ModuleInfoToken::EarliestCompatibleDisplayedVersion { version }
            | ModuleInfoToken::DisplayedVersion { version } => format!("[{key}:{version}]"),
            ModuleInfoToken::Author { name } | ModuleInfoToken::Name { name } => {
                format!("[{key}:{name}]")
            }
            ModuleInfoToken::Description { description } => format!("[{key}:{description}]"),
            ModuleInfoToken::RequiresId { id }
            | ModuleInfoToken::RequiresIdBeforeMe { id }
            | ModuleInfoToken::RequiresIdAfterMe { id }
            | ModuleInfoToken::ConflictsWithId { id } => format!("[{key}:{id}]"),
            ModuleInfoToken::Unknown => String::new(),
        }
    }
}
