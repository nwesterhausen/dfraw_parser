//! Temperature properties of a material
use dfraw_parser_proc_macros::{Cleanable, IsEmpty};

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
    Cleanable,
)]
#[serde(rename_all = "camelCase")]
/// The temperature properties of a material
pub struct Temperatures {
    /// This determines how long it takes the material to heat up or cool down.
    /// A material with a high specific heat capacity will hold more heat and affect its surroundings more
    /// before cooling down or heating up to equilibrium. The input for this token is not temperature,
    /// but rather the specific heat capacity of the material.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    specific_heat: Option<u32>,
    /// This is the temperature at which the material will catch fire.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    ignition_point: Option<u32>,
    /// This is the temperature at which a liquid material will freeze, or a solid material will melt.
    /// In Dwarf Fortress the melting point and freezing point coincide exactly; this is contrary to many
    /// real-life materials, which can be supercooled.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    melting_point: Option<u32>,
    /// This is the temperature at which the material will boil or condense. Water boils at 10180 °U
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    boiling_point: Option<u32>,
    /// This is the temperature above which the material will begin to take heat damage.
    /// Burning items without a heat damage point (or with an exceptionally high one) will take damage very slowly,
    /// causing them to burn for a very long time (9 months and 16.8 days) before disappearing.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    heat_damage_point: Option<u32>,
    /// This is the temperature below which the material will begin to take frost damage.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    cold_damage_point: Option<u32>,
    /// A material's temperature can be forced to always be a certain value via the `MAT_FIXED_TEMP`
    /// material definition token. The only standard material which uses this is nether-cap wood,
    /// whose temperature is always at the melting point of water. If a material's temperature is fixed
    /// to between its cold damage point and its heat damage point, then items made from that material
    /// will never suffer cold/heat damage. This makes nether-caps fire-safe and magma-safe despite being a type of wood.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    material_fixed_temperature: Option<u32>,
}

impl Temperatures {
    /// Returns whether the temperatures are empty
    ///
    /// # Returns
    ///
    /// * `bool` - Whether the temperatures are empty (default values)
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.specific_heat.is_none()
            && self.ignition_point.is_none()
            && self.melting_point.is_none()
            && self.boiling_point.is_none()
            && self.heat_damage_point.is_none()
            && self.cold_damage_point.is_none()
            && self.material_fixed_temperature.is_none()
    }
    /// Updates the specific heat of the material
    ///
    /// # Arguments
    ///
    /// * `value` - The specific heat to set
    pub fn update_specific_heat(&mut self, value: u32) {
        self.specific_heat = Some(value);
    }
    /// Updates the ignition point of the material
    ///
    /// # Arguments
    ///
    /// * `value` - The ignition point to set
    pub fn update_ignition_point(&mut self, value: u32) {
        self.ignition_point = Some(value);
    }
    /// Updates the melting point of the material
    ///
    /// # Arguments
    ///
    /// * `value` - The melting point to set
    pub fn update_melting_point(&mut self, value: u32) {
        self.melting_point = Some(value);
    }
    /// Updates the boiling point of the material
    ///
    /// # Arguments
    ///
    /// * `value` - The boiling point to set
    pub fn update_boiling_point(&mut self, value: u32) {
        self.boiling_point = Some(value);
    }
    /// Updates the heat damage point of the material
    ///
    /// # Arguments
    ///
    /// * `value` - The heat damage point to set
    pub fn update_heat_damage_point(&mut self, value: u32) {
        self.heat_damage_point = Some(value);
    }
    /// Updates the cold damage point of the material
    ///
    /// # Arguments
    ///
    /// * `value` - The cold damage point to set
    pub fn update_cold_damage_point(&mut self, value: u32) {
        self.cold_damage_point = Some(value);
    }
    /// Updates the material fixed temperature
    ///
    /// # Arguments
    ///
    /// * `value` - The material fixed temperature to set
    pub fn update_material_fixed_temperature(&mut self, value: u32) {
        self.material_fixed_temperature = Some(value);
    }
}
