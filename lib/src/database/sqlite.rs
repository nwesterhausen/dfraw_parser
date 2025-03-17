
use rusqlite::{Connection, Result};

/// Load a SQLite database from a file.
///
/// # Arguments
///
/// * `filename` - The path to the SQLite database file.
///
/// # Returns
///
/// A `rusqlite::Connection` to the database.
pub(super) fn load_database(filename: &str) -> Result<Connection> {
    let conn = Connection::open(filename)?;
    Ok(conn)
}

/// Apply sql to a SQLite database.
///
/// # Arguments
///
/// * `conn` - The `rusqlite::Connection` to the database.
/// * `sql` - The SQL to apply.
///
/// # Returns
///
/// An empty `Result` if successful.
///
/// # Errors
///
/// If the SQL fails to execute.
pub(super) fn apply_sql(
    sql: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = super::get_db_conn();
    conn.execute_batch(sql)?;
    Ok(())
}

/// Get the user version of a SQLite database, via the `PRAGMA user_version;` statement.
/// We use this to track the current version of the database schema.
///
/// # Arguments
///
/// * `conn` - The `rusqlite::Connection` to the database.
///
/// # Returns
///
/// The user version of the database.
///
/// # Errors
///
/// If the SQL fails to execute.
pub(crate) fn get_user_version() -> Result<i32, rusqlite::Error> {
    let conn = super::get_db_conn();
    let mut stmt = conn.prepare("PRAGMA user_version;")?;
    let version: i32 = stmt.query_row([], |row| row.get(0))?;
    Ok(version)
}

/// Set the user version of a SQLite database, via the `PRAGMA user_version = ?;` statement.
///
/// # Arguments
///
/// * `version` - The new user version to set.
///
/// # Returns
///
/// An empty `Result` if successful.
///
/// # Errors
///
/// If the SQL fails to execute.
pub(super) fn set_user_version(version: i32) -> Result<(), rusqlite::Error> {
    let conn = super::get_db_conn();
    conn.execute("PRAGMA user_version = ?;", [version])?;
    Ok(())
}