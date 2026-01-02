use directories::BaseDirs;
use std::path::PathBuf;

/// This function gets the user data path for Dwarf Fortress. It will check the "normal" path first, and
/// fall back to a Flatpak-specific path if necessary.
///
/// Returns the path if found, or None if not found.
pub fn find_user_data_path() -> Option<PathBuf> {
    let base_dirs = BaseDirs::new()?;

    let possible_paths = vec![
        // 1. STANDARD PATH
        // Windows: C:\Users\User\AppData\Roaming\Bay 12 Games\Dwarf Fortress
        // Linux:   /home/user/.local/share/Bay 12 Games/Dwarf Fortress
        // The 'directories' crate abstracts this difference automatically.
        base_dirs
            .data_dir()
            .join("Bay 12 Games")
            .join("Dwarf Fortress"),
        // 2. FLATPAK FALLBACK
        // If the user is running Steam via Flatpak, data is sandboxed here.
        base_dirs
            .home_dir()
            .join(".var/app/com.valvesoftware.Steam/.local/share/Bay 12 Games/Dwarf Fortress"),
    ];

    possible_paths.into_iter().find(|path| path.exists())
}
