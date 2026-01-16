//! Material type tags

use crate::traits::IsEmpty;

/// A material template
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
pub enum MaterialTypeTag {
    /// An inorganic material
    Inorganic,
    /// A stone material
    Stone,
    /// A metal material
    Metal,
    /// A coal material
    Coal,
    /// A creature material
    CreatureMaterial,
    /// A creature material for the current creature token only
    LocalCreatureMaterial,
    /// A plant material
    PlantMaterial,
    /// A plant material for the current plant token only
    LocalPlantMaterial,
    /// A material from a reaction
    GetMaterialFromReagent,
    // Special "Hardcoded" Materials
    // Inorganic -> Magma
    /// Amber
    Amber,
    /// Coral
    Coral,
    /// Green Glass
    GlassGreen,
    /// Clear Glass
    GlassClear,
    /// Crystal Glass
    GlassCrystal,
    /// Water
    Water,
    // Coal -> Coal
    /// Potash
    Potash,
    /// Ash
    Ash,
    /// Pearl Ash
    PearlAsh,
    /// Lye
    Lye,
    /// Mud
    Mud,
    /// Vomit
    Vomit,
    /// Salt
    Salt,
    /// Brown Filth
    FilthB,
    /// Yellow Filth
    FilthY,
    /// Unnknown Substance
    UnknownSubstance,
    /// Grime
    Grime,
    /// An unknown token
    #[default]
    Unknown,
}

impl MaterialTypeTag {
    /// Returns true if the material type is the default value
    ///
    /// # Returns
    ///
    /// * `true` if the material type is `Self::Unknown`
    #[must_use]
    pub const fn is_default(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

impl std::fmt::Display for MaterialTypeTag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Inorganic => write!(f, "Inorganic"),
            Self::Stone => write!(f, "Stone"),
            Self::Metal => write!(f, "Metal"),
            Self::Coal => write!(f, "Coal"),
            Self::CreatureMaterial => write!(f, "Creature Material"),
            Self::LocalCreatureMaterial => write!(f, "Creature Material (Local)"),
            Self::PlantMaterial => write!(f, "Plant Material"),
            Self::LocalPlantMaterial => write!(f, "Plant Material (Local)"),
            Self::GetMaterialFromReagent => write!(f, "Get Material From Reagent"),
            Self::Amber => write!(f, "Amber"),
            Self::Coral => write!(f, "Coral"),
            Self::GlassGreen => write!(f, "Green Glass"),
            Self::GlassClear => write!(f, "Clear Glass"),
            Self::GlassCrystal => write!(f, "Crystal Glass"),
            Self::Water => write!(f, "Water"),
            Self::Potash => write!(f, "Potash"),
            Self::Ash => write!(f, "Ash"),
            Self::PearlAsh => write!(f, "Pearl Ash"),
            Self::Lye => write!(f, "Lye"),
            Self::Mud => write!(f, "Mud"),
            Self::Vomit => write!(f, "Vomit"),
            Self::Salt => write!(f, "Salt"),
            Self::FilthB => write!(f, "Brown Filth"),
            Self::FilthY => write!(f, "Yellow Filth"),
            Self::UnknownSubstance => write!(f, "Unknown Substance"),
            Self::Grime => write!(f, "Grime"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl IsEmpty for MaterialTypeTag {
    fn is_empty(&self) -> bool {
        self == &MaterialTypeTag::Unknown
    }
}
