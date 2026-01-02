use turso::{Connection, params};

use super::super::{GET_DYNAMIC_MATERIAL_IN_STATE_ID, INSERT_DYNAMIC_MATERIAL_IN_STATE};

/// Get the id of a specific `dynamic_material_in_state`, inserting it if it doesn't exist
///
/// # Errors
///
/// Passes database errors up
///
/// # Returns
///
/// The index of the `dynamic_material_in_state`
pub async fn get_or_insert_dynamic_material_in_state(
    conn: &Connection,
    material_identifier: &str,
    state: &str,
) -> Result<i64, Box<dyn std::error::Error>> {
    // try to get the existing color
    if let Some(id) = get_dynamic_material_in_state(conn, material_identifier, state).await? {
        return Ok(id);
    }
    // insert it since it doesn't exist
    conn.execute(
        INSERT_DYNAMIC_MATERIAL_IN_STATE,
        params![material_identifier, state],
    )
    .await?;
    // try to get the existing color or return an error
    (get_dynamic_material_in_state(conn, material_identifier, state).await?).map_or_else(
        || Err("Failed to insert material and get it back".into()),
        Ok,
    )
}

/// Get an id for a `dynamic_material_in_state` by values
///
/// # Errors
///
/// Will pass any database errors along
///
/// # Returns
///
/// None if there is no `dynamic_material_in_state` matching provided values.
pub async fn get_dynamic_material_in_state(
    conn: &Connection,
    material_identifier: &str,
    state: &str,
) -> Result<Option<i64>, Box<dyn std::error::Error>> {
    // see if dynamic_material_in_state exists
    let mut id_rows = conn
        .query(
            GET_DYNAMIC_MATERIAL_IN_STATE_ID,
            params![material_identifier, state],
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
