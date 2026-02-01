//! A module containing the `[BodySize]` struct and its implementation.

use std::str::FromStr;

use tracing::error;

/// Represents a creature's body size at a specific age.
///
/// This structure is used to define growth stages for creatures in Dwarf Fortress raw files.
/// It corresponds to the `[BODY_SIZE:YEARS:DAYS:SIZE_CM3]` tag.
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
    /// Creates a new, empty [`BodySize`]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            years: 0,
            days: 0,
            size_cm3: 0,
        }
    }

    /// Parses a raw body size value string into a [`BodySize`] struct.
    ///
    /// * `value` - A string slice in the format `years:days:size_cm3`.
    ///
    /// Returns a new instance of [`BodySize`].
    ///
    /// The string is split by the colon delimiter. If any component fails to parse as a `u32`
    /// or if the string does not contain exactly three parts, the respective fields default to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use dfraw_parser::custom_types::BodySize;
    /// let size = BodySize::from_value("1:150:5000");
    /// ```
    #[must_use]
    pub fn from_value(value: &str) -> Self {
        match Self::from_str(value) {
            Ok(size) => size,
            Err(e) => {
                error!("BodySize::from_value {e}");
                Self::default()
            }
        }
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
    /// Returns the "value" of the bodysize, as used in raw files
    #[must_use]
    pub fn as_value(&self) -> String {
        format!("{}:{}:{}", self.years, self.days, self.size_cm3)
    }
}

impl std::fmt::Display for BodySize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let year_ending = if self.years != 1 { "s" } else { "" };
        let day_ending = if self.days != 1 { "s" } else { "" };

        match (self.years, self.days) {
            (_, 0) => write!(
                f,
                "{}cm³ at {} year{}",
                self.size_cm3, self.years, year_ending
            ),
            (0, _) => write!(f, "{}cm³ at {} day{}", self.size_cm3, self.days, day_ending),
            _ => write!(
                f,
                "{}cm³ at {} year{}, {} day{}",
                self.size_cm3, self.years, year_ending, self.days, day_ending
            ),
        }
    }
}

impl FromStr for BodySize {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(':').collect::<Vec<&str>>();
        if split.len() != 3 {
            return Err("BodySize requires 3 ':'-separated values; cannot parse {s}".into());
        }

        let years = split[0]
            .parse::<u32>()
            .map_err(|e| format!("Invalid years: {e}"))?;

        let days = split[1]
            .parse::<u32>()
            .map_err(|e| format!("Invalid days: {e}"))?;

        let size_cm3 = split[2]
            .parse::<u32>()
            .map_err(|e| format!("Invalid size_cm3: {e}"))?;

        Ok(Self {
            years,
            days,
            size_cm3,
        })
    }
}
