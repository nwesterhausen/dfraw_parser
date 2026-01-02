//! Tags for conditions that can be applied to a tile/entity (graphics)

use crate::raw_definitions::CONDITION_TOKENS;

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
pub enum ConditionTag {
    /// No condition
    None,
    /// The start of a condition
    Condition,
    /// Default condition
    #[default]
    Default,
    /// A condition of "being animated"
    Animated,
    /// Condition of being a corpse
    Corpse,
    /// Condition of being a child
    Child,
    /// Condition of being a baby
    Baby,
    /// Condition of being trained for hunting
    TrainedHunter,
    /// Condition of being trained for war
    TrainedWar,
    /// Condition of being a list icont
    ListIcon,
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
    /// Condition of being a vampire
    VampireCursed,
    /// Condition of being a ghoul
    Ghoul,
    /// Condition of being a disturbed dead
    DisturbedDead,
    /// Condition of being remains
    Remains,
    /// Condition of being a vermin
    Vermin,
    /// Condition of being a light vermin
    LightVermin,
    /// Condition of being a hive
    Hive,
    /// Condition of being a small swarm
    SwarmSmall,
    /// Condition of being a medium swarm
    SwarmMedium,
    /// Condition of being a large swarm
    SwarmLarge,
    /// Condition of being not an artifact
    NotArtifact,
    /// Condition of being a crafted artifact
    CraftedArtifact,
    /// Condition of being dyed
    Dye,
    /// Condition of not being dyed
    NotDyed,
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
    /// Condition of not being a child
    NotChild,
    /// Condition of being at least so many hauled
    HaulCountMin,
    /// Condition of being at most so many hauled
    HaulCountMax,
    /// Condition of being a worn item
    ItemWorn,
    /// Condition of having a profession
    ProfessionCategory,
    /// Condition of being a class
    Class,
    /// Condition of being a syndrome class
    SyndromeClass,
    /// Condition of being a caste
    Caste,
    /// Condition of being a tissue layer
    TissueLayer,
    /// Condition of being a material flag
    MaterialFlag,
    /// Condition of being a material type
    MaterialType,
    /// Condition of being off if an item is present
    ShutOffIfItemPresent,
    /// Condition of being a random part index
    RandomPartIndex,
    /// Condition of being a ghost
    Ghost,
    /// Condition of being a tissue that may have color
    TissueMayHaveColor,
    /// Condition of being a tissue that is at least so long
    TissueMinLength,
    /// Condition of being a tissue that is at most so long
    TissueMaxLength,
    /// Condition of being a tissue at least so curly
    TissueMinCurly,
    /// Condition of being a tissue at most so curly
    TissueMaxCurly,
    /// Condition of being a tissue that may have a shape
    TissueMayHaveShaping,
    /// Condition of being a tissue that is not shaped
    TissueNotShaped,
    /// Condition of being a swapped tissue
    TissueSwap,
    /// Condition of being a specific layer (start layer definition)
    Layer,
    /// Condition of being a specific layer set of layers
    LayerSet,
    /// Condition of being a specific layer group
    LayerGroup,
    /// Condition of being a specific layer group set of layers
    EndLayerGroup,
    /// Condition of being the upper body
    BodyUpper,
    /// Condition of being a copy of a template
    CopyOfTemplate,

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

impl ConditionTag {
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
