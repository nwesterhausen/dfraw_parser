//! selectsert helper functions for color

use turso::{Connection, params};

/// Get the id of a specific color, inserting it if it doesn't exist
///
/// # Errors
///
/// Passes database errors up
///
/// # Returns
///
/// The index of the color
pub async fn get_or_insert_color(
    conn: &Connection,
    foreground: i64,
    background: i64,
    brightness: i64,
) -> Result<i64, Box<dyn std::error::Error>> {
    // try to get the existing color
    if let Some(id) = get_color(conn, foreground, background, brightness).await? {
        return Ok(id);
    }
    // insert it since it doesn't exist
    conn.execute(
        super::INSERT_COLOR,
        params![foreground, background, brightness],
    )
    .await?;
    // try to get the existing color or return an error
    (get_color(conn, foreground, background, brightness).await?)
        .map_or_else(|| Err("Failed to insert color and get it back".into()), Ok)
}

/// Get an id for a color by values
///
/// # Errors
///
/// Will pass any database errors along
///
/// # Returns
///
/// None if there is no color matching provided values.
pub async fn get_color(
    conn: &Connection,
    foreground: i64,
    background: i64,
    brightness: i64,
) -> Result<Option<i64>, Box<dyn std::error::Error>> {
    // see if color exists
    let mut id_rows = conn
        .query(
            super::GET_COLOR_ID_BY_VALUES,
            params![foreground, background, brightness],
        )
        .await?;

    match id_rows.next().await? {
        Some(row) => {
            let id: i64 = row.get(0)?;
            Ok(Some(id))
        }
        None => Ok(None), // Return None if no row exists
    }
}
