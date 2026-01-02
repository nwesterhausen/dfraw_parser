//! Contains the SQL statements used to initialize the database.

mod caste;
mod color;
mod creature;
mod creature_variation;
mod initialize;
mod metadata;
mod misc;
mod reference;
mod reference_data;
mod tile;

pub use initialize::initialize_database;
use reference_data::biome_tags::insert_ref_biome_tags;
use reference_data::caste_tags::insert_ref_caste_tags;
use reference_data::condition_tags::insert_ref_condition_tags;
use reference_data::creature_effect_property_tags::insert_ref_creature_effect_property_tags;
use reference_data::creature_effect_tags::insert_ref_creature_effect_tags;
use reference_data::creature_tags::insert_ref_creature_tags;
use reference_data::creature_variation_tags::insert_ref_creature_variation_tags;
use reference_data::entity_tags::insert_ref_entity_tags;
use reference_data::lair_flags::insert_ref_lair_tags;
use reference_data::object_type::insert_ref_object_types;
use reference_data::secretion_triggers::insert_ref_secretion_triggers;

/// The latest schema version of the database.
///
/// # History
/// - 1: Initial version
pub const LATEST_SCHEMA_VERSION: u32 = 1;
