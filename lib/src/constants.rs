//! Constants used throughout the library

use std::sync::LazyLock;

use uuid::Uuid;

/// The encoding used by dwarf fortress text files
pub static DF_ENCODING: LazyLock<&'static encoding_rs::Encoding> = LazyLock::new(|| {
    encoding_rs::Encoding::for_label(b"latin1").expect("Failed to get encoding: latin1")
});

/// The Steam AppID for Dwarf Fortress
pub const DF_STEAM_APPID: u32 = 975370;

/// A namespace UUID for this project
pub const DFRAW_PARSER_NAMESPACE: Uuid = Uuid::from_u128(0x6ba7b810_9dad_11d1_80b4_00c04fd430c8);
