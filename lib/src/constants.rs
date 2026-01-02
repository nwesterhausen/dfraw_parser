//! Constants used throughout the library

/// The encoding used by dwarf fortress text files
pub static DF_ENCODING: once_cell::sync::Lazy<&encoding_rs::Encoding> =
    once_cell::sync::Lazy::new(|| {
        encoding_rs::Encoding::for_label(b"latin1").unwrap_or_else(|| {
            panic!("Failed to get encoding: latin1");
        })
    });

/// The Steam AppID for Dwarf Fortress
pub const DF_STEAM_APPID: u32 = 975370;
