//! Contains the raw SQL text for migrating the database to different versions.
mod sql_001_initial;
mod sql_002_names;
mod sql_003_graphics;
mod sql_004_db_metadata;
mod sql_005_unique_module_raw_ident;

/// The highest (and most recent) schema version.
pub const LATEST_SCHEMA_VERSION: i32 = 5;

/// Migrations forward in the format (`schema_version`, SQL), in order of ascending schema version.
pub(super) const UP_MIGRATIONS: [(i32, &str); 5] = [
    (1, sql_001_initial::UP),
    (2, sql_002_names::UP),
    (3, sql_003_graphics::UP),
    (4, sql_004_db_metadata::UP),
    (5, sql_005_unique_module_raw_ident::UP),
];
/// Migrations backward in in the format (`previous_schema_version`, SQL), in order of ascending schema version.
pub(super) const DOWN_MIGRATIONS: [(i32, &str); 5] = [
    (0, sql_001_initial::DOWN),
    (1, sql_002_names::DOWN),
    (2, sql_003_graphics::DOWN),
    (3, sql_004_db_metadata::DOWN),
    (4, sql_005_unique_module_raw_ident::DOWN),
];
