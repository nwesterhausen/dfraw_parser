use dfraw_parser::metadata::RawModuleLocation;
use rusqlite::{Connection, Result, params};

/// Get the id of a `RawModuleLocation`
///
/// # Errors
///
/// - location not found in database
pub fn get_id_for_module_location(conn: &Connection, location: RawModuleLocation) -> Result<i64> {
    let name = location.to_string();

    conn.query_row(
        "SELECT id FROM module_locations WHERE name = ?1",
        params![name],
        |row| row.get(0),
    )
}
