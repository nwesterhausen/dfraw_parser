//! Inclusion type tag.

/// The type of inclusion that the stone has.
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
pub enum InclusionTypeTag {
    /// Large ovoids that occupy their entire 48x48 embark tile. Microcline is an example. When mined, stone has a 25% yield (as with layer stones).
    Cluster,
    /// Blobs of 3-9 tiles. Will always be successfully mined. Red pyropes are an example. When mined, stone has a 100% yield.
    ClusterSmall,
    /// Single tiles. Will always be successfully mined. Clear diamonds are an example. When mined, stone has a 100% yield.
    ClusterOne,
    /// Large streaks of stone. Native gold is an example. When mined, stone has a 33% yield instead of the usual 25%.
    Vein,
    /// Default value means parsing error.
    #[default]
    None,
}

impl InclusionTypeTag {
    /// Whether the inclusion type is the default value.
    ///
    /// # Returns
    ///
    /// True if the inclusion type is the default value, false otherwise.
    #[must_use]
    pub fn is_default(&self) -> bool {
        *self == Self::None
    }
}

impl std::fmt::Display for InclusionTypeTag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Cluster => write!(f, "Cluster"),
            Self::ClusterSmall => write!(f, "Small Cluster"),
            Self::ClusterOne => write!(f, "Singular Cluster"),
            Self::Vein => write!(f, "Vein"),
            Self::None => write!(f, "None"),
        }
    }
}
