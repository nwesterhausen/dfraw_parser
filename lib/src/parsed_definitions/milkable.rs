//! Milkable struct and implementation

use dfraw_parser_proc_macros::{Cleanable, IsEmpty};

#[allow(clippy::module_name_repetitions)]
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    specta::Type,
    IsEmpty,
    Cleanable,
)]
#[serde(rename_all = "camelCase")]
/// How often a creature can be milked and what material it produces
pub struct Milkable {
    material: String,
    frequency: u32,
}

impl Milkable {
    /// Creates a new Milkable struct with the given material and frequency
    ///
    /// # Arguments
    ///
    /// * `value` - The value to parse into a Milkable struct (e.g. `MILK:1`)
    #[must_use]
    pub fn from_value(value: &str) -> Self {
        let split = value.split(':').collect::<Vec<&str>>();
        if split.len() == 2 {
            let material_str = *split.first().unwrap_or(&"");
            return Self {
                material: String::from(material_str),
                frequency: split.get(1).unwrap_or(&"").parse::<u32>().unwrap_or(0),
            };
        }
        Self::default()
    }
    /// Returns whether the milkable is the default milkable
    ///
    /// # Returns
    ///
    /// * `true` if the milkable is the default milkable, `false` otherwise
    #[must_use]
    pub fn is_default(&self) -> bool {
        self.material.is_empty() && self.frequency == 0
    }
    /// Returns the milkable as a vector of strings
    ///
    /// # Returns
    ///
    /// * `Vec<String>` - The milkable as a vector of strings
    #[must_use]
    pub fn as_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();
        if !self.material.is_empty() {
            vec.push(self.material.clone());
        }
        if self.frequency > 0 {
            vec.push(self.frequency.to_string());
        }
        vec
    }
    /// Returns the material of the milkable
    ///
    /// # Returns
    ///
    /// * `&str` - The material of the milkable
    #[must_use]
    pub fn get_material(&self) -> &str {
        &self.material
    }
    /// Returns the frequency of the milkable
    ///
    /// # Returns
    ///
    /// * `u32` - The frequency of the milkable
    #[must_use]
    pub const fn get_frequency(&self) -> u32 {
        self.frequency
    }
}
