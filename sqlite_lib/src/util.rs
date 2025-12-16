//! Provides utility functions for the database.

/// Gets the schema version of the database.
///
/// # Errors
/// Returns an error if the schema version cannot be retrieved.
pub(crate) async fn get_schema_version(conn: &turso::Connection) -> Result<u32, turso::Error> {
    // PRAGMA user_version returns one row with one column
    let mut rows = conn.query("PRAGMA user_version;", ()).await?;

    if let Some(row) = rows.next().await? {
        // The value is in the first column (index 0)
        Ok(row.get(0)?)
    } else {
        Ok(0) // Default to 0 if something weird happens
    }
}
