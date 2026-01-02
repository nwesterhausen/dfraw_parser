//! Contains the raw SQL text for migrating the database to different versions.
mod sql_001_initial;

/// The highest (and most recent) schema version.
pub const LATEST_SCHEMA_VERSION: i32 = 1;

/// Migrations forward in the format (`schema_version`, SQL), in order of ascending schema version.
pub(super) const UP_MIGRATIONS: [(i32, &str); 1] = [(1, sql_001_initial::UP)];
/// Migrations backward in in the format (`previous_schema_version`, SQL), in order of ascending schema version.
pub(super) const DOWN_MIGRATIONS: [(i32, &str); 1] = [(0, sql_001_initial::DOWN)];
