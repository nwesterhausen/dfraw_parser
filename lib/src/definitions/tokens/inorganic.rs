//! Tags that can be used in inorganic raws.

/// Tags that can be used in inorganic raws.
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
pub enum InorganicToken {
    /// Used on metals, causes the metal to be made into wafers instead of bars.
    Wafers,
    /// Causes the stone to form hollow tubes leading to the Underworld. Used for raw adamantine. When mined, stone has a 100% yield.
    /// If no material with this token exists, hollow veins will instead be made of the first available inorganic, usually iron. Implies \[SPECIAL\].
    DeepSpecial,
    /// Allows the ore to be smelted into metal in the smelter. Each token with a non-zero chance causes the game to roll d100 four times,
    /// each time creating one bar of the type requested on success.
    MetalOre,
    /// Allows strands to be extracted from the metal at a craftsdwarf's workshop.
    ThreadMetal,
    /// Causes the stone to line the landscape of the Underworld. Used for slade. When mined (if it's mineable), stone has a 100% yield. If no material with this token exists,
    /// other materials will be used in place of slade. Underworld spires will still be referred to as a "spire of slade" in the world's history.
    DeepSurface,
    /// Allows the stone to support an aquifer.
    Aquifer,
    /// Causes the material to form metamorphic layers.
    Metamorphic,
    /// Causes the material to form sedimentary layers.
    Sedimentary,
    /// Causes the material to form soil layers, allowing it to appear in (almost) any biome. Mining is faster and produces no stones.
    Soil,
    /// Causes the material to form pelagic sediment layers beneath deep oceans. Mining is faster and produces no stones.
    SoilOcean,
    /// Causes the material to form sand layers, allowing it to appear in sand deserts and shallow oceans. Mining is faster and produces no stones.
    /// Sand layers can also be used for making glass. Can be combined with \[SOIL\].
    SoilSand,
    /// Permits an already \[SEDIMENTARY\] stone layer to appear underneath shallow ocean regions.
    SedimentaryOceanShallow,
    /// Permits an already \[SEDIMENTARY\] stone layer to appear underneath deep ocean regions.
    SedimentaryOceanDeep,
    /// Causes the material to form igneous intrusive layers.
    IgneousExtrusive,
    /// Causes the material to form igneous extrusive layers.
    IgneousIntrusive,
    /// Specifies what types of layers will contain this mineral. Multiple instances of the same token segment will cause the rock type to occur more frequently,
    /// but won't increase its abundance in the specified environment. See below.
    Environment,
    /// Specifies which specific minerals will contain this mineral. See below.
    EnvironmentSpecific,
    /// Specifies that the stone is created when combining water and magma, also causing it to line the edges of magma pools and volcanoes.
    /// If multiple minerals are marked as lava stones, a different one will be used in each biome or geological region.
    Lava,
    /// Prevents the material from showing up in certain places. AI-controlled entities won't use the material to make items and don't bring it in caravans,
    /// though the player can use it as normal. Also, inorganic generated creatures (forgotten beasts, titans, demons) will never be composed of this material.
    /// Explicitly set by all evil weather materials and implied by `[DEEP_SURFACE]` and `[DEEP_SPECIAL]`.
    Special,
    /// Indicates that this is a generated material. Cannot be specified in user-defined raws.
    Generated,
    /// Found on random-generated metals and cloth. Marks this material as usable by Deity-created generated entities.
    Divine,
    /// Found on divine materials. Presumably links the material to a god of the same sphere.
    Sphere,
    /// Default value means parsing error.
    #[default]
    Unknown,
}

impl std::fmt::Display for InorganicToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Wafers => write!(f, "Wafers"),
            Self::DeepSpecial => write!(f, "Deep Special"),
            Self::MetalOre => write!(f, "Metal Ore"),
            Self::ThreadMetal => write!(f, "Thread Metal"),
            Self::DeepSurface => write!(f, "Deep Surface"),
            Self::Aquifer => write!(f, "Aquifer"),
            Self::Metamorphic => write!(f, "Metamorphic"),
            Self::Sedimentary => write!(f, "Sedimentary"),
            Self::Soil => write!(f, "Soil"),
            Self::SoilOcean => write!(f, "SoilOcean"),
            Self::SoilSand => write!(f, "SoilSand"),
            Self::SedimentaryOceanShallow => write!(f, "Shallow Ocean Sedimentary"),
            Self::SedimentaryOceanDeep => write!(f, "Deep Ocean Sedimentary"),
            Self::IgneousExtrusive => write!(f, "Igneous Extrusive"),
            Self::IgneousIntrusive => write!(f, "Igneous Intrusive"),
            Self::Environment => write!(f, "Environment"),
            Self::EnvironmentSpecific => write!(f, "Environment Specific"),
            Self::Lava => write!(f, "Lava"),
            Self::Special => write!(f, "Special"),
            Self::Generated => write!(f, "Generated"),
            Self::Divine => write!(f, "Divine"),
            Self::Sphere => write!(f, "Sphere"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
