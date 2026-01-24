//! Tags for conditions that can be applied to a tile/entity (graphics)

use crate::{raw_definitions::CONDITION_TOKENS, traits::IsEmpty};

/// A condition that can be applied to a tile/entity
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
pub enum ConditionToken {
    #[default]
    /// No condition
    None,
    /// A portrait of the creature, used when interacting with them
    Portrait,
    /// The start of a condition
    Condition,
    /// Used when defining a default sprite image
    ///
    /// `[DEFAULT:...]`
    Default,
    /// Used when defining a child sprite image
    ///
    /// `[CHILD:...]`
    ChildPrime,
    /// Used when defining a baby sprite image
    ///
    /// `[BABY:...]`
    BabyPrime,
    /// Displayed if the creature is raised from the dead, although it is not
    /// known how this is decided. Raised status is not related to having a
    /// syndrome with the class from `[CONDITION_SYN_CLASS]` or from having
    /// `[NOT_LIVING]`/`[OPPOSED_TO_LIFE]`.
    ///
    /// Used when defining a sprite image.
    ///
    /// `[ANIMATED:...]`
    Animated,
    /// Displayed as soon as the creature dies.
    ///
    /// `[CORPSE:...]`
    Corpse,
    /// Displayed in menus. Useful for large images that would extend beyond the
    /// menu boxes otherwise.
    ///
    /// `[LIST_ICON]`
    ListIcon,
    /// Displayed in interaction menus in Adventure Mode, overrides `LIST_ICON` when
    /// specified in a creature `CAN_DO_INTERACTION` using `CDI:TOKEN:token_name`.
    ///
    /// Might accept referenced token_name before standard secondaries.
    ///
    /// `[CDI_LIST_ICON]`
    CdiListIcon,
    /// Condition of being trained for hunting
    TrainedHunter,
    /// Condition of being trained for war
    TrainedWar,
    /// Condition of being a skeleton
    Skeleton,
    /// Condition of being a skeleton with a skull
    SkeletonWithSkull,
    /// Condition of being a zombie
    Zombie,
    /// Condition of being a necromancer
    Necromancer,
    /// Condition of being male
    Male,
    /// Condition of being female
    Female,
    /// `[CONDITION_BABY]`
    Baby,
    /// Condition of being a vampire
    VampireCursed,
    /// Condition of being a ghoul
    Ghoul,
    /// Condition of being a disturbed dead
    DisturbedDead,
    /// Condition of being remains
    Remains,
    /// Displayed if the unit escorts a tax collector (unused).
    ///
    /// `[TAX_ESCORT]`
    TaxEscort,
    /// Displayed if the unit is law enforcement.
    ///
    /// `[LAW_ENFORCE]`
    LawEnforcement,
    /// Displayed if the creature is an adventurer.
    ///
    /// `[ADVENTURER]`
    Adventurer,
    /// The creature is in the dark. Graphical replacement for `[GLOWTILE]`.
    ///
    /// `[GLOW]`
    Glow,
    /// As `[GLOW]`, but with their left eye missing. If the sprite is facing forwards, then the
    /// visually leftmost eye should remain.
    ///
    /// `[GLOW_LEFT_GONE]`
    GlowLeftGone,
    /// As `[GLOW]`, but with their left eye missing. If the sprite is facing forwards, then the
    /// visually leftmost eye should remain.
    ///
    /// `[GLOW_RIGHT_GONE]`
    GlowRightGone,
    /// A child creature is in darkness. Does not have wound states.
    ///
    /// `[GLOW_CHILD]`
    GlowChild,
    /// The sprite for a clutch of eggs.
    ///
    /// `[EGG]`
    Egg,
    /// The default graphic for this vermin.
    ///
    /// `[VERMIN]`
    Vermin,
    /// The alternating graphic for this vermin. Image cycles every 1 second.
    ///
    /// `[VERMIN_ALT]`
    VerminAlt,
    /// For swarming vermin like flies and fairies in small groups.
    ///
    /// `[SWARM_SMALL]`
    SwarmSmall,
    /// For swarming vermin like flies and fairies in medium-sized groups.
    ///
    /// `[SWARM_MEDIUM]`
    SwarmMedium,
    /// For swarming vermin like flies and fairies in large groups.
    ///
    /// `[SWARM_LARGE]`
    SwarmLarge,
    /// Light-producing vermin, for fireflies etc. Does not replace `[VERMIN]`.
    ///
    /// `[LIGHT_VERMIN]`
    LightVermin,
    /// The alternating graphic for this light-producing vermin. Image cycles every 1 second.
    ///
    /// `[LIGHT_VERMIN_ALT]`
    LightVerminAlt,
    /// For swarming vermin like flies and fairies in small groups.
    ///
    /// `[LIGHT_SWARM_SMALL]`
    LightSwarmSmall,
    /// For swarming vermin like flies and fairies in medium-sized groups.
    ///
    /// `[LIGHT_SWARM_MEDIUM]`
    LightSwarmMedium,
    /// For swarming vermin like flies and fairies in large groups.
    ///
    /// `[LIGHT_SWARM_LARGE]`
    LightSwarmLarge,
    /// Vermin hives.
    ///
    /// `[HIVE]`
    Hive,
    /// Condition of being not an artifact
    NotArtifact,
    /// Condition of being a crafted artifact
    CraftedArtifact,
    /// Condition of being a crop
    Crop,
    /// Condition of being a seed
    Seed,
    /// Condition of being a plant (picked)
    Picked,
    /// Condition of being a shrub
    Shrub,
    /// Condition of being a sapling
    Sapling,
    /// Condition of being a crop sprout
    CropSprout,
    /// Condition of being a large crop
    CropL,
    /// Condition of being a medium crop
    CropM,
    /// Condition of being a small crop
    CropR,
    /// Condition of being a dead shrub
    ShrubDead,
    /// Checks if the creature is a child or baby.
    ///
    /// `[CONDITION_CHILD]`
    Child,
    /// Checks if the creature is an adult.
    ///
    /// `[CONDITION_NOT_CHILD]`
    NotChild,
    /// Counts how many items the creature is hauling. Used for `[PACK_ANIMAL]`s in vanilla.
    ///
    /// `[CONDITION_HAUL_COUNT_MIN:count]`
    HaulCountMin,
    /// Counts how many items the creature is hauling. Used for `[PACK_ANIMAL]`s in vanilla.
    ///
    /// `[CONDITION_HAUL_COUNT_MAX:count]`
    HaulCountMax,
    /// Condition of being a class
    Class,
    /// Defines a body part graphic using standard body token selection criteria.
    ///
    /// Selection is done with `BY_TYPE`, `BY_CATEGORY`, or `BY_TOKEN`
    ///
    /// `[CONDITION_BP:selection:category, type, or token]`
    BodyPart,
    /// Checks if current `[CONDITION_BP]`'s `[BP_APPEARANCE_MODIFIER]` falls within the chosen range.
    ///
    /// `[BP_APPEARANCE_MODIFIER_RANGE]`
    BodyPartAppearanceModifierRange,
    /// Checks if the current `[CONDITION_BP]` is present and not destroyed, pulped, or severed. Can also be applied to
    /// `[LG_CONDITION_BP]`.
    ///
    /// `[BP_PRESENT]`
    BodyPartPresent,
    /// Checks if the current `[CONDITION_BP]` is scarred. Seems to also require `[BP_PRESENT]` to avoid illogical results.
    ///
    /// `[BP_SCARRED]`
    BodyPartScarred,
    /// True if creature size is greater than defined size.
    ///
    /// `[CONDITION_BODY_SIZE_MIN:size]`
    BodySizeMin,
    /// True if creature size is less than defined size.
    ///
    /// `[CONDITION_BODY_SIZE_MAX:size]`
    BodySizeMax,
    /// Changes graphics based on any syndromes the creature is affected by. Vanilla values include:
    /// - `ZOMBIE`
    /// - `NECROMANCER`
    /// - `VAMPCURSE`
    /// - `RAISED_UNDEAD`
    /// - `DISTURBED_DEAD`
    /// - `GHOUL`
    ///
    /// `[CONDITION_SYN_CLASS:class]`
    SyndromeClass,
    /// Selects a tissue layer to use for checking other conditions.
    ///
    /// `[CONDITION_TISSUE_LAYER:BY_CATEGORY:ALL:SKIN]`
    ///
    /// `[CONDITION_TISSUE_LAYER:BY_CATEGORY:bp category or 'ALL':tissue layer or 'ALL']`
    TissueLayer,
    /// Chooses a random layer among layers with a `CONDITION_RANDOM_PART_INDEX` with the same identifier. Index
    /// is which option this condition is, out of Range number of options.
    ///
    /// `[CONDITION_RANDOM_PART_INDEX:HEAD:3:4]` is the third possible random head out of four total options.
    /// One of these random conditions each will be put into a set of four different sprites to add some random
    /// variation in the appearance of the creature's head.
    ///
    /// `[CONDITION_RANDOM_PART_INDEX:identifier:index:range]`
    RandomPartIndex,
    /// Checks if the creature is a ghost.
    ///
    /// `[CONDITION_GHOST]`
    Ghost,
    /// Checks the selected tissue's color. Accepts multiple color tokens, and is true if the any of the colors
    /// is present in the selected tissues.
    ///
    /// `[TISSUE_MAY_HAVE_COLOR:color token:more color tokens]`
    TissueMayHaveColor,
    /// Checks the current `[CONDITION_TISSUE_LAYER]`'s `LENGTH` appearance modifier. Is true if the `LENGTH` is
    /// greater than the integer input.
    ///
    /// `[TISSUE_MIN_LENGTH:length]`
    TissueMinLength,
    /// Checks the current `[CONDITION_TISSUE_LAYER]`'s `LENGTH` appearance modifier. Is true if the `LENGTH` is
    /// less than the integer input.
    ///
    /// `[TISSUE_MAX_LENGTH:length]`
    TissueMaxLength,
    /// Checks the current `[CONDITION_TISSUE_LAYER]`'s `DENSITY` appearance modifier. Is true if the `DENSITY` is
    /// greater than the integer input.
    ///
    /// `[TISSUE_MIN_DENSITY:desnsity]`
    TissueMinDensity,
    /// Checks the current `[CONDITION_TISSUE_LAYER]`'s `DENSITY` appearance modifier. Is true if the `DENSITY` is
    /// less than the integer input.
    ///
    /// `[TISSUE_MAX_DENSITY:desnsity]`
    TissueMaxDensity,
    /// Condition of being a tissue at least so curly
    TissueMinCurly,
    /// Condition of being a tissue at most so curly
    TissueMaxCurly,
    /// Checks the current `[CONDITION_TISSUE_LAYER]`'s shaping (hairstyle). Valid tokens are
    /// - `NEATLY_COMBED`
    /// - `BRAIDED`
    /// - `DOUBLE_BRAIDS`
    /// - `PONY_TAILS`
    /// - `CLEAN_SHAVEN `
    /// - `STANDARD_HAIR/BEARD/MOUSTACHE/SIDEBURNS_SHAPINGS`
    ///
    /// `[TISSUE_MAY_HAVE_SHAPING:styling token]`
    TissueMayHaveShaping,
    /// Checks the current `[CONDITION_TISSUE_LAYER]`'s color. Accepts multiple color tokens, and is true if the
    /// any of the colors is present in the selected tissues.
    ///
    /// `[TISSUE_NOT_SHAPED]`
    TissueNotShaped,
    /// Checks if a tissue is sufficiently curly, and if so swaps to display a different image. The new image
    /// is defined by the tile page ID, x position, and y position.
    ///
    /// This condition should be within a `[LAYER:... ]` that has a similar graphic to the on in the `TISSUE_SWAP`.
    /// The current `[CONDITION_TISSUE_LAYER]` group must also include a `[TISSUE_MIN_LENGTH]`.
    ///
    /// `[TISSUE_SWAP:IF_MIN_CURLY:curl amount:tile page id:x pos:y pos]`
    TissueSwap,
    /// Condition of being a specific layer (start layer definition)
    Layer,
    /// Condition of being the upper body
    BodyUpper,
    /// Condition of being a copy of a template
    CopyOfTemplate,
    /// Checks the current `[CONDITION_ITEM_WORN]`'s quality. 0 is base quality, 5 is masterwork.
    /// See `[CONDITION_MATERIAL_FLAG:NOT_ARTIFACT]` for non-artifact-quality items.
    ///
    /// `[ITEM_QUALITY:quality id]`
    ItemQuality,
    /// Begins a layer group. Only the first-matching layer in a group will be rendered, so list more
    /// specific items at the beginning of the layer group and more general items towards the end.
    ///
    /// `[LAYER_GROUP]`
    LayerGroup,
    /// Condition of being a specific layer group set of layers
    /// Begins defining a layer set for a creature's graphics.
    ///
    /// `[LAYER_SET:condition]`
    LayerSet,
    /// Begins defining a palette for the layer set. Its name can then be referenced by `[USE_PALETTE]`.
    /// Unlike the palettes used to render all descriptor color tokens, it can be of arbitrary length.
    ///
    /// `[LS_PALETTE:name]`
    LayerSetPalette,
    /// The file name of the 8bit RGBA (sometimes called 32bit) in the /graphics/images folder of the mod,
    /// such as `images/portraits/dwarf_portrait_body_palette.png`.
    ///
    /// `[LS_PALETTE_FILE:file path]`
    LayerSetPaletteFile,
    /// Defines the default row of a layer set palette, conventionally 0. The exact color values on this row
    /// will be replaced on layer images with the colors in the same column, based on what row is passed as
    /// an argument to `[USE_PALETTE]`.
    ///
    /// `[LS_PALETTE_DEFAULT:integer]`
    LayerSetPaletteDefault,
    /// Allows the entire layer group (rather than an individual layer) to be switched on and off depending on the
    /// conditions of a body part. Should accept the same tokens `[CONDITION_BP]` does.
    ///
    /// Selection is done with `BY_TYPE`, `BY_CATEGORY`, or `BY_TOKEN`
    ///
    /// `[LG_CONDITION_BP:selection:cateogry, type, or token]`
    LayerGroupBodyPart,
    /// Explicitly marks the end of a layer group, which allows layers after to not belong to any layer group.
    ///
    /// `[END_LAYER_GROUP]`
    EndLayerGroup,
    /// Defines a clothing or armor graphic by the specific part it is equipped to, the type of armor it is, and the
    /// internal ID of that item. Additional arguments can be supplied to check for additional subtypes. Valid if any
    /// matching items are worn.
    ///
    /// For example, a condition representing a right handed mitten or glove would be defined as:
    ///
    /// `[CONDITION_ITEM_WORN:BY_TOKEN:RH:GLOVES:ITEM_GLOVES_MITTENS]` Also accepts the input `ANY_HELD` or `WIELD`
    /// (e.g. `WIELD:WEAPON:ANY`), though `ANY_HELD` has been bugged since v50.14.
    ///
    /// Selection is done with `BY_CATEGORY` or `BY_TOKEN`
    ///
    /// `[CONDITION_ITEM_WORN:selection:cateogry or token:armor type:item id]`
    ItemWorn,
    /// Causes the current layer to not be rendered if the creature has one of the items worn or equipped. Also accepts
    /// the input `ANY_HELD` or `WIELD` (e.g. `WIELD:WEAPON:ANY`). Note that `ANY_HELD` has been bugged since v50.14.
    ///
    /// Selection is done with `BY_CATEGORY` or `BY_TOKEN`
    ///
    /// `[SHUT_OFF_IF_ITEM_PRESENT:selection:cateogry or token:armor type:item id]`
    ShutOffIfItemPresent,
    /// Displays this layer if the creature is this caste. Only one caste is accepted for each condition, but multiple
    /// caste conditions can be used in one layer and the layer will be displayed if any of them match.
    ///
    /// `[CONDITION_CASTE:caste name]`
    Caste,
    /// Represents which color the clothing is dyed. Partially-working.v50.15
    ///
    /// Takes a descriptor color. Vanilla dye options:
    ///
    /// - MIDNIGHT_BLUE - (Dimple cup)
    /// - EMERALD - (Blade weed)
    /// - RED - (Hide root)
    /// - BLACK - (Sliver barb)
    ///
    /// `[CONDITION_DYE:cye color]`
    Dye,
    /// Checks if the clothing is dyed.v50.15
    ///
    /// `[CONDITION_NOT_DYED]`
    NotDyed,
    /// Changes graphics based on the material an equipped item is made of. Specifying multiple of this condition for a
    /// layer uses the "AND" instead of "OR" logical operator, whether placed in the same line or on separate lines. Valid
    /// material flags are similar to reactant conditions including:
    ///
    /// - `WOVEN_ITEM`
    /// - `ANY_X_MATERIAL` with X being:
    ///     - `PLANT`, `SILK`, `YARN`, `LEATHER`, `WOOD`, `SHELL`, `BONE`, `STONE`, `GEM`, `TOOTH`, `HORN`, `PEARL`
    /// - `IS_DIVINE_MATERIAL`
    /// - `NOT_ARTIFACT`
    /// - `IS_CRAFTED_ARTIFACT` (Note that this token might not have ever worked.)
    /// - `METAL_ITEM_MATERIAL`
    /// - `GLASS_MATERIAL`
    /// - `FIRE_BUILD_SAFE`
    /// - `MAGMA_BUILD_SAFE`
    /// - `GROWN_NOT_CRAFTED`
    ///
    /// `[CONDITION_MATERIAL_FLAG:flag]`
    MaterialFlag,
    /// Changes graphics based on the material an equipped item is made of. Valid material types are `INORGANIC` or `METAL:IRON`
    /// where "iron" can be replaced with any weapons-grade metal. General material tokens are not functional.
    ///
    /// `[CONDITION_MATERIAL_FLAG]` is a better option for any material condition other than metal.
    ///
    /// `[CONDITION_MATERIAL_TYPE:material token]`
    MaterialType,
    /// Colors the layer using that row of either the layer-set-specific `[LS_PALETTE]` or a predefined palette such as `DEFAULT`.
    ///
    /// `[USE_PALETTE:layer set palette:row]
    UsePalette,
    /// Uses the default palette to render the layer based on the color of the current `[CONDITION_ITEM_WORN]`.
    ///
    /// `[USE_STANDARD_PALETTE_FROM_ITEM]`
    UseStandardPaletteFromItem,

    /// Note: This condition is bugged and doesn't work since DFv50.14.
    ///
    /// Checks the profession category of the creature to act as a condition. Multiple profession category tokens can be supplied as additional arguments, and will be valid for any of them. You can also use multiple of these tokens instead of listing them all in a single line, but this is functionally identical. Valid Profession tokens which are not categories will be ignored; values that do not match any existing Profession will be treated as NONE and thus apply to doctors, military, etc..
    ///
    /// `[CONDITION_PROFESSION_CATEGORY:prefession tokens (one ore more)]
    ProfessionCategory,
    // Professions (somewhat of a hack.. but some mods don't use profession category and instead call direct)
    /// Hammerman profession
    Hammerman,
    /// Master Hammerman profession
    MasterHammerman,
    /// Spearman profession
    Spearman,
    /// Master Spearman profession
    MasterSpearman,
    /// Wrestler profession
    Wrestler,
    /// Master Wrestler profession
    MasterWrestler,
    /// Axeman profession
    Axeman,
    /// Master Axeman profession
    MasterAxeman,
    /// Swordsman profession
    Swordsman,
    /// Master Swordsman profession
    MasterSwordsman,
    /// Maceman profession
    Maceman,
    /// Master Maceman profession
    MasterMaceman,
    /// Pikeman profession
    Pikeman,
    /// Master Pikeman profession
    MasterPikeman,
    /// Recruit profession
    Recruit,
    /// Thief profession
    Thief,
    /// Master Thief profession
    MasterThief,
    /// Lasher profession
    Lasher,
    /// Master Lasher profession
    MasterLasher,
    /// Monster slayer profession
    MonsterSlayer,
    /// Crossbowman profession
    Crossbowman,
    /// Master Crossbowman profession
    MasterCrossbowman,
    /// Bowman profession
    Bowman,
    /// Master Bowman profession
    MasterBowman,
    /// Blowgunman profession
    Blowgunman,
    /// Master Blowgunman profession
    MasterBlowgunman,
    /// Beat hunter profession
    BeastHunter,
    /// Scout profession
    Scout,
    /// Ranger profession
    Ranger,
    /// Hunter profession
    Hunter,
    /// Sage profession
    Sage,
    /// Scholar profession
    Scholar,
    /// Philosopher profession
    Philosopher,
    /// Mathematician profession
    Mathematician,
    /// Historian profession
    Historian,
    /// Astronomer profession
    Astronomer,
    /// Naturalist profession
    Naturalist,
    /// Chemist profession
    Chemist,
    /// Geographer profession
    Geographer,
    /// Scribe profession
    Scribe,
    /// Bookbinder profession
    Bookbinder,
    /// Performer profession
    Performer,
    /// Poet profession
    Poet,
    /// Bard profession
    Bard,
    /// Dancer profession
    Dancer,
}

impl ConditionToken {
    /// Parse a token into a Condition
    ///
    /// # Arguments
    ///
    /// * `token` - The token to parse
    ///
    /// # Returns
    ///
    /// The parsed Condition
    #[must_use]
    pub fn from_token(token: &str) -> Option<Self> {
        CONDITION_TOKENS.get(token).copied()
    }
    /// Whether the Condition is the default value.
    ///
    /// # Returns
    ///
    /// True if the Condition is the default value, false otherwise.
    #[must_use]
    pub const fn is_default(self) -> bool {
        matches!(self, Self::None)
    }
    /// Whether the Condition is the default value.
    ///
    /// # Returns
    ///
    /// True if the Condition is the default value, false otherwise.
    #[must_use]
    pub const fn is_none(&self) -> bool {
        self.is_default()
    }
}

impl std::fmt::Display for ConditionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl IsEmpty for ConditionToken {
    fn is_empty(&self) -> bool {
        self == &Self::None
    }
}
