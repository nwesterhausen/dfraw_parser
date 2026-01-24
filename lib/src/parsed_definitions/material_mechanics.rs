//! Contains the `MaterialMechanics` struct and associated functions.

use dfraw_parser_proc_macros::IsEmpty;
use tracing::warn;

use crate::{MechanicalProperties, tokens::MaterialPropertyToken};

/// Represents the specific yield, fracture, and elasticity of a material for the various
/// types of mechanical stress.
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
pub struct MaterialMechanics {
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    impact: Option<MechanicalProperties>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    compressive: Option<MechanicalProperties>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    tensile: Option<MechanicalProperties>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    torsion: Option<MechanicalProperties>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    shear: Option<MechanicalProperties>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    bending: Option<MechanicalProperties>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    max_edge: Option<i32>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    solid_density: Option<i32>,
}

impl MaterialMechanics {
    /// Creates a new `Mechanics` struct with default values.
    ///
    /// # Returns
    ///
    /// * The new `Mechanics` struct.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Returns whether the `Mechanics` struct is empty.
    ///
    /// # Returns
    ///
    /// * `true` if the `Mechanics` struct is empty, `false` otherwise.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.impact.is_none()
            && self.compressive.is_none()
            && self.tensile.is_none()
            && self.torsion.is_none()
            && self.shear.is_none()
            && self.bending.is_none()
    }
    /// Parses a tag and value into the `Mechanics` struct.
    ///
    /// # Arguments
    ///
    /// * `key` - The tag to parse.
    /// * `value` - The value to parse.
    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    pub fn parse_tag(&mut self, key: &MaterialPropertyToken, value: &str) {
        match key {
            MaterialPropertyToken::ImpactYield => {
                if self.impact.is_none() {
                    self.impact = Some(MechanicalProperties::new());
                }
                if let Some(impact) = &mut self.impact {
                    impact.set_yield(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyToken::ImpactFracture => {
                if self.impact.is_none() {
                    self.impact = Some(MechanicalProperties::new());
                }
                if let Some(impact) = &mut self.impact {
                    impact.set_fracture(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyToken::ImpactElasticity => {
                if self.impact.is_none() {
                    self.impact = Some(MechanicalProperties::new());
                }
                if let Some(impact) = &mut self.impact {
                    impact.set_elasticity(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyToken::CompressiveYield => {
                if self.compressive.is_none() {
                    self.compressive = Some(MechanicalProperties::new());
                }
                if let Some(compressive) = &mut self.compressive {
                    compressive.set_yield(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyToken::CompressiveFracture => {
                if self.compressive.is_none() {
                    self.compressive = Some(MechanicalProperties::new());
                }
                if let Some(compressive) = &mut self.compressive {
                    compressive.set_fracture(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyToken::CompressiveElasticity => {
                if self.compressive.is_none() {
                    self.compressive = Some(MechanicalProperties::new());
                }
                if let Some(compressive) = &mut self.compressive {
                    compressive.set_elasticity(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyToken::TensileYield => {
                if self.tensile.is_none() {
                    self.tensile = Some(MechanicalProperties::new());
                }
                if let Some(tensile) = &mut self.tensile {
                    tensile.set_yield(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyToken::TensileFracture => {
                if self.tensile.is_none() {
                    self.tensile = Some(MechanicalProperties::new());
                }
                if let Some(tensile) = &mut self.tensile {
                    tensile.set_fracture(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyToken::TensileElasticity => {
                if self.tensile.is_none() {
                    self.tensile = Some(MechanicalProperties::new());
                }
                if let Some(tensile) = &mut self.tensile {
                    tensile.set_elasticity(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyToken::TorsionYield => {
                if self.torsion.is_none() {
                    self.torsion = Some(MechanicalProperties::new());
                }
                if let Some(torsion) = &mut self.torsion {
                    torsion.set_yield(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyToken::TorsionFracture => {
                if self.torsion.is_none() {
                    self.torsion = Some(MechanicalProperties::new());
                }
                if let Some(torsion) = &mut self.torsion {
                    torsion.set_fracture(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyToken::TorsionElasticity => {
                if self.torsion.is_none() {
                    self.torsion = Some(MechanicalProperties::new());
                }
                if let Some(torsion) = &mut self.torsion {
                    torsion.set_elasticity(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyToken::ShearYield => {
                if self.shear.is_none() {
                    self.shear = Some(MechanicalProperties::new());
                }
                if let Some(shear) = &mut self.shear {
                    shear.set_yield(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyToken::ShearFracture => {
                if self.shear.is_none() {
                    self.shear = Some(MechanicalProperties::new());
                }
                if let Some(shear) = &mut self.shear {
                    shear.set_fracture(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyToken::ShearElasticity => {
                if self.shear.is_none() {
                    self.shear = Some(MechanicalProperties::new());
                }
                if let Some(shear) = &mut self.shear {
                    shear.set_elasticity(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyToken::BendingYield => {
                if self.bending.is_none() {
                    self.bending = Some(MechanicalProperties::new());
                }
                if let Some(bending) = &mut self.bending {
                    bending.set_yield(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyToken::BendingFracture => {
                if self.bending.is_none() {
                    self.bending = Some(MechanicalProperties::new());
                }
                if let Some(bending) = &mut self.bending {
                    bending.set_fracture(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyToken::BendingElasticity => {
                if self.bending.is_none() {
                    self.bending = Some(MechanicalProperties::new());
                }
                if let Some(bending) = &mut self.bending {
                    bending.set_elasticity(value.parse::<i32>().unwrap_or(0));
                }
            }

            MaterialPropertyToken::MaxEdge => {
                self.max_edge = Some(value.parse::<i32>().unwrap_or(0));
            }
            MaterialPropertyToken::SolidDensity => {
                self.solid_density = Some(value.parse::<i32>().unwrap_or(0));
            }

            _ => {
                warn!("Unhandled material mechanics token: '{:?}'", key);
            }
        }
    }
}
