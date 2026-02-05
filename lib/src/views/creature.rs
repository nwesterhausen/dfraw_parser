use dfraw_parser_proc_macros::IsEmpty;
use uuid::Uuid;

use crate::{
    custom_types::{Name, Tile},
    metadata::RawMetadata,
    tokens::{BiomeToken, CreatureToken},
    views::CasteView,
};

/// The `Creature` struct represents a creature in a Dwarf Fortress, with the properties
/// that can be set in the raws. Not all the raws are represented here, only the ones that
/// are currently supported by the library.
///
/// Some items like `CREATURE_VARIATION` and `CREATURE_VARIATION_CASTE` are saved in their raw
/// format. `SELECT_CREATURE` is saved here as a sub-creature object with all the properties
/// from that raw. This is because the `SELECT_CREATURE` raws are used to create new creatures
/// based on the properties of the creature they are applied to. But right now the application
/// of those changes is not applied, in order to preserve the original creature. So instead,
/// they are saved and can be applied later (at the consumer's discretion).
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    Default,
    specta::Type,
    PartialEq,
    Eq,
    IsEmpty,
)]
#[serde(rename_all = "camelCase")]
pub struct CreatureView {
    /// The `metadata` field is of type `RawMetadata` and is used to provide additional information
    /// about the raws the `Creature` is found in.
    pub metadata: RawMetadata,
    /// The `identifier` field is a string that represents the identifier of the creature. It is used
    /// to uniquely identify the creature (however it is not guaranteed to be unique across object types
    /// or all raws parsed, *especially* if you are parsing multiple versions of the same raws).
    pub identifier: String,
    /// A generated id that is used to uniquely identify this object.
    ///
    /// This is deterministic based on the following:
    /// * The raw's `identifier`
    /// * The raw's [`ObjectType`]
    /// * [`RawModuleLocation`] where the raw was found
    /// * The containing module's `numeric_version`
    ///
    /// See [`crate::utilities::generate_object_id`]
    pub object_id: Uuid,
    /// The `castes` field is a vector of `Caste` objects. Each `Caste` object represents a caste of the
    /// creature. For example, a creature may have a `MALE` and `FEMALE` caste. Each `Caste` object has
    /// its own properties, such as `name`, `description`, `body`, `flags`, etc.
    ///
    /// A lot of the properties of the `Creature` object are actually properties of a special `Caste`, `ALL`.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    pub castes: Vec<CasteView>,
    /// Any tags that are not parsed into their own fields are stored in the `tags` field.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    pub tags: Vec<CreatureToken>,
    /// The biomes that this creature can be found in
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    pub biomes: Vec<BiomeToken>,
    /// Pref strings are things that make dwarves (or others?) like or dislike the creature.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    pub pref_strings: Vec<String>,
    /// The tile that represents the creature in the game (classic mode)
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    pub tile: Option<Tile>,
    /// Determines the chances of a creature appearing within its environment, with higher values resulting in more frequent appearance.
    ///
    /// Also affects the chance of a creature being brought in a caravan for trading. The game effectively considers all creatures that
    /// can possibly appear and uses the FREQUENCY value as a weight - for example, if there are three creatures with frequencies 10/25/50,
    /// the creature with `[FREQUENCY:50]` will appear approximately 58.8% of the time.
    ///
    /// Defaults to 50 if not specified.
    ///
    /// Minimum value is 0, maximum value is 100.
    ///
    /// pub Note: not to be confused with `[POP_RATIO]`.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    #[is_empty(value = 50)]
    pub frequency: Option<u32>,
    /// The minimum/maximum numbers of how many creatures per spawned cluster. Vermin fish with this token in combination with
    /// temperate ocean and river biome tokens will perform seasonal migrations.
    ///
    /// Defaults to [1,1] if not specified.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    #[is_empty(value = [1,1])]
    pub cluster_number: Option<[u32; 2]>,
    /// The minimum/maximum numbers of how many of these creatures are present in each world map tile of the appropriate region.
    ///
    /// Defaults to [1,1] if not specified.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    #[is_empty(value = [1,1])]
    pub population_number: Option<[u32; 2]>,
    /// Depth that the creature appears underground. Numbers can be from 0 to 5. 0 is actually 'above ground' and can be used if the
    /// creature is to appear both above and below ground. Values from 1-3 are the respective cavern levels, 4 is the magma sea and
    /// 5 is the HFS.
    ///
    /// A single argument may be used instead of min and max.
    ///
    /// Civilizations that can use underground plants or animals will only export (via the embark screen or caravans) things that are available at depth 1.
    ///
    /// Default [0, 0] (aboveground)
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    pub underground_depth: Option<[u32; 2]>,
    /// Like `[BABYNAME]`, but applied regardless of caste.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    pub general_baby_name: Option<Name>,
    /// Like `[CHILDNAME]`, but applied regardless of caste.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    pub general_child_name: Option<Name>,
    /// The generic name for any creature of this type - will be used when distinctions between caste are unimportant. For names for specific castes,
    /// use `[CASTE_NAME]` instead. If left undefined, the creature will be labeled as "nothing" by the game.
    pub name: Name,
}
