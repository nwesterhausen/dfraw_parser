//! Helper struct for managing locations related to the game directory and user directory.

use std::io::Write;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{
    constants::DF_STEAM_APPID,
    metadata::RawModuleLocation,
    utilities::{find_game_path, find_user_data_path},
};

/// Helper struct for managing locations related to the game directory and user directory.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LocationHelper {
    df_directory: Option<PathBuf>,
    user_df_directory: Option<PathBuf>,
}

impl LocationHelper {
    /// Create a new instance of LocationHelper.
    ///
    /// This automatically initializes the game directory and user directory.
    #[must_use]
    pub fn new() -> Self {
        let mut helper = Self {
            df_directory: None,
            user_df_directory: None,
        };
        helper.init();
        helper
    }

    /// Get the game directory.
    pub fn get_df_directory(&self) -> Option<&PathBuf> {
        self.df_directory.as_ref()
    }

    /// Get the user directory.
    pub fn get_user_df_directory(&self) -> Option<&PathBuf> {
        self.user_df_directory.as_ref()
    }

    /// Initialize the game directory and user directory.
    ///
    /// This can be called at any time to update the game directory and user directory.
    pub fn init(&mut self) {
        // Get app installation directory
        self.df_directory = find_game_path(DF_STEAM_APPID);
        // Get user directory
        self.user_df_directory = find_user_data_path();
    }

    pub fn set_user_df_directory(&mut self, path: PathBuf) {
        // Validate the path
        if !path.exists() {
            writeln!(std::io::stderr(), "Invalid Path: {path:?} (doesn't exist)")
                .expect("Failed to write to stderr.");
            return;
        }

        self.user_df_directory = Some(path);
    }

    pub fn set_df_directory(&mut self, path: PathBuf) {
        // Validate the path
        if !path.exists() {
            writeln!(std::io::stderr(), "Invalid Path: {path:?} (doesn't exist)")
                .expect("Failed to write to stderr.");
            return;
        }

        self.df_directory = Some(path);
    }

    /// Get the path for a given `RawModuleLocation`.
    ///
    /// Parameters:
    /// - `location`: The location of the module.
    ///
    /// Returns:
    /// - `Option<PathBuf>`: The path to the module, or `None` if the location is unknown.
    pub fn get_path_for_location(&self, location: RawModuleLocation) -> Option<PathBuf> {
        match location {
            RawModuleLocation::InstalledMods | RawModuleLocation::Mods => self
                .user_df_directory
                .as_ref()
                .map(|dir| dir.join(location.get_path())),
            RawModuleLocation::Vanilla | RawModuleLocation::LegendsExport => self
                .df_directory
                .as_ref()
                .map(|dir| dir.join(location.get_path())),
            RawModuleLocation::Unknown => None,
        }
    }
}
