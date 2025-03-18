//! A module containing the `Color` struct and its implementations.

use crate::traits::Insertable;

/// A struct representing a color in the format "foreground:background:brightness".
#[allow(clippy::module_name_repetitions)]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    foreground: u8,
    background: u8,
    brightness: u8,
}

impl Color {
    /// The function `from_value` takes a string value and splits it into three parts to create a
    /// `Color` struct, or returns a default `Color` if the string does not have three parts.
    ///
    /// # Arguments
    ///
    /// * `value`: A string representing a color in the format "foreground:background:brightness".
    ///
    /// # Returns
    ///
    /// * the `Color` struct.
    #[must_use]
    pub fn from_value(value: &str) -> Self {
        let split = value.split(':').collect::<Vec<&str>>();
        if split.len() == 3 {
            return Self {
                foreground: split.first().unwrap_or(&"").parse::<u8>().unwrap_or(0),
                background: split.get(1).unwrap_or(&"").parse::<u8>().unwrap_or(0),
                brightness: split.get(2).unwrap_or(&"").parse::<u8>().unwrap_or(0),
            };
        }
        Self::default()
    }
    /// The function `is_default` returns whether the color is the default color.
    ///
    /// # Returns
    ///
    /// * `true` if the color is the default color, `false` otherwise.
    #[must_use]
    pub const fn is_default(&self) -> bool {
        self.foreground == 0 && self.background == 0 && self.brightness == 0
    }

    /// Returns the foreground color.
    ///
    /// # Returns
    ///
    /// * The foreground color.
    #[must_use]
    pub const fn get_foreground(&self) -> u8 {
        self.foreground
    }

    /// Returns the background color.
    ///
    /// # Returns
    ///
    /// * The background color.
    #[must_use]
    pub const fn get_background(&self) -> u8 {
        self.background
    }

    /// Returns the brightness.
    ///
    /// # Returns
    ///
    /// * The brightness.
    #[must_use]
    pub const fn get_brightness(&self) -> u8 {
        self.brightness
    }
}

impl Insertable for Color {
    fn to_insert_sql(&self) -> String {
        format!(
            "INSERT INTO color (foreground, background, brightness) VALUES ({}, {}, {})",
            self.foreground, self.background, self.brightness
        )
    }
}
