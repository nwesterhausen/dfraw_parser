/// Tokens specifically for steam workshop data that can be found in a modules info.txt file
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
pub enum SteamWorkshopToken {
    /// The title of the mod on Steam Workshop.
    Title { title: String },
    /// The description of the mod on Steam Workshop. Maximum size is 8000 bytes (about 400 words).
    /// Will overwrite the existing description of the mod on the workshop, can be omitted to avoid this behavior.
    Description { description: String },
    /// Any amount of these can be used. Use a separate STEAM_TAG for each one. Each string must be under 255 chars.
    Tag { tag: String },
    /// Any amount of these can be used. Should be a single key = value relationship. Can be used in searching.
    KeyValueTag { key: String, value: String },
    /// Sets arbitrary metadata for an item. This metadata can be returned from queries without having to download and
    /// install the actual content.
    Metadata { metadata: String },
    /// A brief description of the changes made. (Optional, set to NULL for no change note). The log message is only
    /// for the version you're uploading. This should be different each time you update a mod, and only include the
    /// changes in the new version.
    ///
    /// Steam Workshop congregates all version changelogs, so a full changelog can be seen there.
    Changelog { changes: String },
    /// Connects the mod to an entry on the Steam Workshop.
    FileId { id: u64 },
    /// An unknown tag
    #[default]
    Unknown,
}
