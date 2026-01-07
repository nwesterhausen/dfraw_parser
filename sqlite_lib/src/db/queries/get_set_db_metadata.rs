use rusqlite::{Connection, Result, params};

use crate::db::metadata_markers::TypedMetadata;
use crate::db::rusqlite_extensions::OptionalResultExtension;

/// Sets a metadata value using a typed key marker.
/// This automatically serializes the value to a JSON string for storage.
///
/// # Errors
///
/// - database insertion error
pub fn set_typed_metadata<T>(conn: &Connection, value: &T::Value) -> Result<()>
where
    T: TypedMetadata,
{
    let key = T::key().as_str();
    // We use JSON serialization to ensure complex types or strings with spaces
    // are stored correctly in the TEXT column.
    let val_str = serde_json::to_string(value).map_err(|_| rusqlite::Error::InvalidQuery)?;

    conn.execute(
        "INSERT OR REPLACE INTO app_metadata (key, value) VALUES (?1, ?2)",
        params![key, val_str],
    )?;
    Ok(())
}

/// Gets a metadata value and automatically deserializes it into the correct Rust type.
///
/// # Errors
///
/// - database read error
/// - deserialization of 'value' error
pub fn get_typed_metadata<T>(conn: &Connection) -> Result<Option<T::Value>>
where
    T: TypedMetadata,
{
    let key = T::key().as_str();
    let raw_val: Option<String> = conn
        .query_row(
            "SELECT value FROM app_metadata WHERE key = ?1",
            params![key],
            |row| row.get(0),
        )
        .optional()?;

    raw_val.map_or_else(
        || Ok(None),
        |s| {
            serde_json::from_str(&s)
                .map(Some)
                .map_err(|_| rusqlite::Error::InvalidQuery)
        },
    )
}
