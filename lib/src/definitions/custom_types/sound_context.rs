use std::{fmt::Display, str::FromStr};

/// Specifies where to attach a body part or tissue
#[derive(
    Debug, Default, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, specta::Type, Eq,
)]
pub enum SoundContext {
    /// Can play at any time, including in menus.
    #[default]
    Any,
    /// The gamemode is Fortress Mode.
    Main,
    /// You are controlling a fortress that is less than one year old.
    FirstYear,
    /// Your fortress has been around for more than one year.
    SecondYearPlus,
    /// Your fortress has access to the caverns.
    CavernsOpened,
    /// The current season is spring. Appears to also be played in Legends mode and the main menu.
    Spring,
    /// The current season is summer. Appears to also be played in Legends mode and the main menu.
    Summer,
    /// The current season is autumn. Appears to also be played in Legends mode and the main menu.
    Autumn,
    /// The current season is winter.
    Winter,
}

// To allow `parse_single` to work
impl FromStr for SoundContext {
    type Err = String; // Or a specific error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ANY" => Ok(SoundContext::Any),
            "MAIN" => Ok(SoundContext::Main),
            "FIRST_YEAR" => Ok(SoundContext::FirstYear),
            "SECOND_YEAR_PLUS" => Ok(SoundContext::SecondYearPlus),
            "CAVERNS_OPENED" => Ok(SoundContext::CavernsOpened),
            "SPRING" => Ok(SoundContext::Spring),
            "SUMMER" => Ok(SoundContext::Summer),
            "AUTUMN" => Ok(SoundContext::Autumn),
            "WINTER" => Ok(SoundContext::Winter),
            _ => Err(format!("Unknown body part position '{s}'")),
        }
    }
}

impl Display for SoundContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SoundContext::Any => write!(f, "ANY"),
            SoundContext::Main => write!(f, "MAIN"),
            SoundContext::FirstYear => write!(f, "FIRST_YEAR"),
            SoundContext::SecondYearPlus => write!(f, "SECOND_YEAR_PLUS"),
            SoundContext::CavernsOpened => write!(f, "CAVERNS_OPENED"),
            SoundContext::Spring => write!(f, "SPRING"),
            SoundContext::Summer => write!(f, "SUMMER"),
            SoundContext::Autumn => write!(f, "AUTUMN"),
            SoundContext::Winter => write!(f, "WINTER"),
        }
    }
}
