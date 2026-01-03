//! Helper struct for managing locations related to the game directory and user directory.
use std::path::PathBuf;

use crate::{
    constants::DF_STEAM_APPID,
    metadata::RawModuleLocation,
    utilities::{find_game_path, find_user_data_path},
    ParserError,
};

/// Helper struct for managing locations related to the game directory and user directory.
#[derive(
    Default, Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash, specta::Type,
)]
pub struct LocationHelper {
    df_directory: Option<PathBuf>,
    user_data_directory: Option<PathBuf>,
}

impl LocationHelper {
    /// Create a new instance of LocationHelper.
    ///
    /// This automatically initializes the game directory and user directory.
    #[must_use]
    pub fn new() -> Self {
        let mut helper = Self {
            df_directory: None,
            user_data_directory: None,
        };
        helper.init(true);
        helper
    }

    /// Get the game directory.
    pub fn get_df_directory(&self) -> Option<&PathBuf> {
        self.df_directory.as_ref()
    }

    /// Get the user directory.
    pub fn get_user_data_directory(&self) -> Option<&PathBuf> {
        self.user_data_directory.as_ref()
    }

    /// Initialize the game directory and user directory.
    ///
    /// This can be called at any time to update the game directory and user directory.
    pub fn init(&mut self, force: bool) {
        // Get app installation directory
        if force || self.df_directory.is_none() {
            self.df_directory = find_game_path(DF_STEAM_APPID);
        }
        // Get user directory
        if force || self.user_data_directory.is_none() {
            self.user_data_directory = find_user_data_path();
        }
    }

    /// Set the Dwarf Fortress user data directory explicitly
    ///
    /// Parameters:
    ///
    /// * `path`: the path to the Dwarf Fortress user data directory
    ///
    /// Returns:
    ///
    /// * A result of Ok(()) if directory added successfully (or provided an empty `path`)
    /// * A `ParserError` if there was an issue with the provided directory
    pub fn set_user_data_directory(&mut self, path: &PathBuf) -> Result<(), ParserError> {
        if path.as_os_str().is_empty() {
            tracing::info!("Empty path ignored when called.");
            return Ok(());
        }

        // Canonicalize the path
        let target_path = match path.canonicalize() {
            Ok(p) => p,
            Err(e) => {
                return Err(ParserError::InvalidOptions(format!(
                    "Unable to canonicalize Dwarf Fortress user data path!\n{path:?}\n{e:?}"
                )))
            }
        };

        if !target_path.exists() {
            return Err(ParserError::InvalidOptions(format!(
                "Provided Dwarf Fortress user data directory doesn't exist!\n{}",
                target_path.display()
            )));
        }

        if !target_path.is_dir() {
            return Err(ParserError::InvalidOptions(format!(
                "Dwarf Fortress user data directory needs to be a directory!\n{}",
                target_path.display()
            )));
        }

        self.user_data_directory = Some(target_path);
        Ok(())
    }

    /// Set the Dwarf Fortress game directory explicitly
    ///
    /// Parameters:
    ///
    /// * `path`: the path to the Dwarf Fortress installation directory
    ///
    /// Returns:
    ///
    /// * A result of Ok(()) if directory added successfully (or provided an empty `path`)
    /// * A `ParserError` if there was an issue with the provided directory
    pub fn set_df_directory(&mut self, path: &PathBuf) -> Result<(), ParserError> {
        if path.as_os_str().is_empty() {
            tracing::info!("Empty path ignored when called.");
            return Ok(());
        }

        // Canonicalize the path
        let target_path = match path.canonicalize() {
            Ok(p) => p,
            Err(e) => {
                return Err(ParserError::InvalidOptions(format!(
                    "Unable to canonicalize Dwarf Fortress path!\n{path:?}\n{e:?}"
                )))
            }
        };

        if !target_path.exists() {
            return Err(ParserError::InvalidOptions(format!(
                "Provided Dwarf Fortress path for doesn't exist!\n{}",
                target_path.display()
            )));
        }

        if !target_path.is_dir() {
            return Err(ParserError::InvalidOptions(format!(
                "Dwarf Fortress path needs to be a directory!\n{}",
                target_path.display()
            )));
        }

        tracing::info!("Updating df_directory to {:?}", &target_path.as_os_str());
        self.df_directory = Some(target_path);
        Ok(())
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
                .user_data_directory
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
