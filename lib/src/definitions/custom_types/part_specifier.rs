use std::{fmt::Display, str::FromStr};

/// The method of specifying the method of choosing a body part location
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, specta::Type, Eq, Default,
)]
pub enum PartSpecifier {
    /// `BY_CATEGORY`
    #[default]
    Category,
    /// `BY_TYPE`
    Type,
    /// `BY_TOKEN`
    Token,
}

// To allow `parse_single` to work
impl FromStr for PartSpecifier {
    type Err = String; // Or a specific error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CATEGORY" => Ok(PartSpecifier::Category),
            "BY_TYPE" => Ok(PartSpecifier::Type),
            "BY_TOKEN" => Ok(PartSpecifier::Token),
            _ => Err(format!("Unknown body part specifier '{s}'")),
        }
    }
}

impl Display for PartSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PartSpecifier::Category => write!(f, "BY_CATEGORY"),
            PartSpecifier::Type => write!(f, "BY_TYPE"),
            PartSpecifier::Token => write!(f, "BY_TOKEN"),
        }
    }
}
