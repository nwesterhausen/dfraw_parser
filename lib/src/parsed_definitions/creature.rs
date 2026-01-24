//! The `Creature` struct represents a creature in a Dwarf Fortress, with the properties
//! that can be set in the raws. Not all the raws are represented here, only the ones that
//! are currently supported by the library.

use std::collections::HashSet;

use dfraw_parser_proc_macros::{Cleanable, IsEmpty};
use tracing::{debug, trace, warn};
use uuid::Uuid;

use crate::{
    Caste, Name, SelectCreature, Tile,
    metadata::{NumericToken, RawMetadata},
    raw_definitions::{BIOME_TOKENS, CASTE_TOKENS, CREATURE_TOKENS},
    tokens::{BiomeToken, CasteToken, CreatureToken, ObjectType},
    traits::{
        Cleanable, CreatureVariationRequirements, NumericTokenTransform as _, RawObject,
        RawObjectToken, Searchable, TagOperations,
    },
    utilities::{clean_search_vec, generate_object_id_using_raw_metadata},
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
    Cleanable,
)]
#[serde(rename_all = "camelCase")]
pub struct Creature {
    /// The `metadata` field is of type `RawMetadata` and is used to provide additional information
    /// about the raws the `Creature` is found in.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    metadata: Option<RawMetadata>,
    /// The `identifier` field is a string that represents the identifier of the creature. It is used
    /// to uniquely identify the creature (however it is not guaranteed to be unique across object types
    /// or all raws parsed, *especially* if you are parsing multiple versions of the same raws).
    identifier: String,
    /// The `castes` field is a vector of `Caste` objects. Each `Caste` object represents a caste of the
    /// creature. For example, a creature may have a `MALE` and `FEMALE` caste. Each `Caste` object has
    /// its own properties, such as `name`, `description`, `body`, `flags`, etc.
    ///
    /// A lot of the properties of the `Creature` object are actually properties of a special `Caste`, `ALL`.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    castes: Vec<Caste>,
    /// Any tags that are not parsed into their own fields are stored in the `tags` field.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    tags: Option<Vec<CreatureToken>>,
    /// The biomes that this creature can be found in
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    biomes: Option<Vec<BiomeToken>>,
    /// Pref strings are things that make dwarves (or others?) like or dislike the creature.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pref_strings: Option<Vec<String>>,
    /// The tile that represents the creature in the game (classic mode)
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    tile: Option<Tile>,
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
    /// Note: not to be confused with `[POP_RATIO]`.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[is_empty(value = 50)]
    frequency: Option<u32>,
    /// The minimum/maximum numbers of how many creatures per spawned cluster. Vermin fish with this token in combination with
    /// temperate ocean and river biome tokens will perform seasonal migrations.
    ///
    /// Defaults to [1,1] if not specified.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[is_empty(value = [1,1])]
    cluster_number: Option<[u32; 2]>,
    /// The minimum/maximum numbers of how many of these creatures are present in each world map tile of the appropriate region.
    ///
    /// Defaults to [1,1] if not specified.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[is_empty(value = [1,1])]
    population_number: Option<[u32; 2]>,
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
    underground_depth: Option<[u32; 2]>,
    /// Like `[BABYNAME]`, but applied regardless of caste.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    general_baby_name: Option<Name>,
    /// Like `[CHILDNAME]`, but applied regardless of caste.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    general_child_name: Option<Name>,
    /// The generic name for any creature of this type - will be used when distinctions between caste are unimportant. For names for specific castes,
    /// use `[CASTE_NAME]` instead. If left undefined, the creature will be labeled as "nothing" by the game.
    name: Name,

    /// Copies another specified creature. This will override any definitions made before it; essentially, it makes this creature identical to the other one,
    /// which can then be modified. Often used in combination with `[APPLY_CREATURE_VARIATION]` to import standard variations from a file.
    ///
    /// The vanilla giant animals and animal peoples are examples of this token combination.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    copy_tags_from: Option<String>,
    /// Applies the specified creature variation.
    ///
    /// These are stored "in the raw", i.e. how they appear in the raws. They are not handled until the end of the parsing process.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    apply_creature_variation: Option<Vec<String>>,
    /// A generated id that is used to uniquely identify this object. It is generated from the `metadata`, `identifier`, and `ObjectType`.
    ///
    /// This field is always serialized.
    object_id: Uuid,
    /// Various `SELECT_CREATUR` modifications.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    select_creature_variation: Option<Vec<SelectCreature>>,
}

impl Creature {
    /// Returns a `Creature` object with default values.
    ///
    /// # Returns
    ///
    /// An empty instance of `Creature`.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            metadata: Some(
                RawMetadata::default()
                    .with_object_type(ObjectType::CreatureVariation)
                    .with_hidden(true),
            ),
            castes: vec![Caste::new("ALL")],
            population_number: Some([1, 1]),
            cluster_number: Some([1, 1]),
            frequency: Some(50),
            ..Self::default()
        }
    }

    /// Create a new instance of a `Creature` with the given identifier and metadata.
    ///
    /// # Arguments
    ///
    /// * `identifier`: A string that represents the identifier of the creature. It is used to uniquely
    ///   identify the creature.
    /// * `metadata`: The `metadata` parameter is of type `RawMetadata` and is used to provide
    ///   additional information about the raws the `Creature` is found in.
    ///
    /// # Returns
    ///
    /// a `Creature` object.
    #[must_use]
    pub fn new(identifier: &str, metadata: &RawMetadata) -> Self {
        Self {
            identifier: String::from(identifier),
            metadata: Some(metadata.clone()),
            frequency: Some(50),
            castes: vec![Caste::new("ALL")],
            population_number: Some([1, 1]),
            cluster_number: Some([1, 1]),
            object_id: generate_object_id_using_raw_metadata(
                identifier,
                ObjectType::Creature,
                metadata,
            ),
            ..Self::default()
        }
    }

    /// The function `get_copy_tags_from` returns a reference to the `copy_tags_from` field.
    ///
    /// # Returns
    ///
    /// The private field `copy_tags_from`.
    #[must_use]
    pub fn get_copy_tags_from(&self) -> &str {
        self.copy_tags_from
            .as_ref()
            .map_or("", |copy_tags_from| copy_tags_from.as_str())
    }

    /// Get the identifiers of creature variations to apply.
    ///
    /// # Returns
    ///
    /// A slice of strings representing the identifiers of creature variations to apply.
    #[must_use]
    pub fn get_variations_to_apply(&self) -> &[String] {
        self.apply_creature_variation
            .as_ref()
            .map_or(&[], |apply_creature_variation| {
                apply_creature_variation.as_slice()
            })
    }

    /// Adds a `SelectCreature` object to the internal `SelectCreature` vector.
    ///
    /// # Arguments
    ///
    /// * `select_creature`: The parameter `select_creature` is of type `SelectCreature`.
    pub fn push_select_creature_variation(&mut self, select_creature: SelectCreature) {
        if self.select_creature_variation.is_none() {
            self.select_creature_variation = Some(Vec::new());
        }
        if let Some(select_creature_variation) = self.select_creature_variation.as_mut() {
            select_creature_variation.push(select_creature);
        } else {
            warn!(
                "Creature::push_select_creature_variation: ({}) select_creature_variation is None",
                self.identifier
            );
        }
    }

    /// Extends the internal `SelectCreature` vector with the elements from the `select_creature_vec`
    /// vector. This is a convenience function to enable bulk addition of `SelectCreature` objects.
    ///
    /// # Arguments
    ///
    /// * `select_creature_vec`: A vector of `SelectCreature` objects.
    pub fn extend_select_creature_variation(&mut self, select_creature_vec: Vec<SelectCreature>) {
        if self.select_creature_variation.is_none() {
            self.select_creature_variation = Some(Vec::new());
        }
        if let Some(select_creature_variation) = &mut self.select_creature_variation {
            select_creature_variation.extend(select_creature_vec);
        } else {
            warn!(
                "Creature::extend_select_creature_variation: ({}) select_creature_variation is None",
                self.identifier
            );
        }
    }

    /// The function `add_caste` adds a new `Caste` object with the given name to a vector called
    /// `castes`.
    ///
    /// # Arguments
    ///
    /// * `name`: The `name` parameter is a string that represents the name of the caste to add.
    pub fn add_caste(&mut self, name: &str) {
        self.castes.push(Caste::new(name));
    }

    /// The function `select_caste` moves a caste to the end of a list if it matches the given name,
    /// otherwise it adds a new caste with the given name. This essentially allows the other functions
    /// to assume that the caste they are working with is the last one in the list.
    ///
    /// # Arguments
    ///
    /// * `name`: The `name` parameter is a string that represents the identifier of the caste to select.
    pub fn select_caste(&mut self, name: &str) {
        // Find the caste
        let mut index = 0;
        for (i, caste) in self.castes.iter().enumerate() {
            if caste.get_identifier().eq(name) {
                index = i;
                break;
            }
        }

        if index == 0 {
            // If we have no castes, add a new one
            if self.castes.is_empty() {
                return self.add_caste(name);
            } else if let Some(caste) = self.castes.get(index) {
                // (If we're here, we're at index 0 and the caste list is not empty)
                // If the caste doesn't match the one we need, add a new one
                if !caste.get_identifier().eq(name) {
                    return self.add_caste(name);
                }
            }
        }

        // Move the caste to the end of the list
        let caste = self.castes.remove(index);
        self.castes.push(caste);
    }

    /// Checks if a given name exists in the list of castes.
    ///
    /// # Arguments
    ///
    /// * `name`: A string representing the `identifier` of the caste to check for.
    ///
    /// # Returns
    ///
    /// Returns true if there is a caste with the given name in this creature's caste list,
    /// and false otherwise.
    #[must_use]
    pub fn has_caste(&self, name: &str) -> bool {
        for caste in &self.castes {
            if caste.get_identifier().eq(name) {
                return true;
            }
        }
        false
    }

    /// Returns a vector of object IDs from the creature's `SelectCreature` vector. Essentially,
    /// it's the list of object IDs that have been added to this creature and then can be removed
    /// from the master raw list.
    ///
    /// # Returns
    ///
    /// Returns a vector of `object_id`s.
    pub fn get_child_object_ids(&self) -> Vec<Uuid> {
        self.select_creature_variation
            .as_ref()
            .map_or_else(Vec::new, |select_creature_variation| {
                select_creature_variation
                    .iter()
                    .map(RawObject::get_object_id)
                    .collect()
            })
    }

    /// Takes two `Creature` objects and creates a new `Creature` object
    /// by combining their tags and properties.
    ///
    /// # Arguments
    ///
    /// * `creature`: A reference to the creature that will receive the copied tags.
    /// * `creature_to_copy_from`: A reference to the Creature object from which we want to copy the
    ///   tags.
    ///
    /// # Returns
    ///
    /// A combined `Creature`, which is a combination of the original creature and the
    /// creature to copy from.
    #[must_use]
    pub fn copy_tags_from(creature: &Self, creature_to_copy_from: &Self) -> Self {
        // Because anything specified in our self will override the copied tags, first we need to clone the creature
        let mut combined_creature = creature_to_copy_from.clone();
        // Now apply any tags that exist for us but not for the one we copy.
        // So we need to go through all our properties and castes and overwrite what exists on the combined creature.

        // our metadata is preserved
        combined_creature.metadata.clone_from(&creature.metadata);
        // our identifier is preserved
        combined_creature
            .identifier
            .clone_from(&creature.identifier);
        // our `object_id` is preserved
        combined_creature.object_id.clone_from(&creature.object_id);

        // Clean the "creature" to remove any empty lists or strings for comparison
        let creature = creature.cleaned();

        // We need to loop over our castes and apply any differences.
        for caste in &creature.castes {
            let caste_identifier = caste.get_identifier();
            // If the caste exists in the combined creature, we need to apply the differences
            if combined_creature.has_caste(caste_identifier) {
                combined_creature.select_caste(caste_identifier);
                if let Some(combined_caste) = combined_creature.castes.last_mut() {
                    combined_caste.overwrite_caste(caste);
                }
            } else {
                // If the caste does not exist in the combined creature, we need to add it
                combined_creature.castes.push(caste.clone());
            }
        }

        // Loop over our pref_strings and if they aren't in combined_creature, add them
        if let Some(pref_strings) = &creature.pref_strings {
            let mut combined_pref_strings = combined_creature.pref_strings.unwrap_or_default();
            for pref_string in pref_strings {
                if !combined_pref_strings.contains(pref_string) {
                    combined_pref_strings.push(pref_string.clone());
                }
            }
            combined_creature.pref_strings = Some(combined_pref_strings);
        }

        // Loop over our biomes and if they aren't in combined_creature, add them
        if let Some(biomes) = &creature.biomes {
            let mut combined_biomes = combined_creature.biomes.unwrap_or_default();
            for biome in biomes {
                if !combined_biomes.contains(biome) {
                    combined_biomes.push(*biome);
                }
            }
            combined_creature.biomes = Some(combined_biomes);
        }

        // Loop over our tags and if they aren't in combined_creature, add them
        if let Some(tags) = &creature.tags {
            let mut combined_tags = combined_creature.tags.unwrap_or_default();
            for tag in tags {
                if !combined_tags.contains(tag) {
                    combined_tags.push(tag.clone());
                }
            }
            combined_creature.tags = Some(combined_tags);
        }

        // If any of our other properties are not default, we need to apply them to the combined creature.
        if creature.frequency.is_some() {
            combined_creature.frequency = creature.frequency;
        }
        if creature.cluster_number.is_some() {
            combined_creature.population_number = creature.population_number;
        }
        if creature.cluster_number.is_some() {
            combined_creature.cluster_number = creature.cluster_number;
        }
        if creature.underground_depth.is_some() {
            combined_creature.underground_depth = creature.underground_depth;
        }
        if creature.general_baby_name.is_some() {
            combined_creature
                .general_baby_name
                .clone_from(&creature.general_baby_name);
        }
        if creature.general_child_name.is_some() {
            combined_creature
                .general_child_name
                .clone_from(&creature.general_child_name);
        }
        if !creature.name.is_empty() {
            combined_creature.name = creature.name.clone();
        }
        if creature.tile.is_some() {
            combined_creature.tile.clone_from(&creature.tile);
        }

        combined_creature
    }

    /// The function `get_castes` returns a slice of `Caste` objects.
    ///
    /// # Returns
    ///
    /// The castes that belong to this creature.
    #[must_use]
    pub fn get_castes(&self) -> &[Caste] {
        self.castes.as_slice()
    }

    /// Get a list of tags that belong to this creature.
    #[must_use]
    pub fn get_tags(&self) -> Vec<CreatureToken> {
        if self.tags.is_none() {
            return Vec::new();
        }

        let mut ret_tags = Vec::new();
        if let Some(tags) = &self.tags {
            for tag in tags {
                ret_tags.push(tag.clone());
            }
        }
        ret_tags
    }
    /// Get the biomes the creature can be found in.
    #[must_use]
    pub fn get_biomes(&self) -> Vec<BiomeToken> {
        if self.biomes.is_none() {
            return Vec::new();
        }

        let mut ret_biomes = Vec::new();
        if let Some(biomes) = &self.biomes {
            for biome in biomes {
                ret_biomes.push(*biome);
            }
        }
        ret_biomes
    }
    /// Set the name of the creature.
    ///
    /// # Parameters
    ///
    /// * `name`: The name to set for the creature
    pub fn set_name(&mut self, name: Name) {
        self.name = name;
    }

    pub fn get_all_names(&self) -> Vec<&str> {
        let mut names = HashSet::new();

        names.insert(self.name.get_singular());
        names.insert(self.name.get_plural());
        names.insert(self.name.get_adjective());

        if let Some(general_baby_name) = self.general_baby_name.as_ref() {
            names.insert(general_baby_name.get_singular());
            names.insert(general_baby_name.get_plural());
            names.insert(general_baby_name.get_adjective());
        }

        if let Some(general_child_name) = self.general_child_name.as_ref() {
            names.insert(general_child_name.get_singular());
            names.insert(general_child_name.get_plural());
            names.insert(general_child_name.get_adjective());
        }

        self.castes.iter().for_each(|caste| {
            if let Some(caste_name) = caste.get_caste_name() {
                names.insert(caste_name.get_singular());
                names.insert(caste_name.get_plural());
                names.insert(caste_name.get_adjective());
            }
            if let Some(caste_child_name) = caste.get_child_name() {
                names.insert(caste_child_name.get_singular());
                names.insert(caste_child_name.get_plural());
                names.insert(caste_child_name.get_adjective());
            }
            if let Some(caste_baby_name) = caste.get_baby_name() {
                names.insert(caste_baby_name.get_singular());
                names.insert(caste_baby_name.get_plural());
                names.insert(caste_baby_name.get_adjective());
            }
        });

        names.into_iter().collect()
    }
    pub fn get_all_descriptions(&self) -> Vec<&str> {
        let mut descriptions = HashSet::new();

        self.castes.iter().for_each(|caste| {
            if let Some(description) = caste.get_description() {
                descriptions.insert(description);
            }
        });

        descriptions.into_iter().collect()
    }
    /// Parse a creature from a set of XML tags from a legends export.
    ///
    /// Expects to run on an empty or default creature. Fills in everything it can
    /// from the XML tags. It's likely that `<creature>` objects are only in
    /// legends-plus exports, which are enhanced from the base legends export by dfhack.
    ///
    /// # Parameters
    ///
    /// * `xml_tags`: A vector of strings representing the XML tags for the creature.
    pub fn parse_tags_from_xml(&mut self, xml_tags: &[String]) {
        for tag in xml_tags {
            if tag.contains("has_male") {
                self.add_caste("MALE");
            } else if tag.contains("has_female") {
                self.add_caste("FEMALE");
            } else if tag.starts_with("biome_") {
                // Parse the biome from "biome_pool_temperate_freshwater" or "biome_savanna_temperate"
                let biome = tag
                    .split('_')
                    .skip(1)
                    .collect::<Vec<&str>>()
                    .join("_")
                    .to_uppercase();
                if let Some(biome) = BIOME_TOKENS.get(&biome) {
                    self.add_biome(*biome);
                } else {
                    warn!(
                        "Creature::parse_tags_from_xml: ({}) Unknown biome '{}'",
                        self.identifier, biome
                    );
                }
            } else if tag.starts_with("has_any_") {
                // Remove the "has_any_" prefix and parse the caste tag
                let mut caste_tag = tag
                    .split('_')
                    .skip(2)
                    .collect::<Vec<&str>>()
                    .join("_")
                    .to_uppercase();
                // Handle some edge cases
                if caste_tag.ends_with("INTELLIGENT_LEARNS") {
                    caste_tag = String::from("CAN_LEARN");
                } else if caste_tag.ends_with("INTELLIGENT_SPEAKS") {
                    caste_tag = String::from("CAN_SPEAK");
                } else if caste_tag.ends_with("CAN_SWIM") {
                    caste_tag = String::from("SWIMS_INNATE");
                } else if caste_tag.ends_with("FLY_RACE_GAIT") {
                    caste_tag = String::from("FLIER");
                }
                // Parse the tag
                if let Some(_caste_tag) = CASTE_TOKENS.get(&caste_tag) {
                    self.select_caste("ALL");
                    if let Some(caste) = self.castes.last_mut() {
                        caste.parse_tag(caste_tag.as_str(), "");
                    } else {
                        debug!(
                            "Creature::parse_tags_from_xml: ({}) No castes found to apply tag {}",
                            self.identifier, caste_tag
                        );
                    }
                } else {
                    // Try parsing the tag as a creature tag
                    if let Some(tag) = CREATURE_TOKENS.get(&caste_tag) {
                        self.add_tag(tag.clone());
                    } else {
                        warn!(
                            "Creature::parse_tags_from_xml: ({}) Unknown tag {}",
                            self.identifier, caste_tag
                        );
                    }
                }
            } else {
                // Try to parse the tag
                if let Some(tag) = CREATURE_TOKENS.get(&tag.to_uppercase()) {
                    self.add_tag(tag.clone());
                } else {
                    warn!(
                        "Creature::parse_tags_from_xml: ({}) Unknown tag {}",
                        self.identifier, tag
                    );
                }
            }
        }
    }

    /// Add a tag to the creature.
    ///
    /// This handles making sure the tags vector is initialized.
    pub fn add_tag(&mut self, tag: CreatureToken) {
        if self.tags.is_none() {
            self.tags = Some(Vec::new());
        }
        if let Some(tags) = self.tags.as_mut() {
            tags.push(tag);
        } else {
            warn!(
                "Creature::add_tag: ({}) Failed to add tag {:?}",
                self.identifier, tag
            );
        }
    }

    /// Add a biome to the creature.
    ///
    /// This handles making sure the biomes vector is initialized.
    pub fn add_biome(&mut self, biome: BiomeToken) {
        if self.biomes.is_none() {
            self.biomes = Some(Vec::new());
        }
        if let Some(biomes) = self.biomes.as_mut() {
            biomes.push(biome);
        } else {
            warn!(
                "Creature::add_biome: ({}) Failed to add biome {:?}",
                self.identifier, biome
            );
        }
    }

    /// Check whether the creature has the specified creature tag (found in the `tags` field).
    ///
    /// # Arguments
    ///
    /// * `tag`: The tag to check for.
    ///
    /// # Returns
    ///
    /// Returns true if the creature has the specified tag, and false otherwise.
    #[must_use]
    pub fn has_tag(&self, tag: &CreatureToken) -> bool {
        if let Some(tags) = &self.tags {
            for t in tags {
                if std::mem::discriminant(t) == std::mem::discriminant(tag) {
                    return true;
                }
            }
        }
        false
    }

    /// Check whether the creature has a specified biome.
    ///
    /// # Arguments
    ///
    /// * `biome`: The biome to check for.
    ///
    /// # Returns
    ///
    /// Returns true if the creature has the specified biome, and false otherwise.
    #[must_use]
    pub fn has_biome(&self, biome: &BiomeToken) -> bool {
        if let Some(biomes) = &self.biomes {
            for b in biomes {
                if b == biome {
                    return true;
                }
            }
        }
        false
    }

    /// Check whether any of the castes have the specified creature caste tag.
    ///
    /// # Arguments
    ///
    /// * `tag`: The tag to check for.
    ///
    /// # Returns
    ///
    /// Returns true if any of the castes have the specified tag, and false otherwise.
    #[must_use]
    pub fn has_caste_tag(&self, tag: &CasteToken) -> bool {
        for caste in &self.castes {
            if caste.has_tag(tag) {
                return true;
            }
        }
        false
    }
}

#[typetag::serde]
impl RawObject for Creature {
    fn get_metadata(&self) -> RawMetadata {
        self.metadata.as_ref().map_or_else(
            || {
                warn!(
                    "Creature::get_metadata: ({}) metadata is None",
                    self.identifier
                );
                RawMetadata::default()
                    .with_object_type(ObjectType::Creature)
                    .with_hidden(true)
            },
            std::clone::Clone::clone,
        )
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
    fn get_type(&self) -> ObjectType {
        ObjectType::Creature
    }
    #[allow(clippy::too_many_lines)]
    fn parse_tag(&mut self, key: &str, value: &str) {
        if CASTE_TOKENS.contains_key(key) {
            #[allow(clippy::unwrap_used)]
            self.castes.last_mut().unwrap().parse_tag(key, value);
            return;
        }
        if !CREATURE_TOKENS.contains_key(key) {
            trace!("parse_tag: unknown tag {} with value {}", key, value);
            return;
        }

        let Some(tag) = CreatureToken::parse(key, value) else {
            warn!("parse_tag: unknown tag {} with value {}", key, value);
            return;
        };

        self.add_tag(tag.clone());

        match tag {
            CreatureToken::Biome { id } => {
                if let Some(biome) = BIOME_TOKENS.get(&id) {
                    self.add_biome(*biome);
                } else {
                    warn!(
                        "parse_tag: unknown biome {} for creature {}",
                        id, self.identifier
                    );
                }
            }
            CreatureToken::Name { .. } => {
                self.name = Name::from_value(value);
            }
            CreatureToken::GeneralBabyName { .. } => {
                self.general_baby_name = Some(Name::from_value(value));
            }
            CreatureToken::GeneralChildName { .. } => {
                self.general_child_name = Some(Name::from_value(value));
            }
            CreatureToken::PrefString { pref_string } => {
                if let Some(pref_strings) = self.pref_strings.as_mut() {
                    pref_strings.push(pref_string);
                } else {
                    self.pref_strings = Some(vec![pref_string]);
                }
            }
            CreatureToken::PopulationNumber { min, max } => {
                self.population_number = Some([min, max]);
            }
            CreatureToken::Frequency { frequency } => {
                self.frequency = Some(frequency);
            }
            CreatureToken::UndergroundDepth { min, max } => {
                self.underground_depth = Some([min, max]);
            }
            CreatureToken::ClusterNumber { min, max } => {
                self.cluster_number = Some([min, max]);
            }
            CreatureToken::CopyTagsFrom { creature } => {
                self.copy_tags_from = Some(creature);
            }
            CreatureToken::ApplyCreatureVariation { .. } => {
                if let Some(apply_creature_variation) = self.apply_creature_variation.as_mut() {
                    apply_creature_variation.push(String::from(value));
                } else {
                    self.apply_creature_variation = Some(vec![String::from(value)]);
                }
            }
            CreatureToken::CreatureTile { .. } => {
                if self.tile.is_none() {
                    self.tile = Some(Tile::default());
                }
                if let Some(tile) = self.tile.as_mut() {
                    tile.set_character(value);
                }
            }
            CreatureToken::AltTile { .. } => {
                if self.tile.is_none() {
                    self.tile = Some(Tile::default());
                }
                if let Some(tile) = self.tile.as_mut() {
                    tile.set_alt_character(value);
                }
            }
            CreatureToken::Color { .. } => {
                if self.tile.is_none() {
                    self.tile = Some(Tile::default());
                }
                if let Some(tile) = self.tile.as_mut() {
                    tile.set_color(value);
                }
            }
            CreatureToken::GlowColor { .. } => {
                if self.tile.is_none() {
                    self.tile = Some(Tile::default());
                }
                if let Some(tile) = self.tile.as_mut() {
                    tile.set_glow_color(value);
                }
            }
            CreatureToken::GlowTile { .. } => {
                if self.tile.is_none() {
                    self.tile = Some(Tile::default());
                }
                if let Some(tile) = self.tile.as_mut() {
                    tile.set_glow_character(value);
                }
            }
            _ => {}
        }
    }
    fn get_object_id(&self) -> Uuid {
        self.object_id
    }
    fn get_name(&self) -> &str {
        self.name.get_singular()
    }
    fn get_searchable_tokens(&self) -> Vec<&str> {
        let mut tokens = HashSet::new();

        for token in CreatureToken::FLAG_TOKENS {
            if self.has_tag(token) {
                tokens.insert(CreatureToken::get_key(token).unwrap_or_default());
            }
        }

        for caste in &self.castes {
            for token in CasteToken::FLAG_TOKENS {
                if caste.has_tag(token) {
                    tokens.insert(CasteToken::get_key(token).unwrap_or_default());
                }
            }
        }

        tokens.into_iter().collect()
    }
    fn get_numeric_flags(&self) -> Vec<NumericToken> {
        let mut tokens = Vec::new();

        // 1. Collect from Creature Tags
        if let Some(tags) = &self.tags {
            for tag in tags {
                tokens.extend(tag.as_numeric_tokens());
            }
        }

        // 2. Collect from Caste Tags
        for caste in &self.castes {
            for tag in caste.get_tags() {
                tokens.extend(tag.as_numeric_tokens());
            }
        }

        tokens
    }
    fn get_module_object_id(&self) -> Uuid {
        match &self.metadata {
            Some(meta) => meta.get_module_object_id(),
            None => Uuid::nil(),
        }
    }
}

#[typetag::serde]
impl CreatureVariationRequirements for Creature {
    fn remove_tag(&mut self, key: &str) {
        self.remove_tag_and_value(key, "");
    }

    fn remove_tag_and_value(&mut self, key: &str, value: &str) {
        if CASTE_TOKENS.contains_key(key) {
            #[allow(clippy::unwrap_used)]
            self.castes
                .last_mut()
                .unwrap()
                .remove_tag_and_value(key, value);
            return;
        }
        if !CREATURE_TOKENS.contains_key(key) {
            debug!("CreatureParsing: Unknown tag {} with value {}", key, value);
            return;
        }

        let Some(tag) = CREATURE_TOKENS.get(key) else {
            warn!(
                "CreatureParsing: called `Option::unwrap()` on a `None` value for presumed creature tag: {}",
                key
            );
            return;
        };

        match tag {
            CreatureToken::Biome { .. } => {
                let Some(biome) = BIOME_TOKENS.get(value) else {
                    warn!(
                        "CreatureParsing: called `Option::unwrap()` on a `None` value for presumed biome: {}",
                        value
                    );
                    return;
                };
                if let Some(biomes) = self.biomes.as_mut() {
                    biomes.retain(|x| x != biome);
                }
            }
            CreatureToken::Name { .. } => {
                self.name = Name::default();
            }
            CreatureToken::GeneralBabyName { .. } => {
                self.general_baby_name = None;
            }
            CreatureToken::GeneralChildName { .. } => {
                self.general_child_name = None;
            }
            CreatureToken::PrefString { .. } => {
                if let Some(pref_strings) = self.pref_strings.as_mut() {
                    pref_strings.retain(|x| x != value);
                }
            }
            CreatureToken::PopulationNumber { .. } => {
                self.population_number = None;
            }
            CreatureToken::Frequency { .. } => {
                self.frequency = None;
            }
            CreatureToken::UndergroundDepth { .. } => {
                self.underground_depth = None;
            }
            CreatureToken::ClusterNumber { .. } => {
                self.cluster_number = None;
            }
            CreatureToken::CopyTagsFrom { .. } => {
                self.copy_tags_from = None;
            }
            CreatureToken::ApplyCreatureVariation { .. } => {
                if let Some(apply_creature_variation) = self.apply_creature_variation.as_mut() {
                    apply_creature_variation.retain(|x| x != value);
                }
            }
            CreatureToken::CreatureTile { .. } => {
                if let Some(tile) = self.tile.as_mut() {
                    tile.set_character("");
                }
            }
            CreatureToken::AltTile { .. } => {
                if let Some(tile) = self.tile.as_mut() {
                    tile.set_alt_character("");
                }
            }
            CreatureToken::Color { .. } => {
                if let Some(tile) = self.tile.as_mut() {
                    tile.set_color("");
                }
            }
            CreatureToken::GlowColor { .. } => {
                if let Some(tile) = self.tile.as_mut() {
                    tile.set_glow_color("");
                }
            }
            CreatureToken::GlowTile { .. } => {
                if let Some(tile) = self.tile.as_mut() {
                    tile.set_glow_character("");
                }
            }
            _ => {
                if let Some(tags) = self.tags.as_mut() {
                    tags.retain(|x| x != tag);
                }
            }
        }
    }

    fn remove_tag_for_caste(&mut self, key: &str, caste: &str) {
        self.select_caste(caste);
        self.remove_tag(key);
    }

    fn remove_tag_and_value_for_caste(&mut self, key: &str, value: &str, caste: &str) {
        self.select_caste(caste);
        self.remove_tag_and_value(key, value);
    }

    fn add_tag(&mut self, key: &str) {
        self.parse_tag(key, "");
    }

    fn add_tag_and_value(&mut self, key: &str, value: &str) {
        self.parse_tag(key, value);
    }

    fn add_tag_for_caste(&mut self, key: &str, caste: &str) {
        self.select_caste(caste);
        self.parse_tag(key, "");
    }

    fn add_tag_and_value_for_caste(&mut self, key: &str, value: &str, caste: &str) {
        self.select_caste(caste);
        self.parse_tag(key, value);
    }
}

impl Searchable for Creature {
    fn get_search_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();
        // Add caste search strings
        for caste in &self.castes {
            vec.extend(caste.get_search_vec());
        }
        // Add tags
        if let Some(tags) = &self.tags {
            for tag in tags {
                vec.push(tag.to_string());
            }
        }
        // Add biomes
        if let Some(biomes) = &self.biomes {
            for biome in biomes {
                vec.push(biome.to_string());
            }
        }
        // Add pref strings
        if let Some(pref_strings) = &self.pref_strings {
            vec.extend(pref_strings.clone());
        }
        // Add name
        vec.extend(self.name.as_vec());
        // Add general baby name
        if let Some(general_baby_name) = &self.general_baby_name {
            vec.extend(general_baby_name.as_vec());
        }
        // Add general child name
        if let Some(general_child_name) = &self.general_child_name {
            vec.extend(general_child_name.as_vec());
        }
        // Add identifier
        vec.push(self.identifier.clone());

        clean_search_vec(vec.as_slice())
    }
}
