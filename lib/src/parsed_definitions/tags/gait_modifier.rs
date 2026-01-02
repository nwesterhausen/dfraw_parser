//! Gait modifier tags are used to modify the speed of a creature based on various factors.

/// An enum representing a gait modifier.
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Copy,
    specta::Type,
    strum_macros::EnumIter,
)]
pub enum GaitModifierTag {
    /// Fat/muscle layers slow the movement (muscle-slowing counter-acted by strength bonus)
    /// Makes `THICKENS_ON_ENERGY_STORAGE` and `THICKENS_ON_STRENGTH` tissue layers slow movement depending on how thick they are.
    /// Adding the `STRENGTH` gait flag counteracts the impact of the latter layer.
    LayersSlow,
    /// Speeds/slows movement depending on the creature's Strength stat.
    Strength,
    /// Speeds/slows movement depending on the creature's Agility stat.
    Agility,
    /// Stealth slows movement by the specified percentage when the creature is sneaking.
    StealthSlows {
        /// The percentage slowed
        percentage: u32,
    },
    /// No build up time
    NoBuildUp,
    /// Build up time. Only used if the gait has a build up time.
    BuildUp {
        /// The build up time indicates how long it will take for a creature using this gait to go from `<start speed>` to `<max speed>`.
        /// For example, a value of 10 means that it should be able to reach the maximum speed by moving 10 tiles in a straight line over even terrain.
        time: u32,
        /// The turning max indicates the maximum speed permissible when the creature suddenly changes its direction of motion.
        /// The creature's speed will be reduced to `<max turning speed>` if traveling at a higher speed than this before turning.
        turning_max: u32,
        /// The creature's speed when it starts moving using this gait
        start_speed: u32,
    },
}
