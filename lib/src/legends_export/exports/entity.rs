//! The `ExportedEntity` struct is used to store information about an entity that has been exported
//! from the Legends Viewer.

use crate::{Entity, metadata::RawMetadata};

/// The `ExportedEntity` struct is used to store information about an entity that has been exported
#[derive(Debug, Default)]
pub struct ExportedEntity {
    entity_id: u32,
    race: String,
    entity_type: String,
    child_id: u32,
    // population: u32,
    // civ_id: u32,
    // positions: Vec<ExportedEntityPosition>,
}

// #[derive(Debug, Default)]
// pub(crate) struct ExportedEntityPosition {
//     id: u32,
//     name: String,
//     male_name: String,
//     female_name: String,
//     spouse_name: String,
//     male_spouse_name: String,
//     female_spouse_name: String,
// }

impl ExportedEntity {
    /// Set the entity ID
    ///
    /// # Arguments
    ///
    /// * `id` - A `u32` that represents the entity ID
    pub fn set_id(&mut self, id: u32) {
        self.entity_id = id;
    }
    /// Set the entity race
    ///
    /// # Arguments
    ///
    /// * `race` - A `&str` that represents the race of the entity
    pub fn set_race(&mut self, race: &str) {
        self.race = race.into();
    }
    /// Set the entity type
    ///
    /// # Arguments
    ///
    /// * `entity_type` - A `&str` that represents the type of the entity
    pub fn set_entity_type(&mut self, entity_type: &str) {
        self.entity_type = entity_type.into();
    }
    /// Set the child ID
    ///
    /// # Arguments
    ///
    /// * `child_id` - A `u32` that represents the child ID
    pub fn set_child_id(&mut self, child_id: u32) {
        self.child_id = child_id;
    }
    /// Convert the `ExportedEntity` struct into an `Entity` struct
    ///
    /// # Arguments
    ///
    /// * `legend_metadata` - A reference to a `RawMetadata` object
    ///
    /// # Returns
    ///
    /// An `Entity` object
    #[must_use]
    #[allow(dead_code)]
    pub fn into_entity(self, legend_metadata: &RawMetadata) -> Entity {
        Entity::new(
            format!("{}-{}{}", self.race, self.entity_type, self.entity_id).as_str(),
            legend_metadata,
        )
    }
    // pub fn set_population(&mut self, population: u32) {
    //     self.population = population;
    // }
    // pub fn set_civ_id(&mut self, civ_id: u32) {
    //     self.civ_id = civ_id;
    // }
}
