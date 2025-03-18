//! A module containing the `Dimensions` struct and its implementations.

use crate::traits::Insertable;
use tracing::{error, warn};

/// A struct representing a Dimensions object.
#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug, Default, specta::Type)]
pub struct Dimensions {
    x: i32,
    y: i32,
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
    pub const fn from_xy(x: i32, y: i32) -> Self {
        Self { x, y }
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

        let x: i32 = match dim_x.parse() {
            Ok(n) => n,
            Err(e) => {
                warn!("Failed to parse {} as Dimensions:x, {:?}", token, e);
                0
            }
        };
        let y: i32 = match dim_y.parse() {
            Ok(n) => n,
            Err(e) => {
                warn!("Failed to parse {} as Dimensions:y, {:?}", token, e);
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

impl Insertable for Dimensions {
    fn to_insert_sql(&self) -> String {
        format!(
            "INSERT OR IGNORE INTO dimension_data (x, y) VALUES ({}, {})",
            self.x, self.y
        )
    }
}
