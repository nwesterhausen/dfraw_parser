//! Seed material definition

use dfraw_parser_proc_macros::IsEmpty;
use tracing::warn;

use crate::{color::Color, name::Name};

/// A struct representing a seed material
#[allow(clippy::module_name_repetitions)]
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    Default,
    specta::Type,
    PartialEq,
    Eq,
    IsEmpty,
)]
#[serde(rename_all = "camelCase")]
pub struct SeedMaterial {
    name: Name,
    color: Color,
    material: String,
}

impl SeedMaterial {
    /// Whether the seed material is empty
    ///
    /// # Returns
    ///
    /// * `true` if the seed material is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.name.is_empty() && self.color.is_default() && self.material.is_empty()
    }
    /// Create a new seed material based on a value
    ///
    /// # Arguments
    ///
    /// * `value` - The value to create the seed material from
    ///
    /// # Returns
    ///
    /// A new seed material
    #[must_use]
    pub fn from_value(value: &str) -> Self {
        // Example seed tag:
        // [SEED:apricot pit:apricot pits:6:0:0:LOCAL_PLANT_MAT:SEED]
        // Leaving value to be "apricot pit:apricot pits:6:0:0:LOCAL_PLANT_MAT:SEED"
        // We need to split value into its parts
        let mut parts = value.split(':');

        // If the parts are less than 7, then we don't have enough information
        if parts.clone().count() < 7 {
            warn!(
                "SeedMaterial::from_value() was provided a value with less than 7 parts: {}",
                value
            );
            return Self::default();
        }

        // The name uses the first two parts
        let name = Name::from_value(&format!(
            "{}:{}",
            parts.next().unwrap_or_default(),
            parts.next().unwrap_or_default()
        ));
        // The color uses the next three parts
        let color = Color::from_value(&format!(
            "{}:{}:{}",
            parts.next().unwrap_or_default(),
            parts.next().unwrap_or_default(),
            parts.next().unwrap_or_default()
        ));
        // The material uses the remaining parts
        let material = parts.collect::<Vec<&str>>().join(":");
        Self {
            name,
            color,
            material,
        }
    }
}
