//! Raw module location helper.
//!
//! Based on version 53.08 of Dwarf Fortress, game raws are stored in these locations:
//! * `{df_directory}/data/vanilla` (vanilla raws)
//! * `{user_df_directory}/mods` (downloaded mods from the steam workshop)
//! * `{user_df_directory}/data/installed_mods` (mods used in at least one save file)
//!
//! It used to be in older version that `mods` and `installed_mods` were in the same directory as `data`.
use std::{
    fmt::{Debug, Display},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use tracing::warn;

/// Raws are part of modules since 50.xx. Raw modules are loaded from 3 common locations:
/// `{df_directory}/data/vanilla`, `{df_directory}/mods`, and `{df_directory/data/installed_mods}`
#[derive(
    Serialize, Debug, Deserialize, Clone, Copy, PartialEq, Eq, Default, Hash, specta::Type,
)]
pub enum RawModuleLocation {
    /// The "installed" mods directory
    InstalledMods = 3,
    /// The "downloaded" mods directory
    WorkshopMods = 2,
    /// The vanilla data file location
    Vanilla = 1,
    /// An unknown location
    #[default]
    Unknown = 4,
    /// Used for handling legends exported files
    LegendsExport,
}

impl RawModuleLocation {
    /// Returns the path of the raw module location
    ///
    /// # Returns
    ///
    /// * A `PathBuf` representing the path of the raw module location
    #[must_use]
    pub fn get_path(self) -> PathBuf {
        match self {
            Self::WorkshopMods => PathBuf::from("mods"),
            Self::InstalledMods => ["data", "installed_mods"].iter().collect(),
            Self::Vanilla => ["data", "vanilla"].iter().collect(),
            Self::Unknown => PathBuf::from("unknown"),
            Self::LegendsExport => PathBuf::from("."),
        }
    }
    /// Returns a `RawModuleLocation` from a path
    ///
    /// # Arguments
    ///
    /// * `path`: the path of an `info.txt` file or the module directory
    ///
    /// # Returns
    ///
    /// * A `RawModuleLocation` representing the path it was found in
    #[must_use]
    pub fn from_path<P: AsRef<Path>>(path: &P) -> Self {
        for component in path.as_ref().iter().rev() {
            match component.to_string_lossy().as_ref() {
                "mods" => return Self::WorkshopMods,
                "installed_mods" => return Self::InstalledMods,
                "vanilla" => return Self::Vanilla,
                _ => continue, // Not a match, keep checking the next part
            }
        }

        // If the loop finishes without returning, no match was found
        warn!(
            "RawModuleLocation - Unable to match source directory \"{:?}\"",
            path.as_ref()
        );
        Self::Unknown
    }
}

impl Display for RawModuleLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl From<RawModuleLocation> for i32 {
    fn from(value: RawModuleLocation) -> Self {
        match value {
            RawModuleLocation::InstalledMods => 2,
            RawModuleLocation::WorkshopMods => 3,
            RawModuleLocation::Vanilla => 1,
            RawModuleLocation::Unknown | RawModuleLocation::LegendsExport => 4,
        }
    }
}
impl From<&RawModuleLocation> for i32 {
    fn from(value: &RawModuleLocation) -> Self {
        match *value {
            RawModuleLocation::InstalledMods => 2,
            RawModuleLocation::WorkshopMods => 3,
            RawModuleLocation::Vanilla => 1,
            RawModuleLocation::Unknown | RawModuleLocation::LegendsExport => 4,
        }
    }
}
