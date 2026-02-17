//! A material fuel type that can be set in a material definition.

use crate::traits::IsEmpty;

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
pub enum FuelTypeToken {
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

impl FuelTypeToken {
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

impl std::fmt::Display for FuelTypeToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl IsEmpty for FuelTypeToken {
    fn is_empty(&self) -> bool {
        self == &FuelTypeToken::None
    }
}
