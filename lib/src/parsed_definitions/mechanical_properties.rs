//! Contains the `MechanicalProperties` struct and its implementation
use dfraw_parser_proc_macros::IsEmpty;

/// Represents the mechanical properties of a material via the yield, fracture, and elasticity
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
pub struct MechanicalProperties {
    #[serde(rename = "yield")]
    yield_stress: i32,
    fracture: i32,
    elasticity: i32,
}

impl MechanicalProperties {
    /// Creates a new Properties struct
    ///
    /// # Returns
    ///
    /// * The Properties struct
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Returns whether the properties are empty
    ///
    /// # Returns
    ///
    /// * `true` if the properties are empty, `false` otherwise.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.yield_stress == 0 && self.fracture == 0 && self.elasticity == 0
    }
    /// Sets the yield stress of the material
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set
    pub fn set_yield(&mut self, value: i32) {
        self.yield_stress = value;
    }
    /// Sets the fracture of the material
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set
    pub fn set_fracture(&mut self, value: i32) {
        self.fracture = value;
    }
    /// Sets the elasticity of the material
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set
    pub fn set_elasticity(&mut self, value: i32) {
        self.elasticity = value;
    }
}
