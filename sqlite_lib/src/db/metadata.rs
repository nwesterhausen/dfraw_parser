//! Defines the metadata tracked about the database, inside the database.

/// Represents known identity keys for the `app_metadata` table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppMetadataKey {
    /// The ISO-8601 (RFC 3339) timestamp of the last successful raw database sync
    LastRawsInsertion,
    /// Whether to use the Steam-based autodetect to find relevant directories
    UseSteamAutodetect,
    /// Array of the last 10 searches (greedy, so prefers the most complete search; e.g. searched for 'dva','dvar', and 'dvark' will store 'dvark')
    RecentSearchTerms,
    /// The last-used path for the Dwarf Fortress installation directory
    PreviousDwarfFortressGamePath,
    /// The last-used path for the Dwarf Fortress user-data directory
    PreviousDwarfFortressUserPath,
    /// The duration of the last parse
    PreviousParseDuration,
    /// The options used for the last parse
    PreviousParserOptions,
    /// The duration of the last raws insertion to the database
    PreviousInsertionDuration,
    /// Array of raws the user has favorited
    FavoriteRaws,
    /// The user preference for how many results per page
    PreferredSearchLimit,
    /// The ISO-8601 (RFC 3339) timestamp of the last successful raw parsing operation
    LastRawsParsingOperation,
    /// JSON blob with app settings
    StoredSettings,
}

impl AppMetadataKey {
    /// Returns the string key used in the database.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LastRawsInsertion => "last_raws_insertion",
            Self::UseSteamAutodetect => "use_steam_autodetect",
            Self::RecentSearchTerms => "recent_search_terms",
            Self::PreviousDwarfFortressGamePath => "previous_dwarf_fortress_game_path",
            Self::PreviousDwarfFortressUserPath => "previous_dwarf_fortress_user_path",
            Self::PreviousParseDuration => "previous_parse_duration",
            Self::PreviousParserOptions => "previous_parser_options",
            Self::FavoriteRaws => "favorite_raws",
            Self::PreferredSearchLimit => "preferred_search_limit",
            Self::PreviousInsertionDuration => "previous_insertion_duration",
            Self::LastRawsParsingOperation => "last_raws_parse",
            Self::StoredSettings => "stored_settings",
        }
    }

    /// Returns a human-readable description for documentation or UI.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::LastRawsInsertion => {
                "The ISO-8601 (RFC 3339) timestamp of the last successful raw database sync"
            }
            Self::UseSteamAutodetect => {
                "Whether to use the Steam-based autodetect to find relevant directories"
            }
            Self::RecentSearchTerms => {
                "Array of the last 10 searches (greedy, so prefers the most complete search; e.g. searched for 'dva','dvar', and 'dvark' will store 'dvark')"
            }
            Self::PreviousDwarfFortressGamePath => {
                "The last-used path for the Dwarf Fortress installation directory"
            }
            Self::PreviousDwarfFortressUserPath => {
                "The last-used path for the Dwarf Fortress user-data directory"
            }
            Self::PreviousParseDuration => "The duration of the last parse",
            Self::PreviousParserOptions => "The options used for the last parse",
            Self::FavoriteRaws => "Array of raws the user has favorited",
            Self::PreferredSearchLimit => "The user preference for how many results per page",
            Self::PreviousInsertionDuration => {
                "The duration of the last raws insertion to the database"
            }
            Self::LastRawsParsingOperation => {
                "The ISO-8601 (RFC 3339) timestamp of the last successful raw parsing operation"
            }
            Self::StoredSettings => "A JSON blob of stored settings used by a consuming app.",
        }
    }
}
