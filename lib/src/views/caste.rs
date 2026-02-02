use dfraw_parser_proc_macros::IsEmpty;

use crate::{
    Gait,
    custom_types::{BodySize, Name, Tile},
    tokens::CasteToken,
};

/// A struct representing a creature caste.
///
/// Castes are specific subgroups within a creature species, often representing
/// biological sexes, specialized roles, or unique variations specified in the raw files.
#[allow(clippy::module_name_repetitions)]
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    Default,
    specta::Type,
    Eq,
    PartialEq,
    IsEmpty,
)]
#[serde(rename_all = "camelCase")]
pub struct CasteView {
    /// The unique name used in raw files for this caste (e.g., "MALE", "FEMALE").
    pub identifier: String,
    /// A collection of tags assigned to this caste.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub tokens: Vec<CasteToken>,
    /// Flavor text shown in-game when examining a creature of this caste.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub description: Option<String>,
    /// The specific name for a creature in its baby stage.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub baby_name: Option<Name>,
    /// The name used specifically for this caste.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub caste_name: Option<Name>,
    /// The name for a creature in its child stage.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub child_name: Option<Name>,
    /// The range of eggs produced per clutch, measured as `[min, max]`.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub clutch_size: Option<[u32; 2]>,
    /// The range of offspring produced per birth, measured as `[min, max]`.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub litter_size: Option<[u32; 2]>,
    /// The range of life expectancy in game ticks, measured as `[min, max]`.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub max_age: Option<[u32; 2]>,
    /// The age in game ticks at which a creature ceases to be a baby.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub baby: Option<u32>,
    /// The age in game ticks at which a creature ceases to be a child.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub child: Option<u32>,
    /// A rating used to determine the challenge level of the creature.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub difficulty: Option<u32>,
    /// The size of eggs laid by this caste, measured in cubic centimeters.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub egg_size: Option<u32>,
    /// The distance or frequency at which this creature tramples grass.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub grass_trample: Option<u32>,
    /// The grazing requirement for the creature to survive.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub grazer: Option<u32>,
    /// The level of vision the creature has in dark environments.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub low_light_vision: Option<u32>,
    /// The value assigned to the creature when kept as a pet.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub pet_value: Option<u32>,
    /// The relative frequency this caste appears in wild populations.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub pop_ratio: Option<u32>,
    /// The percentage change applied to the base body size.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub change_body_size_percentage: Option<u32>,
    /// The classes or categories this caste belongs to for targeting.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub creature_class: Option<Vec<String>>,
    /// Growth stages and volume measurements.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub body_size: Option<Vec<BodySize>>,
    /// Material and frequency information for milking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milkable: Option<(String, u32)>,
    /// Character and color data for map representation.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub tile: Option<Tile>,
    /// The gaits by which the creature can move.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub gaits: Option<Vec<Gait>>,
}
