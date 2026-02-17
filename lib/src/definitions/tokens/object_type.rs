use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

use crate::traits::IsEmpty;

/// The various types of objects that are within the raw files.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    Default,
    Hash,
    specta::Type,
    strum_macros::EnumIter,
)]
pub enum ObjectType {
    /// A creature
    Creature = 1,
    /// An inorganic material
    Inorganic = 2,
    /// A plant
    Plant = 3,
    /// An item
    Item = 4,
    /// An item of type ammo
    ItemAmmo = 5,
    /// An item of type armor
    ItemArmor = 6,
    /// An item of type food
    ItemFood = 7,
    /// An item of type gloves
    ItemGloves = 8,
    /// An item of type helm
    ItemHelm = 9,
    /// An item of type instrument
    ItemInstrument = 10,
    /// An item of type pants
    ItemPants = 11,
    /// An item of type shield
    ItemShield = 12,
    /// An item of type shoes
    ItemShoes = 13,
    /// An item of type siege ammo
    ItemSiegeAmmo = 14,
    /// An item of type tool
    ItemTool = 15,
    /// An item of type toy
    ItemToy = 16,
    /// An item of type trap component
    ItemTrapComponent = 17,
    /// An item of type weapon
    ItemWeapon = 18,
    /// A building
    Building = 19,
    /// A workshop building
    BuildingWorkshop = 20,
    /// A furnace building
    BuildingFurnace = 21,
    /// A reaction
    Reaction = 22,
    /// Graphics
    Graphics = 23,
    /// A material template
    MaterialTemplate = 24,
    /// A body detail plan
    BodyDetailPlan = 25,
    /// A body
    Body = 26,
    /// An entity
    Entity = 27,
    /// A language
    Language = 28,
    /// A translation
    Translation = 29,
    /// A tissue template
    TissueTemplate = 30,
    /// A creature variation
    CreatureVariation = 31,
    /// A text set
    TextSet = 32,
    /// A tile page
    TilePage = 33,
    /// A descriptor color
    DescriptorColor = 34,
    /// A descriptor pattern
    DescriptorPattern = 35,
    /// A descriptor shape
    DescriptorShape = 36,
    /// A palette
    Palette = 37,
    /// Music
    Music = 38,
    /// Sound
    Sound = 39,
    /// An interaction
    Interaction = 40,
    /// An unknown object type
    #[default]
    Unknown = 99,
    /// `SelectCreature` tag
    SelectCreature = 41,
    /// A creature caste
    CreatureCaste = 42,
    /// A module's info.txt file
    ModuleInfo = 43,
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Creature => write!(f, "Creature"),
            Self::Inorganic => write!(f, "Inorganic"),
            Self::Plant => write!(f, "Plant"),
            Self::Item => write!(f, "Item"),
            Self::ItemAmmo => write!(f, "Ammo (Item)"),
            Self::ItemArmor => write!(f, "Armor (Item)"),
            Self::ItemFood => write!(f, "Food (Item)"),
            Self::ItemGloves => write!(f, "Gloves (Item)"),
            Self::ItemHelm => write!(f, "Helm (Item)"),
            Self::ItemInstrument => write!(f, "Instrument (Item)"),
            Self::ItemPants => write!(f, "Pants (Item)"),
            Self::ItemShield => write!(f, "Shield (Item)"),
            Self::ItemShoes => write!(f, "Shoes (Item)"),
            Self::ItemSiegeAmmo => write!(f, "Siege Ammo (Item)"),
            Self::ItemTool => write!(f, "Tool (Item)"),
            Self::ItemToy => write!(f, "Toy (Item)"),
            Self::ItemTrapComponent => write!(f, "Trap Component (Item)"),
            Self::ItemWeapon => write!(f, "Weapon (Item)"),
            Self::Building => write!(f, "Building"),
            Self::BuildingWorkshop => write!(f, "Workshop Building"),
            Self::BuildingFurnace => write!(f, "Furnace Building"),
            Self::Reaction => write!(f, "Reaction"),
            Self::Graphics => write!(f, "Graphics"),
            Self::MaterialTemplate => write!(f, "Material Template"),
            Self::BodyDetailPlan => write!(f, "Body Detail Plan"),
            Self::Body => write!(f, "Body"),
            Self::Entity => write!(f, "Entity"),
            Self::Language => write!(f, "Language"),
            Self::Translation => write!(f, "Translation"),
            Self::TissueTemplate => write!(f, "Tissue Template"),
            Self::CreatureVariation => write!(f, "Creature Variation"),
            Self::TextSet => write!(f, "Text Set"),
            Self::TilePage => write!(f, "Tile Page"),
            Self::DescriptorColor => write!(f, "Color Descriptor"),
            Self::DescriptorPattern => write!(f, "Pattern Descriptor"),
            Self::DescriptorShape => write!(f, "Shape Descriptor"),
            Self::Palette => write!(f, "Palette"),
            Self::Music => write!(f, "Music"),
            Self::Sound => write!(f, "Sound"),
            Self::Interaction => write!(f, "Interaction"),
            Self::Unknown => write!(f, "Unknown"),
            Self::SelectCreature => write!(f, "Select Creature"),
            Self::CreatureCaste => write!(f, "Creature Caste"),
            Self::ModuleInfo => write!(f, "Module Info"),
        }
    }
}

impl IsEmpty for ObjectType {
    fn is_empty(&self) -> bool {
        self == &ObjectType::Unknown
    }
}

impl From<ObjectType> for u32 {
    fn from(value: ObjectType) -> Self {
        value as u32
    }
}

impl From<&ObjectType> for u32 {
    fn from(value: &ObjectType) -> Self {
        u32::from(*value)
    }
}
