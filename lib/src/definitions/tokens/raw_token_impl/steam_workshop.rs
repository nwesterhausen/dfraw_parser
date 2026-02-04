use crate::tokens::SteamWorkshopToken;
use crate::tokens::raw_definitions::STEAM_WORKSHOP_TOKENS;
use crate::traits::RawToken;
use std::collections::HashMap;
use std::mem::{Discriminant, discriminant};
use std::sync::OnceLock;

impl RawToken for SteamWorkshopToken {
    fn get_key(&self) -> Option<&'static str> {
        // Lazily-initialized static reverse map: Discriminant<BiomeTag> -> &'static str
        static REVERSE_MAP: OnceLock<HashMap<Discriminant<SteamWorkshopToken>, &'static str>> =
            OnceLock::new();

        let map = REVERSE_MAP.get_or_init(|| {
            let mut m = HashMap::new();
            // Populate the reverse map from the existing PHF token map
            for (key, tag_template) in &STEAM_WORKSHOP_TOKENS {
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
            SteamWorkshopToken::Title { title } => format!("[{key}:{title}]"),
            SteamWorkshopToken::Description { description } => format!("[{key}:{description}]"),
            SteamWorkshopToken::Tag { tag } => format!("[{key}:{tag}]"),
            SteamWorkshopToken::KeyValueTag {
                key: tag_key,
                value,
            } => format!("[{key}:{tag_key}:{value}]"),
            SteamWorkshopToken::Metadata { metadata: metdata } => format!("[{key}:{metdata}]"),
            SteamWorkshopToken::Changelog { changes } => format!("[{key}:{changes}]"),
            SteamWorkshopToken::FileId { id } => format!("[{key}:{id}]"),
            SteamWorkshopToken::Unknown => String::new(),
        }
    }
}
