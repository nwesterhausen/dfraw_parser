//! The `Creature` struct represents a creature in a Dwarf Fortress, with the properties
//! that can be set in the raws. Not all the raws are represented here, only the ones that
//! are currently supported by the library.

use std::mem::discriminant;

use dfraw_parser_proc_macros::{Cleanable, IsEmpty};
use tracing::{debug, warn};
use uuid::Uuid;

use crate::{
    Caste, SelectCreature,
    metadata::RawMetadata,
    raw_definitions::{CASTE_TOKENS, CREATURE_TOKENS},
    tokens::{CasteToken, CreatureToken, ObjectType},
    traits::{Cleanable, CreatureVariationRequirements, RawObject},
    utilities::generate_object_id_using_raw_metadata,
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
    pub metadata: Option<RawMetadata>,
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
    pub castes: Vec<Caste>,
    /// Any tags that are not parsed into their own fields are stored in the `tags` field.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub tokens: Vec<CreatureToken>,
    /// Copies another specified creature. This will override any definitions made before it; essentially, it makes this creature identical to the other one,
    /// which can then be modified. Often used in combination with `[APPLY_CREATURE_VARIATION]` to import standard variations from a file.
    ///
    /// The vanilla giant animals and animal peoples are examples of this token combination.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub copy_tags_from: Option<String>,
    /// Applies the specified creature variation.
    ///
    /// These are stored "in the raw", i.e. how they appear in the raws. They are not handled until the end of the parsing process.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub apply_creature_variation: Option<Vec<String>>,
    /// Various `SELECT_CREATUR` modifications.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub select_creature_variation: Option<Vec<SelectCreature>>,
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
            castes: vec![Caste::new("ALL")],
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

        // Loop over our tags and if they aren't in combined_creature, add them
        let mut combined_tags = combined_creature.tokens;
        for tag in creature.tokens {
            if !combined_tags.contains(&tag) {
                combined_tags.push(tag.clone());
            }
        }
        combined_creature.tokens = combined_tags;

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
    pub fn get_tokens(&self) -> Vec<CreatureToken> {
        self.tokens.clone()
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
                self.tokens.push(CreatureToken::Biome { id: biome });
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
                        self.add_token(tag.clone());
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
                    self.add_token(tag.clone());
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
    pub fn add_token(&mut self, token: CreatureToken) {
        self.tokens.push(token);
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
        for t in &self.tokens {
            if std::mem::discriminant(t) == std::mem::discriminant(tag) {
                return true;
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

        let Some(token) = CREATURE_TOKENS.get(key) else {
            warn!(
                "CreatureParsing: called `Option::unwrap()` on a `None` value for presumed creature tag: {}",
                key
            );
            return;
        };

        if matches!(token, CreatureToken::Biome { .. })
            || matches!(token, CreatureToken::PrefString { .. })
        {
            self.tokens.retain(|t| t != token);
        } else {
            self.tokens
                .retain(|t| discriminant(t) != discriminant(token));
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
