//! Material property tags.

/// A material property that can be set in a material definition.
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
pub enum MaterialPropertyTag {
    /// Imports the properties of the specified preexisting material template.
    UseMaterialTemplate,
    /// Applies a prefix to all items made from the material. For `PLANT` and `CREATURE` materials, this defaults to the plant/creature name.
    /// Not permitted in material template definitions.
    Prefix,
    /// Overrides the name of `BOULDER` items (i.e. mined-out stones) made of the material (used for native copper/silver/gold/platinum
    /// to make them be called "nuggets" instead of "boulders").
    StoneName,
    /// Used to indicate that said material is a gemstone - when tiles are mined out, rough gems will be yielded instead of boulders.
    /// Plural can be "STP" to automatically append an "s" to the singular form, and `OVERWRITE_SOLID` will override the relevant `STATE_NAME` and `STATE_ADJ` values.
    IsGem,
    /// Specifies what the material should be treated as when drinking water contaminated by it, for generating unhappy thoughts.
    /// Valid values are `BLOOD`, `SLIME`, `VOMIT`, `ICHOR`, `PUS`, `GOO`, `GRIME`, and `FILTH`.
    TempDietInfo,
    /// Allows the material to be used as dye, and defines color of dyed items.
    PowderDye,
    /// Specifies the tile that will be used to represent unmined tiles made of this material. Generally only used with stones. Defaults to 219 ('█').
    Tile,
    /// Specifies the tile that will be used to represent BOULDER items made of this material. Generally only used with stones. Defaults to 7 ('•').
    ItemSymbol,
    /// The on-screen color of the material. Uses a standard 3-digit color token. Equivalent to `TILE_COLOR:a:b:c`,
    /// `BUILD_COLOR:b:a:X` (X = 1 if 'a' equals 'b', 0 otherwise), and `BASIC_COLOR:a:c`
    DisplayColor,
    /// The color of objects made of this material which use both the foreground and background color: doors, floodgates, hatch covers, bins, barrels, and cages.
    /// Defaults to `7:7:1` (white).
    BuildColor,
    /// The color of unmined tiles containing this material (for stone and soil), as well as engravings in this material. Defaults to `7:7:1` (white).
    TileColor,
    /// The color of objects made of this material which use only the foreground color, including workshops, floors and boulders, and smoothed walls. Defaults to `7:1` (white).
    BasicColor,
    /// Determines the color of the material at the specified state. See below for a list of valid material states. Color comes from `descriptor_color_standard.txt`.
    /// The nearest color value is used to display contaminants and body parts made of this material in ASCII and to color items and constructions made from this
    /// material with graphics.
    /// Example:`STATE_COLOR:ALL_SOLID:GRAY`
    StateColor,
    /// Determines the name of the material at the specified state, as displayed in-game. `STATE_NAME:ALL_SOLID:stone`
    StateName,
    /// Like `STATE_NAME`, but used in different situations. Equipment made from the material uses the state adjective and not the state name.
    StateAdjective,
    /// Sets both `STATE_NAME` and `STATE_ADJ` at the same time.
    StateNameAdjective,
    /// The material's tendency to absorb liquids. Containers made of materials with nonzero absorption cannot hold liquids unless they have been glazed.
    /// Defaults to 0.
    Absorption,
    /// Specifies how hard of an impact (in kilopascals) the material can withstand before it will start deforming permanently.
    /// Used for blunt-force combat. Defaults to `10_000`.
    ImpactYield,
    /// Specifies how hard of an impact the material can withstand before it will fail entirely. Used for blunt-force combat. Defaults to `10_000`.
    ImpactFracture,
    /// Specifies how much the material will have given (in parts-per-`100_000`) when the yield point is reached. Used for blunt-force combat. Defaults to 0.
    /// Apparently affects in combat whether the corresponding tissue is bruised (`value >= 50_000`), torn (value between `25_000` and `49_999`), or fractured (`value <= 24_999`)
    ImpactElasticity,
    /// Specifies how hard the material can be compressed before it will start deforming permanently. Determines a tissue's resistance to pinching and response to strangulation.
    /// Defaults to `10_000`.
    CompressiveYield,
    /// Specifies how hard the material can be compressed before it will fail entirely. Determines a tissue's resistance to pinching and response to strangulation.
    /// Defaults to `10_000`.
    CompressiveFracture,
    /// Specifies how much the material will have given when it has been compressed to its yield point. Determines a tissue's resistance to pinching and
    /// response to strangulation. Defaults to 0.
    CompressiveElasticity,
    /// Specifies how hard the material can be stretched before it will start deforming permanently. Determines a tissue's resistance to a latching and tearing bite.
    /// Defaults to `10_000`.
    TensileYield,
    /// Specifies how hard the material can be stretched before it will fail entirely. Determines a tissue's resistance to a latching and tearing bite. Defaults to `10_000`.
    TensileFracture,
    /// Specifies how much the material will have given when it is stretched to its yield point. Determines a tissue's resistance to a latching and tearing bite.
    /// Defaults to 0.
    TensileElasticity,
    /// Specifies how hard the material can be twisted before it will start deforming permanently. Used for latching and shaking with a blunt attack
    /// (no default creature has such an attack, but they can be modded in). Defaults to `10_000`.
    TorsionYield,
    /// Specifies how hard the material can be twisted before it will fail entirely. Used for latching and shaking with a blunt attack
    /// (no default creature has such an attack, but they can be modded in). Defaults to `10_000`.
    TorsionFracture,
    /// Specifies how much the material will have given when it is twisted to its yield point. Used for latching and shaking with a blunt attack
    /// (no default creature has such an attack, but they can be modded in). Defaults to 0.
    TorsionElasticity,
    /// Specifies how hard the material can be sheared before it will start deforming permanently. Used for cutting calculations. Defaults to `10_000`.
    ShearYield,
    /// Specifies how hard the material can be sheared before it will fail entirely. Used for cutting calculations. Defaults to `10_000`.
    ShearFracture,
    /// Specifies how much the material will have given when sheared to its yield point. Used for cutting calculations. Defaults to 0.
    ShearElasticity,
    /// Specifies how hard the material can be bent before it will start deforming permanently. Determines a tissue's resistance to being mangled with a joint lock.
    /// Defaults to `10_000`.
    BendingYield,
    /// Specifies how hard the material can be bent before it will fail entirely. Determines a tissue's resistance to being mangled with a joint lock. Defaults to `10_000`.
    BendingFracture,
    /// Specifies how much the material will have given when bent to its yield point. Determines a tissue's resistance to being mangled with a joint lock. Defaults to 0.
    BendingElasticity,
    /// How sharp the material is. Used in cutting calculations. Applying a value of at least `10_000` to a stone will allow weapons to be made from that stone. Defaults to `10_000`.
    MaxEdge,
    /// Value modifier for the material. Defaults to 1. This number can be made negative by placing a "-" in front, resulting in things that you are paid to buy and
    /// must pay to sell.
    MaterialValue,
    /// Multiplies the value of the material. Not permitted in material template definitions.
    MultiplyValue,
    /// Rate at which the material heats up or cools down (in joules/kilogram-kelvin). If set to `NONE`, the temperature will be fixed at its initial value.
    /// Defaults to `NONE`.
    SpecificHeat,
    /// Temperature above which the material takes damage from heat. Defaults to `NONE`.
    /// If the material has an ignite point but no heatdam point, it will burn for a very long time (9 months and 16.8 days).
    HeatDamagePoint,
    /// Temperature below which the material takes damage from cold. Defaults to `NONE`.
    ColdDamagePoint,
    /// Temperature at which the material will catch fire. Defaults to `NONE`.
    IgnitionPoint,
    /// Temperature at which the material melts. Defaults to `NONE`.
    MeltingPoint,
    /// Temperature at which the material boils. Defaults to `NONE`.
    BoilingPoint,
    /// Items composed of this material will initially have this temperature.
    /// Used in conjunction with `SPEC_HEAT:NONE` to make material's temperature fixed at the specified value.
    /// Defaults to `NONE`.
    MaterialFixedTemperature,
    /// Changes a material's `HEATDAM_POINT`, but only if it was not set to `NONE`. Not permitted in material template definitions.
    IfExistsSetHeatDamagePoint,
    /// Changes a material's `COLDDAM_POINT`, but only if it was not set to `NONE`. Not permitted in material template definitions.
    IfExistsSetColdDamagePoint,
    /// Changes a material's `IGNITE_POINT`, but only if it was not set to `NONE`. Not permitted in material template definitions.
    IfExistsSetIgnitePoint,
    /// Changes a material's `MELTING_POINT`, but only if it was not set to `NONE`. Not permitted in material template definitions.
    IfExistsSetMeltingPoint,
    /// Changes a material's `BOILING_POINT`, but only if it was not set to `NONE`. Not permitted in material template definitions.
    IfExistsSetBoilingPoint,
    /// Changes a material's `MAT_FIXED_TEMP`, but only if it was not set to `NONE`. Not permitted in material template definitions.
    IfExistsSetMatFixedTemp,
    /// Specifies the density (in kilograms per cubic meter) of the material when in solid form. Also affects combat calculations;
    /// affects blunt-force damage and ability of weak-in-impact-yield blunt attacks to pierce armor. Defaults to `NONE`.
    SolidDensity,
    /// Specifies the density of the material when in liquid form. Defaults to `NONE`. Also affects combat calculations;
    /// affects blunt force damage like `SOLID_DENSITY`, but only for attacks made by liquids (e.g. forgotten beasts made of water).
    LiquidDensity,
    /// Specifies (in kg/mol) the molar mass of the material in gaseous form. Also affects combat calculations like the densities,
    /// but only for attacks made by gases (e.g. forgotten beasts made of steam).
    MolarMass,
    /// Specifies the type of container used to store the material. Used in conjunction with the `EXTRACT_BARREL`, `EXTRACT_VIAL`,
    /// or `EXTRACT_STILL_VIAL` plant tokens.
    /// Defaults to `BARREL`.
    ExtractStorage,
    /// Specifies the item type used for butchering results made of this material. Stock raws use `GLOB:NONE` for fat and `MEAT:NONE` for other meat materials.
    ButcherSpecial,
    /// When a creature is butchered, meat yielded from organs made from this material will be named via this token.
    MeatName,
    /// Specifies the name of blocks made from this material.
    BlockName,
    /// The material forms "wafers" instead of "bars".
    Wafers,
    /// Used with reaction raws to associate a reagent material with a product material. The first argument is used by `HAS_MATERIAL_REACTION_PRODUCT` and `GET_MATERIAL_FROM_REAGENT` in reaction raws.
    /// The remainder is a material reference, generally `LOCAL_CREATURE_MAT:SUBTYPE` or `LOCAL_PLANT_MAT:SUBTYPE` or `INORGANIC:STONETYPE`.
    /// `MATERIAL_REACTION_PRODUCT:TAN_MAT:LOCAL_CREATURE_MAT:LEATHER`
    MaterialReactionProduct,
    /// Used with reaction raws to associate a reagent material with a complete item. The first argument is used by `HAS_ITEM_REACTION_PRODUCT` and `GET_ITEM_DATA_FROM_REAGENT` in reaction raws.
    /// The rest refers to the type of item, then its material.
    /// `ITEM_REACTION_PRODUCT:BAG_ITEM:PLANT_GROWTH:LEAVES:LOCAL_PLANT_MAT:LEAF`
    ItemReactionProduct,
    /// "Used to classify all items made of the material, so that reactions can use them as generic reagents.In default raws, the following are used:
    /// `FAT`, `TALLOW`, `SOAP`, `PARCHMENT`, `PAPER_PLANT`, `PAPER_SLURRY`, `MILK`, `CHEESE`, `WAX`.
    /// `CAN_GLAZE` - items made from this material can be glazed.
    /// `FLUX` - can be used as flux in pig iron and steel making.
    /// `GYPSUM` - can be processed into gypsum plaster.
    /// `CALCIUM_CARBONATE` - can be used in production of quicklime."
    ReactionClass,
    /// Makes `BOULDER` acceptable as a reagent in reactions that require `METAL_ORE:MATERIAL_NAME`, as well as smelting directly into metal bars.
    /// Places the material under Metal Ores in Stone stockpiles. The specified value determines the probability for this product (see Tetrahedrite or Galena for details).
    MetalOre,
    /// Makes `BOULDER` items made of the material acceptable for strand extraction into threads; see also `STOCKPILE_THREAD_METAL`.
    /// Value presumably determines the probability of this product extracted.
    ThreadMetal,
    /// Allows the material to be used to make casts.
    HardensWithWater,
    /// Determines effectiveness of soap - if the amount of grime on a body part is more than 3-SOAP_LEVEL, it sets it to 3-SOAP_LEVEL; as such setting it above 3 is bad.
    /// Soap has `[SOAP_LEVEL:2]`. Defaults to 0.
    SoapLevel,
    /// Begins defining a syndrome applied by the material. Multiple syndromes can be specified. See Syndrome token.
    Syndrome,
    /// This is since .50 in the raws of several antler-wielding animals. It is used to show an antler as bodypart.
    Antler,

    // Additional tokens from 50.x
    /// Hair material
    Hair,
    /// Feather material
    Feather,
    /// Scale material
    Scale,
    /// Hoof material
    Hoof,
    /// Chitin material
    Chitin,
    /// Cartilage material
    Cartilage,
    /// Nervous tissue
    NervousTissue,
    /// Category of meat
    MeatCategory,
    /// For default value, use unknown.
    #[default]
    Unknown,
}
