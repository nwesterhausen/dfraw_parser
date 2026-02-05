use std::{fmt::Display, str::FromStr};

/// Specifies where to attach a body part or tissue
#[derive(
    Debug, Default, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, specta::Type, Eq,
)]
pub enum BodyPartPosition {
    /// Front
    #[default]
    Front,
    /// Back
    Back,
    /// Left
    Left,
    /// Right
    Right,
    /// Top
    Top,
    /// Bottom
    Bottom,
}

// To allow `parse_single` to work
impl FromStr for BodyPartPosition {
    type Err = String; // Or a specific error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FRONT" => Ok(BodyPartPosition::Front),
            "BACK" => Ok(BodyPartPosition::Back),
            "LEFT" => Ok(BodyPartPosition::Left),
            "RIGHT" => Ok(BodyPartPosition::Right),
            "TOP" => Ok(BodyPartPosition::Top),
            "BOTTOM" => Ok(BodyPartPosition::Bottom),
            _ => Err(format!("Unknown body part position '{s}'")),
        }
    }
}

impl Display for BodyPartPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BodyPartPosition::Front => write!(f, "FRONT"),
            BodyPartPosition::Back => write!(f, "BACK"),
            BodyPartPosition::Left => write!(f, "LEFT"),
            BodyPartPosition::Right => write!(f, "RIGHT"),
            BodyPartPosition::Top => write!(f, "TOP"),
            BodyPartPosition::Bottom => write!(f, "BOTTOM"),
        }
    }
}
