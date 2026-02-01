//! # `ExportedCreature` struct
//!
//! The `ExportedCreature` struct is used to store information about a creature that has been exported
//! from the Legends Viewer.

use crate::{Creature, custom_types::Name, metadata::RawMetadata};

/// The `ExportedCreature` struct is used to store information about a creature that has been exported
#[derive(Debug, Default)]
pub struct ExportedCreature {
    creature_id: String,
    name_singular: String,
    name_plural: String,
    tags: Vec<String>,
}

impl ExportedCreature {
    /// Whether the creature is empty
    ///
    /// # Returns
    ///
    /// `true` if the creature is empty, `false` otherwise
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.creature_id.is_empty()
    }
    /// Set the creature ID
    ///
    /// # Arguments
    ///
    /// * `creature_id` - A `&str` that represents the creature ID
    pub fn set_creature_id(&mut self, creature_id: &str) {
        self.creature_id = creature_id.to_string();
    }
    /// Set the singular name of the creature
    ///
    /// # Arguments
    ///
    /// * `name_singular` - A `&str` that represents the singular name of the creature
    pub fn set_name_singular(&mut self, name_singular: &str) {
        self.name_singular = name_singular.to_string();
    }
    /// Set the plural name of the creature
    ///
    /// # Arguments
    ///
    /// * `name_plural` - A `&str` that represents the plural name of the creature
    pub fn set_name_plural(&mut self, name_plural: &str) {
        self.name_plural = name_plural.to_string();
    }
    /// Add a tag to the creature
    ///
    /// # Arguments
    ///
    /// * `tag` - A `&str` that represents the tag to add to the creature
    pub fn add_tag(&mut self, tag: &str) {
        self.tags.push(tag.to_string());
    }
    /// Convert the `ExportedCreature` struct into a `Creature` struct
    ///
    /// # Arguments
    ///
    /// * `metadata` - A reference to a `RawMetadata` struct
    ///
    /// # Returns
    ///
    /// A `Creature` struct
    #[must_use]
    pub fn into_creature(self, metadata: &RawMetadata) -> Creature {
        let mut creature = Creature::new(&self.creature_id, metadata);
        creature.set_name(Name::new(&self.name_singular, &self.name_plural, ""));
        creature.parse_tags_from_xml(self.tags.as_slice());

        creature
    }
}
