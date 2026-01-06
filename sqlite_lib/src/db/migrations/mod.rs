//! Contains the raw SQL text for migrating the database to different versions.
mod sql_001_initial;
mod sql_002_names;
mod sql_003_graphics;

/// The highest (and most recent) schema version.
pub const LATEST_SCHEMA_VERSION: i32 = 3;

/// Migrations forward in the format (`schema_version`, SQL), in order of ascending schema version.
pub(super) const UP_MIGRATIONS: [(i32, &str); 3] = [
    (1, sql_001_initial::UP),
    (2, sql_002_names::UP),
    (3, sql_003_graphics::UP),
];
/// Migrations backward in in the format (`previous_schema_version`, SQL), in order of ascending schema version.
pub(super) const DOWN_MIGRATIONS: [(i32, &str); 3] = [
    (0, sql_001_initial::DOWN),
    (1, sql_002_names::DOWN),
    (2, sql_003_graphics::DOWN),
];
