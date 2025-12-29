use directories::BaseDirs;
use std::path::PathBuf;
use std::{fs, path::Path};

#[cfg(feature = "windows-support")]
use winreg::{enums::*, RegKey};

/// Find the path to a specific game installed in Steam.
///
/// This function searches through all Steam libraries to locate the specified game by its app ID.
/// It returns the path to the game's installation directory if found, or None if not found.
///
/// Parameters:
/// - `app_id`: The Steam app ID of the game to find.
///
/// Returns:
/// - `Option<PathBuf>`: The path to the game's installation directory if found, or None if not found.
pub fn find_game_path(app_id: u32) -> Option<PathBuf> {
    let steam_path = get_steam_base_path()?;

    let library_folders = get_library_folders(&steam_path);

    for lib_path in library_folders {
        let manifest_path = lib_path
            .join("steamapps")
            .join(format!("appmanifest_{}.acf", app_id));

        if manifest_path.exists() {
            // Parse the manifest to get the actual installation directory name
            // (The folder name isn't always the same as the game name)
            if let Some(install_dir_name) = parse_install_dir_from_manifest(&manifest_path) {
                return Some(lib_path.join("steamapps/common").join(install_dir_name));
            }
        }
    }
    None
}

/// Read the installation directory from a Steam app manifest file.
///
/// Returns the installation directory name if found, otherwise None.
///
/// Parameters:
/// - `path`: The path to the Steam app manifest file.
///
/// Returns:
/// - `Option<String>`: The installation directory name if found, otherwise None.
fn parse_install_dir_from_manifest(path: &PathBuf) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    // Quick and dirty parser for the "installdir" key
    for line in content.lines() {
        if line.contains("\"installdir\"") {
            let parts: Vec<&str> = line.split('"').collect();
            // The value is usually in the 4th slot: "key" "value" -> ["", "key", " ", "value", ""]
            if parts.len() >= 4 {
                return Some(parts[3].to_string());
            }
        }
    }
    None
}

/// Find all Steam Library folders.
///
/// Returns a vector of paths to Steam library folders. Steam is able to install
/// games on multiple drives or library locations. This function is used to find
/// all library folders.
///
/// Parameters:
/// - `steam_base`: The base path to the Steam installation.
///
/// Returns:
/// - `Vec<PathBuf>`: A vector of paths to Steam library folders.
fn get_library_folders(steam_base: &Path) -> Vec<PathBuf> {
    let mut paths = vec![steam_base.to_path_buf()]; // Always check the main Steam folder
    let vdf_path = steam_base.join("steamapps/libraryfolders.vdf");

    if let Ok(content) = fs::read_to_string(vdf_path) {
        for line in content.lines() {
            if line.contains("\"path\"") {
                let parts: Vec<&str> = line.split('"').collect();
                if parts.len() >= 4 {
                    paths.push(PathBuf::from(parts[3]));
                }
            }
        }
    }
    paths
}

// --- Platform Specific: Get Base Steam Path ---
/// This function retrieves the base path to the Steam installation on Windows by reading the registry key.
///
/// Returns:
/// - `Option<PathBuf>`: The base path to the Steam installation on Windows, or `None` if the path cannot be found.
#[cfg(feature = "windows-support")]
fn get_steam_base_path() -> Option<PathBuf> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu.open_subkey("Software\\Valve\\Steam").ok()?;
    let path_str: String = key.get_value("SteamPath").ok()?;
    Some(PathBuf::from(path_str))
}

/// This function retrieves the base path to the Steam installation on Linux by checking the user's home directory.
/// This may work on macOS as well, but since Dwarf Fortress is not available on macOS, this function is not implemented.
///
/// Returns:
/// - `Option<PathBuf>`: The base path to the Steam installation on Linux, or `None` if the path cannot be found.
fn get_steam_base_path() -> Option<PathBuf> {
    let base_dirs = BaseDirs::new()?;
    let home = base_dirs.home_dir();

    // Priority list of paths to check
    let possible_paths = [
        // 1. Standard Native (Most common)
        home.join(".local/share/Steam"),
        // 2. Symlink Native (Older Ubuntu/Debian)
        home.join(".steam/steam"),
        // 3. Flatpak (Sandboxed version)
        home.join(".var/app/com.valvesoftware.Steam/.local/share/Steam"),
        // 4. Flatpak Alternate (Sometimes seen in older Flatpak configs)
        home.join(".var/app/com.valvesoftware.Steam/.steam/steam"),
    ];

    possible_paths.into_iter().find(|path| path.exists())
}
