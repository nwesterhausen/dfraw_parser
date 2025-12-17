//! Contains the SQL statements used to initialize the database.

mod caste;
mod creature;
mod creature_variation;
mod initialize;
mod metadata;
mod reference;
mod reference_data;
mod tile;

pub use initialize::initialize_database;
use reference_data::caste_flags::insert_ref_caste_flags;

/// The latest schema version of the database.
///
/// # History
/// - 1: Initial version
pub const LATEST_SCHEMA_VERSION: u32 = 1;
