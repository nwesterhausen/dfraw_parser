use std::collections::HashSet;

use dfraw_parser_proc_macros::IsEmpty;
use uuid::Uuid;

use crate::{
    Creature, SelectCreature,
    custom_types::{Name, Tile},
    metadata::{NumericToken, RawMetadata},
    tokens::{BiomeToken, CasteToken, CreatureToken, ObjectType},
    traits::{NumericTokenTransform as _, RawObjectView, RawToken},
    views::CasteView,
};

/// A view for the Creature raw object. This includes all fields from Creature plus some
/// additional fields for ease-of-use when displaying information to users.
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

    /// Trait passed-thru from `Creature`,
    /// Copies another specified creature. This will override any definitions made before it; essentially, it makes this creature identical to the other one,
    /// which can then be modified. Often used in combination with `[APPLY_CREATURE_VARIATION]` to import standard variations from a file.
    ///
    /// The vanilla giant animals and animal peoples are examples of this token combination.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    pub copy_tokens_from: Option<String>,
    /// Trait passed-thru from `Creature`,
    /// Applies the specified creature variation.
    ///
    /// These are stored "in the raw", i.e. how they appear in the raws. They are not handled until the end of the parsing process.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    pub apply_creature_variation: Option<Vec<String>>,
    /// Trait passed-thru from `Creature`,
    /// Various `SELECT_CREATURE` modifications.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    pub select_creature_variation: Option<Vec<SelectCreature>>,
}

impl From<Creature> for CreatureView {
    fn from(value: Creature) -> Self {
        Self {
            metadata: value.metadata.clone(),
            identifier: value.identifier.clone(),
            object_id: value.object_id,
            tags: value.tokens.clone(),
            biomes: value.get_biomes().clone(),
            pref_strings: value.get_pref_strings(),
            // todo: handle Tile value better
            tile: None,
            frequency: value.get_frequency(),
            cluster_number: value.get_cluster_number(),
            population_number: value.get_population_number(),
            underground_depth: value.get_underground_depth(),
            general_baby_name: value.get_general_baby_name(),
            general_child_name: value.get_general_child_name(),
            name: value.get_name(),
            castes: value.castes.into_iter().map(CasteView::from).collect(),
            apply_creature_variation: value.apply_creature_variation.clone(),
            copy_tokens_from: value.copy_tokens_from.clone(),
            select_creature_variation: value.select_creature_variation.clone(),
        }
    }
}

#[typetag::serde]
impl RawObjectView for Creature {
    fn get_metadata(&self) -> RawMetadata {
        self.metadata.clone()
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
    fn get_type(&self) -> ObjectType {
        ObjectType::Creature
    }
    fn get_object_id(&self) -> Uuid {
        self.object_id
    }
    fn get_name(&self) -> &str {
        self.tokens
            .iter()
            .find_map(|token| match token {
                CreatureToken::Name { name } => Some(name.get_singular()),
                _ => None,
            })
            // If find_map returns None, return the identifier instead
            .unwrap_or(&self.identifier)
    }
    fn get_searchable_tokens(&self) -> Vec<&str> {
        let mut tokens = HashSet::new();

        for token in CreatureToken::FLAG_TOKENS {
            if self.has_token(token) {
                tokens.insert(RawToken::get_key(token).unwrap_or_default());
            }
        }

        for caste in &self.castes {
            for token in CasteToken::FLAG_TOKENS {
                if caste.has_token(token) {
                    tokens.insert(RawToken::get_key(token).unwrap_or_default());
                }
            }
        }

        tokens.into_iter().collect()
    }
    fn get_numeric_flags(&self) -> Vec<NumericToken> {
        let mut tokens = Vec::new();

        // Collect from Creature Tags
        for token in &self.tokens {
            tokens.extend(token.as_numeric_tokens());
        }

        // Collect from Caste Tags
        for caste in &self.castes {
            for tag in caste.get_tokens() {
                tokens.extend(tag.as_numeric_tokens());
            }
        }

        tokens
    }
}
