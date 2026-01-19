use dfraw_parser::metadata::RawModuleLocation;
use rusqlite::{Connection, Result, params};

use super::super::rusqlite_extensions::OptionalResultExtension;

/// Returns true if the module exists in the database.
///
/// Searches based on key identifiers: identifier, version and location
///
/// # Errors
///
/// - database error
pub fn exists_module_by_identifiers(
    conn: &Connection,
    identifier: &str,
    numeric_version: i64,
    location: RawModuleLocation,
) -> Result<bool> {
    match try_get_module_id_by_identifiers(conn, identifier, numeric_version, location) {
        Ok(res) => Ok(res.is_some()),
        Err(e) => Err(e),
    }
}

/// Attempts to find the database ID for a specific module.
///
/// Searches to find an existing module in the database based on key identifying
/// factors: identifier, version and location.
///
/// # Errors
///
/// - database error
pub fn try_get_module_id_by_identifiers(
    conn: &Connection,
    identifier: &str,
    numeric_version: i64,
    location: RawModuleLocation,
) -> Result<Option<i64>> {
    conn.query_row(
        "SELECT id FROM modules
         WHERE identifier = ?1
           AND version = ?2
           AND module_location_id = ?3
         LIMIT 1",
        params![identifier, numeric_version, location as i32],
        |row| row.get(0),
    )
    .optional()
}
