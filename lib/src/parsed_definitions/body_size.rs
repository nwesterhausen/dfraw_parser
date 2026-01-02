//! A module containing the `BodySize` struct and its implementation.

/// A struct representing a body size in the format `years:days:size_cm3`
#[allow(clippy::module_name_repetitions)]
#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq, specta::Type,
)]
#[serde(rename_all = "camelCase")]
pub struct BodySize {
    years: u32,
    days: u32,
    size_cm3: u32,
}

impl BodySize {
    /// Creates a new `BodySize` struct with the given years, days, and `size_cm3`
    ///
    /// # Arguments
    ///
    /// * `value` - The value to parse into a `BodySize` struct (e.g. `1:2:3`)
    ///
    /// # Returns
    ///
    /// * The `BodySize` struct
    #[must_use]
    pub fn from_value(value: &str) -> Self {
        let split = value.split(':').collect::<Vec<&str>>();
        if split.len() == 3 {
            return Self {
                years: split.first().unwrap_or(&"").parse::<u32>().unwrap_or(0),
                days: split.get(1).unwrap_or(&"").parse::<u32>().unwrap_or(0),
                size_cm3: split.get(2).unwrap_or(&"").parse::<u32>().unwrap_or(0),
            };
        }
        Self::default()
    }
    /// Returns the years portion of this body size
    #[must_use]
    pub fn get_years(&self) -> u32 {
        self.years
    }
    /// Returns the days portion of the body size
    #[must_use]
    pub fn get_days(&self) -> u32 {
        self.days
    }
    /// Returns the size portion of the body size
    #[must_use]
    pub fn get_size(&self) -> u32 {
        self.size_cm3
    }
}
