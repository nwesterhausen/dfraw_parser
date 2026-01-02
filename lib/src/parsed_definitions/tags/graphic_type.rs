//! Graphic type tags for the tileset

/// The graphic type of the tile
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
#[serde(rename_all = "camelCase")]
pub enum GraphicTypeTag {
    /// The tile is a creature
    Creature,
    /// The tile is a creature caste
    CreatureCaste,
    /// The tile is a statue of a creature
    StatueCreature,
    /// The tile is a statue of a creature caste
    StatueCreatureCaste,
    /// The tile is a statue of a creature caste with a giant size
    StatuesSurfaceGiant,
    /// A tile
    Tile,
    /// An empty tile
    Empty,
    /// A plant
    Plant,
    /// An unknown type
    #[default]
    Unknown,
    /// A template
    Template,
    /// The tile is soil background
    SoilBackground,
    /// The tile is grass-1
    Grass1,
    /// The tile is grass-2
    Grass2,
    /// The tile is grass-3
    Grass3,
    /// The tile is grass-4
    Grass4,
    /// The tile is custom edging
    CustomEdging,
    /// The tile is custom ramp
    CustomRamp,
    /// The tile is custom corner (W)
    CustomEdgeW,
    /// The tile is custom corner (E)
    CustomEdgeE,
    /// The tile is custom corner (N)
    CustomEdgeN,
    /// The tile is custom corner (S)
    CustomEdgeS,
    /// The tile is custom corner (NW)
    CustomEdgeNW,
    /// The tile is custom corner (NE)
    CustomEdgeNE,
    /// The tile is custom corner (SW)
    CustomEdgeSW,
    /// The tile is custom corner (SE)
    CustomEdgeSE,
    /// The tile is a custom workshop
    CustomWorkshop,
    /// The tile is a list icon
    ListIcon,
    /// The tile is an add tool
    AddTool,
    /// The tile is ammo
    Ammo,
    /// The tile is ammo straight default
    AmmoStraightDefault,
    /// The tile is ammo straight wood
    AmmoStraightWood,
    /// The tile is ammo diagonal default
    AmmoDiagonalDefault,
    /// The tile is ammo diagonal wood
    AmmoDiagonalWood,
    /// The tile is an armor
    Armor,
    /// The tile is food
    Food,
    /// The graphic is of gloves
    Gloves,
    /// The graphic is of a helm
    Helm,
    /// The graphic is of pants
    Pants,
    /// The graphic is of a rough gem
    RoughGem,
    /// The graphic is of a large gem
    ShapeLargeGem,
    /// The graphic is of a small gem
    ShapeSmallGem,
    /// The graphic is of a shield
    Shield,
    /// The graphic is of a wooden shield
    ShieldWooden,
    /// The graphic is of shoes
    Shoes,
    /// The graphic is of metal shoes
    ShoesMetal,
    /// The graphic is of siege ammo
    SiegeAmmo,
    /// The graphic is of siege ammo straight default
    SiegeAmmoStraightDefault,
    /// The graphic is of siege ammo straight wood
    SiegeAmmoStraightWood,
    /// The graphic is of siege ammo diagonal default
    SiegeAmmoDiagonalDefault,
    /// The graphic is of siege ammo diagonal wood
    SiegeAmmoDiagonalWood,
    /// The graphic is of a tool
    Tool,
    /// The graphic is of a tool wood
    ToolWood,
    /// The graphic is of a tool stone
    ToolStone,
    /// The graphic is of a tool metal
    ToolMetal,
    /// The graphic is of a beehive
    ToolHiveBuilding,
    /// The graphic is of a glass tool
    ToolGlass,
    /// The graphic is of a tool shape
    ToolShape,
    /// The graphic is a glass tool variant
    ToolGlassVariant,
    /// The graphic is a metal tool variant
    ToolMetalVariant,
    /// The graphic is a stone tool variant
    ToolStoneVariant,
    /// The graphic is a wood tool variant
    ToolWoodVariant,
    /// The graphic is a mud tool
    ToolMud,
    /// The graphic is a water tool
    ToolWater,
    /// The graphic is a vomit tool
    ToolVomit,
    /// The graphic is a blood tool
    ToolBlood,
    /// The graphic is a plant tool
    ToolDamage,
    /// The graphic is a tool with binds on it
    ToolBands,
    /// The graphic is a tool with engravings
    ToolEngraving,
    /// The graphic is a tool with studs
    ToolStuds,
    /// The graphic is a tool with rings
    ToolRings,
    /// The graphic is a tool with spikes
    ToolSpikes,
    /// The graphic is a toy
    Toy,
    /// The graphic is a trap component
    TrapComponent,
    /// The graphic is a weapon trap
    TrapComponentWeaponTrap,
    /// The graphic is a weapon trap upright 1-T
    TrapComponentUpright1T,
    /// The graphic is a weapon trap upright 2-T
    TrapComponentUpright2T,
    /// The graphic is a weapon trap upright 3-T
    TrapComponentUpright3T,
    /// The graphic is a weapon trap upright 4-T
    TrapComponentUpright4T,
    /// The graphic is a weapon trap upright 5-T
    TrapComponentUpright5T,
    /// The graphic is a weapon trap upright 6-T
    TrapComponentUpright6T,
    /// The graphic is a weapon trap upright 7-T
    TrapComponentUpright7T,
    /// The graphic is a weapon trap upright 8-T
    TrapComponentUpright8T,
    /// The graphic is a weapon trap upright 9-T
    TrapComponentUpright9T,
    /// The graphic is a weapon trap upright 10-T
    TrapComponentUpright10T,
    /// The graphic is a weapon trap upright 1-B
    TrapComponentUpright1B,
    /// The graphic is a weapon trap upright 2-B
    TrapComponentUpright2B,
    /// The graphic is a weapon trap upright 3-B
    TrapComponentUpright3B,
    /// The graphic is a weapon trap upright 4-B
    TrapComponentUpright4B,
    /// The graphic is a weapon trap upright 5-B
    TrapComponentUpright5B,
    /// The graphic is a weapon trap upright 6-B
    TrapComponentUpright6B,
    /// The graphic is a weapon trap upright 7-B
    TrapComponentUpright7B,
    /// The graphic is a weapon trap upright 8-B
    TrapComponentUpright8B,
    /// The graphic is a weapon trap upright 9-B
    TrapComponentUpright9B,
    /// The graphic is a weapon trap upright 10-B
    TrapComponentUpright10B,
    /// The graphic is a weapon
    Weapon,
    /// The graphic is a weapon default
    WeaponDefault,
    /// The graphic is a weapon made of wood
    WeaponWood,
    /// The graphic is a weapon made of grown wood
    WeaponWoodGrown,
    /// The graphic is a weapon made of material
    WeaponMaterial,
    /// The graphic is of a weapon used in traps
    WeaponTrap,
    /// The graphic is of a weapon upright 1-T
    WeaponUpright1T,
    /// The graphic is of a weapon upright 2-T
    WeaponUpright2T,
    /// The graphic is of a weapon upright 3-T
    WeaponUpright3T,
    /// The graphic is of a weapon upright 4-T
    WeaponUpright4T,
    /// The graphic is of a weapon upright 5-T
    WeaponUpright5T,
    /// The graphic is of a weapon upright 6-T
    WeaponUpright6T,
    /// The graphic is of a weapon upright 7-T
    WeaponUpright7T,
    /// The graphic is of a weapon upright 8-T
    WeaponUpright8T,
    /// The graphic is of a weapon upright 9-T
    WeaponUpright9T,
    /// The graphic is of a weapon upright 10-T
    WeaponUpright10T,
    /// The graphic is of a weapon upright 1-B
    WeaponUpright1B,
    /// The graphic is of a weapon upright 2-B
    WeaponUpright2B,
    /// The graphic is of a weapon upright 3-B
    WeaponUpright3B,
    /// The graphic is of a weapon upright 4-B
    WeaponUpright4B,
    /// The graphic is of a weapon upright 5-B
    WeaponUpright5B,
    /// The graphic is of a weapon upright 6-B
    WeaponUpright6B,
    /// The graphic is of a weapon upright 7-B
    WeaponUpright7B,
    /// The graphic is of a weapon upright 8-B
    WeaponUpright8B,
    /// The graphic is of a weapon upright 9-B
    WeaponUpright9B,
    /// The graphic is of a weapon upright 10-B
    WeaponUpright10B,
}
