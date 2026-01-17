//! A module containing the `Dimensions` struct and its implementations.

use dfraw_parser_proc_macros::IsEmpty;
use tracing::{error, warn};

/// A struct representing a Dimensions object.
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Copy,
    Clone,
    Debug,
    Default,
    specta::Type,
    PartialEq,
    Eq,
    IsEmpty,
)]
pub struct Dimensions {
    /// The x coordinate
    pub x: i64,
    /// The y coordinate
    pub y: i64,
}

#[allow(dead_code)] // Until we add graphics parsing
impl Dimensions {
    /// Function to create a new Dimensions object with x and y set to 0.
    ///
    /// # Returns
    ///
    /// * `Dimensions` - The new Dimensions object with x and y set to 0.
    #[must_use]
    pub const fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
    /// Returns a new Dimension where each component is the maximum of the two.
    #[must_use]
    pub fn max_components(self, other: Dimensions) -> Dimensions {
        Dimensions {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }
    /// Function to create a new Dimensions object with given x and y values.
    ///
    /// # Parameters
    ///
    /// * `x` - The x value for the new Dimensions object.
    /// * `y` - The y value for the new Dimensions object.
    ///
    /// # Returns
    ///
    /// * `Dimensions` - The new Dimensions object with the given x and y values.
    #[must_use]
    pub fn from_xy(x: u32, y: u32) -> Self {
        (x, y).into()
    }
    /// Function to create a new Dimensions object from a token.
    ///
    /// # Parameters
    ///
    /// * `token` - The token to parse.
    ///
    /// # Returns
    ///
    /// * `Dimensions` - The new Dimensions object parsed from the token.
    #[must_use]
    pub fn from_token(token: &str) -> Self {
        let split = token.split(':').collect::<Vec<&str>>();
        //  [TILE_DIM:32:32]

        let Some(dim_x) = split.first() else {
            error!(
                "Missing required number of tokens for Dimensions! {}",
                token
            );
            return Self { x: 0, y: 0 };
        };
        let Some(dim_y) = split.get(1) else {
            error!(
                "Missing required number of tokens for Dimensions! {}",
                token
            );
            return Self { x: 0, y: 0 };
        };

        Self::from_two_tokens(dim_x, dim_y)
    }
    /// Creates a `Dimensions` from two tokens, one for the `xpos` and one for `ypos`
    ///
    /// If it fails to parse a token, returns `0` for that value.
    pub fn from_two_tokens(dim_x: &&str, dim_y: &&str) -> Self {
        let x: i64 = match dim_x.parse() {
            Ok(n) => n,
            Err(e) => {
                warn!("Failed to parse dim_x: {e}");
                0
            }
        };
        let y: i64 = match dim_y.parse() {
            Ok(n) => n,
            Err(e) => {
                warn!("Failed to parse dim_y: {e}");
                0
            }
        };

        Self { x, y }
    }
    /// Returns an empty Dimensions object.
    ///
    /// # Returns
    ///
    /// * `Dimensions` - The empty Dimensions object.
    #[must_use]
    pub const fn empty() -> Self {
        Self::zero()
    }
    /// Create a new Dimensions object.
    ///
    /// # Returns
    ///
    /// * `Dimensions` - The new Dimensions object.
    #[must_use]
    pub const fn new() -> Self {
        Self::zero()
    }
    /// Whether the Dimensions object is the default.
    ///
    /// # Returns
    ///
    /// * `true` - If the Dimensions object is the default.
    #[must_use]
    pub const fn is_default(self) -> bool {
        self.x == 0 && self.y == 0
    }
    /// Whether the Dimensions object is empty.
    ///
    /// # Returns
    ///
    /// * `true` - If the Dimensions object is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.is_default()
    }
}

#[allow(clippy::from_over_into)]
impl Into<(i64, i64)> for Dimensions {
    fn into(self) -> (i64, i64) {
        (self.x, self.y)
    }
}

impl<T, U> From<(T, U)> for Dimensions
where
    T: TryInto<i64>,
    U: TryInto<i64>,
    T::Error: std::fmt::Display,
    U::Error: std::fmt::Display,
{
    fn from(value: (T, U)) -> Self {
        Self {
            x: value
                .0
                .try_into()
                .inspect_err(|e| tracing::error!("from::x {e}"))
                .unwrap_or(0),
            y: value
                .1
                .try_into()
                .inspect_err(|e| tracing::error!("from::y {e}"))
                .unwrap_or(0),
        }
    }
}
