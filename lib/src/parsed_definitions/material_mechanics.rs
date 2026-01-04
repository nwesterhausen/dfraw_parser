//! Contains the `MaterialMechanics` struct and associated functions.

use tracing::warn;

use crate::{
    default_checks, mechanical_properties::MechanicalProperties, tags::MaterialPropertyTag,
};

/// Represents the specific yield, fracture, and elasticity of a material for the various
/// types of mechanical stress.
#[allow(clippy::module_name_repetitions)]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct MaterialMechanics {
    #[serde(skip_serializing_if = "Option::is_none")]
    impact: Option<MechanicalProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compressive: Option<MechanicalProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tensile: Option<MechanicalProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    torsion: Option<MechanicalProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shear: Option<MechanicalProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bending: Option<MechanicalProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_edge: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub fn parse_tag(&mut self, key: &MaterialPropertyTag, value: &str) {
        match key {
            MaterialPropertyTag::ImpactYield => {
                if self.impact.is_none() {
                    self.impact = Some(MechanicalProperties::new());
                }
                if let Some(impact) = &mut self.impact {
                    impact.set_yield(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyTag::ImpactFracture => {
                if self.impact.is_none() {
                    self.impact = Some(MechanicalProperties::new());
                }
                if let Some(impact) = &mut self.impact {
                    impact.set_fracture(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyTag::ImpactElasticity => {
                if self.impact.is_none() {
                    self.impact = Some(MechanicalProperties::new());
                }
                if let Some(impact) = &mut self.impact {
                    impact.set_elasticity(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyTag::CompressiveYield => {
                if self.compressive.is_none() {
                    self.compressive = Some(MechanicalProperties::new());
                }
                if let Some(compressive) = &mut self.compressive {
                    compressive.set_yield(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyTag::CompressiveFracture => {
                if self.compressive.is_none() {
                    self.compressive = Some(MechanicalProperties::new());
                }
                if let Some(compressive) = &mut self.compressive {
                    compressive.set_fracture(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyTag::CompressiveElasticity => {
                if self.compressive.is_none() {
                    self.compressive = Some(MechanicalProperties::new());
                }
                if let Some(compressive) = &mut self.compressive {
                    compressive.set_elasticity(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyTag::TensileYield => {
                if self.tensile.is_none() {
                    self.tensile = Some(MechanicalProperties::new());
                }
                if let Some(tensile) = &mut self.tensile {
                    tensile.set_yield(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyTag::TensileFracture => {
                if self.tensile.is_none() {
                    self.tensile = Some(MechanicalProperties::new());
                }
                if let Some(tensile) = &mut self.tensile {
                    tensile.set_fracture(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyTag::TensileElasticity => {
                if self.tensile.is_none() {
                    self.tensile = Some(MechanicalProperties::new());
                }
                if let Some(tensile) = &mut self.tensile {
                    tensile.set_elasticity(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyTag::TorsionYield => {
                if self.torsion.is_none() {
                    self.torsion = Some(MechanicalProperties::new());
                }
                if let Some(torsion) = &mut self.torsion {
                    torsion.set_yield(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyTag::TorsionFracture => {
                if self.torsion.is_none() {
                    self.torsion = Some(MechanicalProperties::new());
                }
                if let Some(torsion) = &mut self.torsion {
                    torsion.set_fracture(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyTag::TorsionElasticity => {
                if self.torsion.is_none() {
                    self.torsion = Some(MechanicalProperties::new());
                }
                if let Some(torsion) = &mut self.torsion {
                    torsion.set_elasticity(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyTag::ShearYield => {
                if self.shear.is_none() {
                    self.shear = Some(MechanicalProperties::new());
                }
                if let Some(shear) = &mut self.shear {
                    shear.set_yield(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyTag::ShearFracture => {
                if self.shear.is_none() {
                    self.shear = Some(MechanicalProperties::new());
                }
                if let Some(shear) = &mut self.shear {
                    shear.set_fracture(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyTag::ShearElasticity => {
                if self.shear.is_none() {
                    self.shear = Some(MechanicalProperties::new());
                }
                if let Some(shear) = &mut self.shear {
                    shear.set_elasticity(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyTag::BendingYield => {
                if self.bending.is_none() {
                    self.bending = Some(MechanicalProperties::new());
                }
                if let Some(bending) = &mut self.bending {
                    bending.set_yield(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyTag::BendingFracture => {
                if self.bending.is_none() {
                    self.bending = Some(MechanicalProperties::new());
                }
                if let Some(bending) = &mut self.bending {
                    bending.set_fracture(value.parse::<i32>().unwrap_or(0));
                }
            }
            MaterialPropertyTag::BendingElasticity => {
                if self.bending.is_none() {
                    self.bending = Some(MechanicalProperties::new());
                }
                if let Some(bending) = &mut self.bending {
                    bending.set_elasticity(value.parse::<i32>().unwrap_or(0));
                }
            }

            MaterialPropertyTag::MaxEdge => {
                self.max_edge = Some(value.parse::<i32>().unwrap_or(0));
            }
            MaterialPropertyTag::SolidDensity => {
                self.solid_density = Some(value.parse::<i32>().unwrap_or(0));
            }

            _ => {
                warn!("Unhandled material mechanics token: '{:?}'", key);
            }
        }
    }
    /// Function to "clean" the raw. This is used to remove any empty list or strings,
    /// and to remove any default values. By "removing" it means setting the value to None.
    ///
    /// This also will remove the metadata if `is_metadata_hidden` is true.
    ///
    /// Steps for all "Option" fields:
    /// - Set any metadata to None if `is_metadata_hidden` is true.
    /// - Set any empty string to None.
    /// - Set any empty list to None.
    /// - Set any default values to None.
    #[must_use]
    pub fn cleaned(&self) -> Self {
        let mut cleaned = self.clone();

        if let Some(impact) = &cleaned.impact
            && impact.is_empty()
        {
            cleaned.impact = None;
        }
        if let Some(compressive) = &cleaned.compressive
            && compressive.is_empty()
        {
            cleaned.compressive = None;
        }
        if let Some(tensile) = &cleaned.tensile
            && tensile.is_empty()
        {
            cleaned.tensile = None;
        }
        if let Some(torsion) = &cleaned.torsion
            && torsion.is_empty()
        {
            cleaned.torsion = None;
        }
        if let Some(shear) = &cleaned.shear
            && shear.is_empty()
        {
            cleaned.shear = None;
        }
        if let Some(bending) = &cleaned.bending
            && bending.is_empty()
        {
            cleaned.bending = None;
        }

        if default_checks::is_zero_i32(cleaned.max_edge) {
            cleaned.max_edge = None;
        }
        if default_checks::is_zero_i32(cleaned.solid_density) {
            cleaned.solid_density = None;
        }

        cleaned
    }
}
