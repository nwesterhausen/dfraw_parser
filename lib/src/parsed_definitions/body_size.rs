//! A module containing the `[BodySize]` struct and its implementation.

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
    /// use dfraw_parser::BodySize;
    /// let size = BodySize::from_value("1:150:5000");
    /// ```
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
