use std::{fmt::Display, str::FromStr};

/// Dwarf fortress raws can allow specifying a number for amounts with special cases for
/// AS_NEEDED or NONE. This allows parsing to work without an issue and abstracts away
/// these special values (into 0).
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, specta::Type, Eq)]
pub enum Amount {
    /// `AS_NEEDED` gets treated specially, but for example, for a position it will describe
    /// that the position can be created if it needs to be (military positions).
    AsNeeded,
    /// A specific amount, `u32`
    Specific(u32),
    /// None, a no-op. This could mean "no maximum" or "no minimum" and we transform it to 0
    None,
}

// Implement Default so "NONE" handling works
impl Default for Amount {
    fn default() -> Self {
        Self::Specific(0)
    }
}

impl From<Amount> for u32 {
    fn from(val: Amount) -> Self {
        match val {
            Amount::Specific(num) => num,
            Amount::AsNeeded | Amount::None => 0,
        }
    }
}
impl From<&Amount> for u32 {
    fn from(val: &Amount) -> Self {
        match *val {
            Amount::Specific(num) => num,
            Amount::AsNeeded | Amount::None => 0,
        }
    }
}

// To allow `parse_single` to work
impl FromStr for Amount {
    type Err = String; // Or a specific error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NONE" => Ok(Self::None),
            "AS_NEEDED" => Ok(Self::AsNeeded),
            _ => {
                // Try to parse as a number
                let val = s
                    .parse::<u32>()
                    .map_err(|_| format!("Invalid amount: {s}"))?;
                Ok(Self::Specific(val))
            }
        }
    }
}

impl Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Amount::AsNeeded => write!(f, "AS_NEEDED"),
            Amount::None => write!(f, "NONE"),
            Amount::Specific(val) => write!(f, "{}", val),
        }
    }
}
