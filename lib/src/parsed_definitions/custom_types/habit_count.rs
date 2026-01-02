//! Implementation of custom 'HABIT_NUM' value which can be a number or "TEST_ALL"
use std::str::FromStr;

/// The 'HABIT_NUM' value which can be a number or "TEST_ALL"
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, specta::Type, Eq)]
pub enum HabitCount {
    TestAll,
    Specific(u32),
}

// Implement Default so "NONE" handling works
impl Default for HabitCount {
    fn default() -> Self {
        Self::Specific(0)
    }
}

impl From<HabitCount> for u32 {
    fn from(val: HabitCount) -> Self {
        match val {
            HabitCount::Specific(num) => num,
            HabitCount::TestAll => 0,
        }
    }
}
impl From<&HabitCount> for u32 {
    fn from(val: &HabitCount) -> Self {
        match *val {
            HabitCount::Specific(num) => num,
            HabitCount::TestAll => 0,
        }
    }
}

// To allow `parse_single` to work
impl FromStr for HabitCount {
    type Err = String; // Or a specific error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TEST_ALL" => Ok(Self::TestAll),
            _ => {
                // Try to parse as a number
                let val = s
                    .parse::<u32>()
                    .map_err(|_| format!("Invalid habit number: {s}"))?;
                Ok(Self::Specific(val))
            }
        }
    }
}
