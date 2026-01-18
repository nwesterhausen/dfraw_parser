use chrono::TimeDelta;
use dfraw_parser::metadata::ParserOptions;
use serde::{Deserialize, Serialize};

use crate::db::metadata::AppMetadataKey;

/// A trait that connects a metadata key to its expected Rust type.
pub trait TypedMetadata {
    /// The type this metadata value should deserialize into.
    type Value: Serialize + for<'de> Deserialize<'de>;

    /// Returns the corresponding enum key.
    fn key() -> AppMetadataKey;
}

/// Marker struct for the last insertion timestamp.
pub struct LastRawsInsertion;
impl TypedMetadata for LastRawsInsertion {
    type Value = String; // Store as ISO-8601 string
    fn key() -> AppMetadataKey {
        AppMetadataKey::LastRawsInsertion
    }
}

/// Marker struct for the last insertion timestamp.
pub struct LastRawsParsingOperation;
impl TypedMetadata for LastRawsParsingOperation {
    type Value = String; // Store as ISO-8601 string
    fn key() -> AppMetadataKey {
        AppMetadataKey::LastRawsParsingOperation
    }
}

/// Marker struct for Steam autodetect preference.
pub struct UseSteamAutodetect;
impl TypedMetadata for UseSteamAutodetect {
    type Value = bool;
    fn key() -> AppMetadataKey {
        AppMetadataKey::UseSteamAutodetect
    }
}

/// Marker struct for the recent search terms array.
pub struct RecentSearchTerms;
impl TypedMetadata for RecentSearchTerms {
    type Value = Vec<String>;
    fn key() -> AppMetadataKey {
        AppMetadataKey::RecentSearchTerms
    }
}

/// Marker struct for the last game installation path.
pub struct PreviousDwarfFortressGamePath;
impl TypedMetadata for PreviousDwarfFortressGamePath {
    type Value = String;
    fn key() -> AppMetadataKey {
        AppMetadataKey::PreviousDwarfFortressGamePath
    }
}

/// Marker struct for the last user data path.
pub struct PreviousDwarfFortressUserPath;
impl TypedMetadata for PreviousDwarfFortressUserPath {
    type Value = String;
    fn key() -> AppMetadataKey {
        AppMetadataKey::PreviousDwarfFortressUserPath
    }
}

/// Marker struct for the last parse duration.
pub struct PreviousParseDuration;
impl TypedMetadata for PreviousParseDuration {
    type Value = TimeDelta;
    fn key() -> AppMetadataKey {
        AppMetadataKey::PreviousParseDuration
    }
}

/// Marker struct for the last insertion duration.
pub struct PreviousInsertionDuration;
impl TypedMetadata for PreviousInsertionDuration {
    type Value = TimeDelta;
    fn key() -> AppMetadataKey {
        AppMetadataKey::PreviousInsertionDuration
    }
}

/// Marker struct for the last parse options.
pub struct PreviousParserOptions;
impl TypedMetadata for PreviousParserOptions {
    type Value = ParserOptions;
    fn key() -> AppMetadataKey {
        AppMetadataKey::PreviousParserOptions
    }
}

/// Marker struct for favorited raw identifiers.
pub struct FavoriteRaws;
impl TypedMetadata for FavoriteRaws {
    type Value = Vec<i64>;
    fn key() -> AppMetadataKey {
        AppMetadataKey::FavoriteRaws
    }
}

/// Marker struct for the preferred search results limit.
pub struct PreferredSearchLimit;
impl TypedMetadata for PreferredSearchLimit {
    type Value = u32;
    fn key() -> AppMetadataKey {
        AppMetadataKey::PreferredSearchLimit
    }
}

/// marker struct for the stored app settings (as JSON string)
pub struct StoredSettings;
impl TypedMetadata for StoredSettings {
    type Value = String;
    fn key() -> AppMetadataKey {
        AppMetadataKey::StoredSettings
    }
}
