//! Provides utility functions for the database.
use std::fmt::Write;

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

/// Escape a string for safe inclusion in a SQL literal by doubling single quotes.
fn escape_sql_string(value: &str) -> String {
    value.replace('\'', "''")
}

/// Wrap a string in single quotes after escaping it for SQL.
///
/// This can be used directly for any string needed to be included in a SQL literal. This returns
/// a string that is surrounded by single quotes.
///
/// Example: `quoted_sql("O'Brien")` -> `'O''Brien'`
pub(crate) fn quoted_sql(value: &str) -> String {
    format!("'{}'", escape_sql_string(value))
}

/// Build a simple batch INSERT statement for a single-column table.
///
/// The function returns a concatenated series of `INSERT INTO ... VALUES ...;`
/// statements. It is intended to be executed with a single `execute_batch` call.
///
/// # Arguments
///
/// * `table` - table name (unquoted). Caller is responsible for ensuring it is a safe identifier.
/// * `column` - column name (unquoted).
/// * `values` - slice of string values to insert.
///
/// Example: `build_batch_insert("ref_biome_token_tags", "token", &["FOO","BAR"])` returns the SQL to insert.
pub(crate) fn build_batch_insert(table: &str, column: &str, values: &[&str]) -> String {
    let mut batch = String::new();
    for v in values {
        let esc = quoted_sql(v);
        write!(&mut batch, "INSERT INTO {table} ({column}) VALUES ({esc});",)
            .expect("Failed to write to string");
    }
    batch
}
