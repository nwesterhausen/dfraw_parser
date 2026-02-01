//! A module containing the `Color` struct and its implementations.

use std::str::FromStr;

use dfraw_parser_proc_macros::{Cleanable, IsEmpty};
use tracing::error;

/// Represents a Dwarf Fortress color triplet.
///
/// This format is used throughout the game raws to define the foreground,
/// background, and brightness/intensity of tiles and text.
#[allow(clippy::module_name_repetitions)]
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    IsEmpty,
    Cleanable,
    specta::Type,
)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    /// The foreground color index (0-7).
    foreground: u8,
    /// The background color index (0-7).
    background: u8,
    /// The brightness or intensity toggle (0 or 1).
    brightness: u8,
}

impl Color {
    /// Creates a new, empty Color
    #[must_use]
    pub const fn new() -> Self {
        Self {
            foreground: 0,
            background: 0,
            brightness: 0,
        }
    }

    /// Parses a color triplet from a string value.
    ///
    /// * `value` - A string representing a color in the format "foreground:background:brightness".
    ///
    /// Returns a new [Color] instance, or [Color::default] if the string format is invalid.
    ///
    /// This is typically used to parse values from tags like `[COLOR:7:0:1]` found in raw files.
    ///
    /// # Examples
    ///
    /// ```
    /// use dfraw_parser::custom_types::Color;
    /// let color = Color::from_value("7:0:1");
    /// assert_eq!(color.get_foreground(), 7);
    /// ```
    #[must_use]
    pub fn from_value(value: &str) -> Self {
        match Self::from_str(value) {
            Ok(size) => size,
            Err(e) => {
                error!("Color::from_value {e}");
                Self::default()
            }
        }
    }

    /// Returns true if the color is the default (all components are 0).
    ///
    /// This indicates that no specific color was defined or parsing failed.
    #[must_use]
    pub const fn is_default(&self) -> bool {
        self.foreground == 0 && self.background == 0 && self.brightness == 0
    }

    /// Returns the foreground color index.
    ///
    /// This value corresponds to the 0-7 color palette indices used by the game.
    #[must_use]
    pub fn get_foreground(&self) -> u8 {
        self.foreground
    }

    /// Returns the background color index.
    ///
    /// This value corresponds to the 0-7 color palette indices used by the game.
    #[must_use]
    pub fn get_background(&self) -> u8 {
        self.background
    }

    /// Returns the brightness value.
    ///
    /// A value of 1 typically represents a "bright" or "bold" version of the foreground color.
    #[must_use]
    pub fn get_brightness(&self) -> u8 {
        self.brightness
    }

    /// Returns the "value" of the color as used in raw files
    #[must_use]
    pub fn as_value(&self) -> String {
        format!(
            "{}:{}:{}",
            self.foreground, self.background, self.brightness
        )
    }
}

impl std::convert::From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Self {
        Self {
            foreground: value.0,
            background: value.1,
            brightness: value.2,
        }
    }
}

impl PartialEq<(u8, u8, u8)> for Color {
    fn eq(&self, other: &(u8, u8, u8)) -> bool {
        self.foreground == other.0 && self.background == other.1 && self.brightness == other.2
    }
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(':').collect::<Vec<&str>>();
        if split.len() != 3 {
            return Err("Color requires 3 ':'-separated values; cannot parse {s}".into());
        }

        let foreground = split[0]
            .trim()
            .parse::<u8>()
            .map_err(|e| format!("Invalid foreground: {e}"))?;

        let background = split[1]
            .trim()
            .parse::<u8>()
            .map_err(|e| format!("Invalid background: {e}"))?;

        let brightness = split[2]
            .trim()
            .parse::<u8>()
            .map_err(|e| format!("Invalid brightness: {e}"))?;

        Ok(Self {
            foreground,
            background,
            brightness,
        })
    }
}
