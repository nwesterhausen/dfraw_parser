use crate::tokens::SteamWorkshopToken;

/// A map of the steam workshop raw module info tokens to their enum variant
pub static STEAM_WORKSHOP_TOKENS: phf::Map<&'static str, SteamWorkshopToken> = phf::phf_map! {
    "STEAM_TITLE" => SteamWorkshopToken::Title { title: String::new() },
    "STEAM_DESCRIPTION" => SteamWorkshopToken::Description { description: String::new() },
    "STEAM_TAG" => SteamWorkshopToken::Tag { tag: String::new()},
    "STEAM_KEY_VALUE_TAG" => SteamWorkshopToken::KeyValueTag {key: String::new(), value: String::new() },
    "STEAM_METADATA" => SteamWorkshopToken::Metadata { metadata: String::new()},
    "STEAM_CHANGELOG" => SteamWorkshopToken::Changelog {changes: String::new()},
    "STEAM_FILE_ID" => SteamWorkshopToken::FileId { id: 0 },
};
