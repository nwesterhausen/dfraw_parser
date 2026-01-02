use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

/// A map of the object tokens to their respective object types.
pub static OBJECT_TOKEN_MAP: phf::Map<&'static str, ObjectType> = phf::phf_map! {
    "CREATURE" => ObjectType::Creature,
    "INORGANIC" => ObjectType::Inorganic,
    "PLANT" => ObjectType::Plant,
    "ITEM" => ObjectType::Item,
    "ITEM_AMMO" => ObjectType::ItemAmmo,
    "ITEM_ARMOR" => ObjectType::ItemArmor,
    "ITEM_FOOD" => ObjectType::ItemFood,
    "ITEM_GLOVES" => ObjectType::ItemGloves,
    "ITEM_HELM" => ObjectType::ItemHelm,
    "ITEM_INSTRUMENT" => ObjectType::ItemInstrument,
    "ITEM_PANTS" => ObjectType::ItemPants,
    "ITEM_SHIELD" => ObjectType::ItemShield,
    "ITEM_SHOES" => ObjectType::ItemShoes,
    "ITEM_SIEGEAMMO" => ObjectType::ItemSiegeAmmo,
    "ITEM_TOOL" => ObjectType::ItemTool,
    "ITEM_TOY" => ObjectType::ItemToy,
    "ITEM_TRAPCOMP" => ObjectType::ItemTrapComponent,
    "ITEM_WEAPON" => ObjectType::ItemWeapon,
    "BUILDING" => ObjectType::Building,
    "BUILDING_WORKSHOP" => ObjectType::BuildingWorkshop,
    "BUILDING_FURNACE" => ObjectType::BuildingFurnace,
    "REACTION" => ObjectType::Reaction,
    "GRAPHICS" => ObjectType::Graphics,
    "MATERIAL_TEMPLATE" => ObjectType::MaterialTemplate,
    "BODY_DETAIL_PLAN" => ObjectType::BodyDetailPlan,
    "BODY" => ObjectType::Body,
    "ENTITY" => ObjectType::Entity,
    "LANGUAGE" => ObjectType::Language,
    "TRANSLATION" => ObjectType::Translation,
    "TISSUE_TEMPLATE" => ObjectType::TissueTemplate,
    "CREATURE_VARIATION" => ObjectType::CreatureVariation,
    "TEXT_SET" => ObjectType::TextSet,
    "TILE_PAGE" => ObjectType::TilePage,
    "DESCRIPTOR_COLOR" => ObjectType::DescriptorColor,
    "DESCRIPTOR_PATTERN" => ObjectType::DescriptorPattern,
    "DESCRIPTOR_SHAPE" => ObjectType::DescriptorShape,
    "PALETTE" => ObjectType::Palette,
    "MUSIC" => ObjectType::Music,
    "SOUND" => ObjectType::Sound,
    "INTERACTION" => ObjectType::Interaction,
};

/// The various types of objects that are within the raw files.
#[derive(
    Debug,
    Clone,
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
    Creature,
    /// An inorganic material
    Inorganic,
    /// A plant
    Plant,
    /// An item
    Item,
    /// An item of type ammo
    ItemAmmo,
    /// An item of type armor
    ItemArmor,
    /// An item of type food
    ItemFood,
    /// An item of type gloves
    ItemGloves,
    /// An item of type helm
    ItemHelm,
    /// An item of type instrument
    ItemInstrument,
    /// An item of type pants
    ItemPants,
    /// An item of type shield
    ItemShield,
    /// An item of type shoes
    ItemShoes,
    /// An item of type siege ammo
    ItemSiegeAmmo,
    /// An item of type tool
    ItemTool,
    /// An item of type toy
    ItemToy,
    /// An item of type trap component
    ItemTrapComponent,
    /// An item of type weapon
    ItemWeapon,
    /// A building
    Building,
    /// A workshop building
    BuildingWorkshop,
    /// A furnace building
    BuildingFurnace,
    /// A reaction
    Reaction,
    /// Graphics
    Graphics,
    /// A material template
    MaterialTemplate,
    /// A body detail plan
    BodyDetailPlan,
    /// A body
    Body,
    /// An entity
    Entity,
    /// A language
    Language,
    /// A translation
    Translation,
    /// A tissue template
    TissueTemplate,
    /// A creature variation
    CreatureVariation,
    /// A text set
    TextSet,
    /// A tile page
    TilePage,
    /// A descriptor color
    DescriptorColor,
    /// A descriptor pattern
    DescriptorPattern,
    /// A descriptor shape
    DescriptorShape,
    /// A palette
    Palette,
    /// Music
    Music,
    /// Sound
    Sound,
    /// An interaction
    Interaction,
    /// An unknown object type
    #[default]
    Unknown,
    /// `SelectCreature` tag
    SelectCreature,
    /// A creature caste
    CreatureCaste,
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
        }
    }
}
