//! Custom wrapper for parsing the tile character

use std::str::FromStr;

/// Custom wrapper for the Tile character used in tags
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, specta::Type, Eq)]
pub struct TileCharacter {
    /// The character used for the tile
    pub value: char,
}

impl TileCharacter {
    pub const fn new() -> Self {
        Self { value: '?' }
    }
}

impl Default for TileCharacter {
    fn default() -> Self {
        Self { value: '?' }
    }
}

impl FromStr for TileCharacter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Cannot create a tile without a char to use.".into());
        }

        if s.len() > 1 {
            // Sometimes an integer is provided for the ASCII character code.
            if let Ok(ascii_code) = s.parse::<u32>() {
                if let Some(ascii_char) = char::from_u32(ascii_code) {
                    return Ok(TileCharacter { value: ascii_char });
                }
            };
            // Othertimes it's a literal quoted character with `'`: `'t'`
            let stripped = s.trim_matches('\'');
            if stripped.len() != 1 {
                return Err(format!("Unable to find single character to use: {s}"));
            }
            return TileCharacter::from_str(stripped);
        }

        match s.chars().next() {
            Some(chr) => Ok(Self { value: chr }),
            None => Err(format!("Unable to find valid character: {s}")),
        }
    }
}
