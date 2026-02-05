use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, specta::Type, Eq)]
pub enum BodyPartSpecifier {
    /// `BY_CATEGORY`
    Category,
    /// `BY_TYPE`
    Type,
    /// `BY_TOKEN`
    Token,
}

// To allow `parse_single` to work
impl FromStr for BodyPartSpecifier {
    type Err = String; // Or a specific error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CATEGORY" => Ok(BodyPartSpecifier::Category),
            "BY_TYPE" => Ok(BodyPartSpecifier::Type),
            "BY_TOKEN" => Ok(BodyPartSpecifier::Token),
            _ => Err(format!("Unknown body part specifier '{s}'")),
        }
    }
}

impl Display for BodyPartSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BodyPartSpecifier::Category => write!(f, "BY_CATEGORY"),
            BodyPartSpecifier::Type => write!(f, "BY_TYPE"),
            BodyPartSpecifier::Token => write!(f, "BY_TOKEN"),
        }
    }
}
