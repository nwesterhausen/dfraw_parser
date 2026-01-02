//! A material fuel type that can be set in a material definition.

/// A material fuel type that can be set in a material definition.
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Default,
    specta::Type,
    Copy,
    strum_macros::EnumIter,
)]
pub enum FuelTypeTag {
    /// Charcoal or coal
    Charcoal,
    /// Coal coke
    Coke,
    /// No glass furnace fuel
    NoMaterialGloss,
    /// None is an invalid option, so its a hint that this is not set.
    #[default]
    None,
}

impl FuelTypeTag {
    /// Returns true if the fuel type is the default value
    ///
    /// # Returns
    ///
    /// * `true` if the fuel type is `FuelType::None`
    #[must_use]
    pub const fn is_default(&self) -> bool {
        matches!(self, Self::None)
    }
}
