//! Tokens used in module definitions (in `info.txt` files)

/// Tokens that can be found in a creature's caste definitions.
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Default,
    specta::Type,
    strum_macros::EnumIter,
)]
pub enum ModuleInfoToken {
    /// An integer version number for the mod. Must be greater than or equal to EARLIEST_COMPATIBLE_NUMERIC_VERSION.
    /// "Integer" here means 0, 1, 2, and so on. Negatives are allowed. Anything that is not an integer will not work;
    ///
    /// 0.2 will be read as "0".
    NumericVersion { version: i32 },
    /// The version of the mod, as displayed in-game. This is only a display, and will have no effect.
    DisplayedVersion { version: String },
    /// The earliest compatible numeric version of the mod. Installed mods are automatically updated, if a later
    /// compatible version is available. This must be at most the same as NUMERIC_VERSION, and doing otherwise will
    /// result in an error.
    EarliestCompatibleNumericVersion { version: i32 },
    /// The earliest compatible numeric version, as displayed in-game.
    EarliestCompatibleDisplayedVersion { version: String },
    /// The name of the author
    Author { name: String },
    /// The name of the mod
    Name { name: String },
    /// A description of the mod, shown in the mod loading screen
    Description { description: String },
    /// Mod cannot be used unless mod with given ID is also loaded.
    RequiresId { id: String },
    /// Mod cannot be used unless mod with given ID is earlier in the mod load list.
    RequiresIdBeforeMe { id: String },
    /// Mod cannot be used unless mod with given ID is later in the mod load list.
    RequiresIdAfterMe { id: String },
    /// Mod cannot be used if mod with given ID is also loaded.
    ConflictsWithId { id: String },
    /// The title of the mod on Steam Workshop.
    SteamTitle { title: String },
    /// The description of the mod on Steam Workshop. Maximum size is 8000 bytes (about 400 words).
    /// Will overwrite the existing description of the mod on the workshop, can be omitted to avoid this behavior.
    SteamDescription { description: String },
    /// Any amount of these can be used. Use a separate STEAM_TAG for each one. Each string must be under 255 chars.
    SteamTag { tag: String },
    /// Any amount of these can be used. Should be a single key = value relationship. Can be used in searching.
    SteamKeyValueTag { key: String, value: String },
    /// Sets arbitrary metadata for an item. This metadata can be returned from queries without having to download and
    /// install the actual content.
    SteamMetadata { metdata: String },
    /// A brief description of the changes made. (Optional, set to NULL for no change note). The log message is only
    /// for the version you're uploading. This should be different each time you update a mod, and only include the
    /// changes in the new version.
    ///
    /// Steam Workshop congregates all version changelogs, so a full changelog can be seen there.
    SteamChangelog { changes: String },
    /// Connects the mod to an entry on the Steam Workshop.
    SteamFileId { id: u64 },
    /// An unknown tag
    #[default]
    Unknown,
}
