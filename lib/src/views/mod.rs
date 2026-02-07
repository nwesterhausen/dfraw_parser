//! Data view models for the various structs/types of raw objects.
//!
//! These provide a nice way to access details about the various raws, and can be easily
//! created from the parsed structs.

mod caste;
mod creature;

pub use caste::CasteView;
pub use creature::CreatureView;
