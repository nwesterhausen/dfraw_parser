//! Utility functions and helpers for parsing Dwarf Fortress raw files.
//!
//! This module provides various utilities including:
//! - File operations for reading and writing raw files
//! - Directory lookup functions for Steam and user data directories
//! - Tag lookup tables for creatures, plants, entities, and more
//! - Flag constants for various game object types
//!
//! # Examples
//!
//! Finding the game installation path:
//!
//! ```
//! use dfraw_parser::utilities::find_game_path;
//!
//! let app_id = 975370
//! if let Some(path) = find_game_path(app_id) {
//!     println!("Game found at: {:?}", path);
//! }
//! ```

mod biome_tag_lookup;
mod caste_tag_flags;
mod condition_tag_lookup;
mod creature_effect_property_tag_lookup;
mod creature_effect_tag_lookup;
mod creature_tag_flags;
mod creature_variation_tag_lookup;
mod entity_tag_lookup;
mod file_operations;
mod object_type_lookup;
mod plant_growth_tag_lookup;
mod plant_growth_type_tag_flags;
mod plant_growth_type_tag_lookup;
mod plant_part_tag_lookup;
mod plant_tag_flags;
mod plant_tag_lookup;
mod searchable;
mod steam_directory_lookup;
mod user_directory_lookup;

pub use file_operations::*;
pub use searchable::*;
pub use steam_directory_lookup::find_game_path;
pub use user_directory_lookup::find_user_data_path;
