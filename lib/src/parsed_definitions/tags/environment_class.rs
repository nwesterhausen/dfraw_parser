//! The class of environment that the stone appears in.

/// The class of environment that the stone appears in.
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
pub enum EnvironmentClassTag {
    /// Will appear in every stone.
    AllStone,
    /// Will appear in all igneous layers
    IgneousAll,
    /// Will appear in igneous extrusive layers
    IgneousExtrusive,
    /// Will appear in igneous intrusive layers
    IgneousIntrusive,
    /// Will appear in soil.
    Soil,
    /// Will appear in sand.
    SoilSand,
    /// Will appear in soil in the oceans.
    SoilOcean,
    /// Will appear in sedimentary layers.
    Sedimentary,
    /// Will appear in metamorphic layers.
    Metamorphic,
    /// Will appear in alluvial layers.
    Alluvial,
    /// Default value means parsing error.
    #[default]
    None,
}

impl EnvironmentClassTag {
    /// Whether the environment class is the default value.
    ///
    /// # Returns
    ///
    /// True if the environment class is the default value, false otherwise.
    #[must_use]
    pub fn is_default(&self) -> bool {
        *self == Self::None
    }
}

impl std::fmt::Display for EnvironmentClassTag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AllStone => write!(f, "AllStone"),
            Self::IgneousAll => write!(f, "IgneousAll"),
            Self::IgneousExtrusive => write!(f, "IgneousExtrusive"),
            Self::IgneousIntrusive => write!(f, "IgneousIntrusive"),
            Self::Soil => write!(f, "Soil"),
            Self::SoilSand => write!(f, "SoilSand"),
            Self::SoilOcean => write!(f, "SoilOcean"),
            Self::Sedimentary => write!(f, "Sedimentary"),
            Self::Metamorphic => write!(f, "Metamorphic"),
            Self::Alluvial => write!(f, "Alluvial"),
            Self::None => write!(f, "None"),
        }
    }
}
