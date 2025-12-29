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
    InstalledMods,
    /// The "downloaded" mods directory
    Mods,
    /// The vanilla data file location
    Vanilla,
    /// An unknown location
    #[default]
    Unknown,
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
            Self::Mods => PathBuf::from("mods"),
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
    /// * `df_directory` - The path to the Dwarf Fortress directory
    ///
    /// # Returns
    ///
    /// * A `RawModuleLocation` representing the path
    #[must_use]
    pub fn from_path<P: AsRef<Path>>(path: &P) -> Self {
        path.as_ref()
            .file_name()
            .map_or(Self::Unknown, |file_name| {
                match file_name.to_string_lossy().as_ref() {
                    "mods" => Self::Mods,
                    "installed_mods" => Self::InstalledMods,
                    "vanilla" => Self::Vanilla,
                    _ => {
                        warn!(
                            "RawModuleLocation - Unable to match source directory \"{dir}\"",
                            dir = file_name.to_string_lossy()
                        );
                        Self::Unknown
                    }
                }
            })
    }
    /// Returns a `RawModuleLocation` from a sourced directory
    ///
    /// # Arguments
    ///
    /// * `sourced_directory` - The sourced directory
    ///
    /// # Returns
    ///
    /// * A `RawModuleLocation` representing the sourced directory
    #[must_use]
    pub fn from_sourced_directory(sourced_directory: &str) -> Self {
        match sourced_directory {
            "mods" => Self::Mods,
            "vanilla" => Self::Vanilla,
            "installed_mods" => Self::InstalledMods,
            _ => {
                warn!(
                    "RawModuleLocation - Unable to match source directory \"{dir}\"",
                    dir = sourced_directory
                );
                Self::Unknown
            }
        }
    }
    /// Returns a `RawModuleLocation` from an info.txt file path
    ///
    /// # Arguments
    ///
    /// * `full_path` - The full path to the info.txt file
    ///
    /// # Returns
    ///
    /// * A `RawModuleLocation` representing the info.txt file path
    #[must_use]
    pub fn from_info_text_file_path<P: AsRef<Path>>(full_path: &P) -> Self {
        // info.txt is relative by 2 parents from our module location
        // <MODULE LOCATION>/<RAW MODULE>/info.txt
        match full_path.as_ref().parent() {
            Some(parent_dir) => match parent_dir.parent() {
                Some(grandparent_dir) => {
                    let path_string = String::from(
                        grandparent_dir
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy(),
                    );
                    Self::from_sourced_directory(path_string.as_str())
                }
                None => Self::Unknown,
            },
            None => Self::Unknown,
        }
    }
}

impl Display for RawModuleLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
