use turso::{Connection, params};

use super::super::{GET_DYNAMIC_ITEM_OF_MATERIAL_ID, INSERT_DYNAMIC_ITEM_OF_MATERIAL};

/// Get the id of a specific `dynamic_item_of_material`, inserting it if it doesn't exist
///
/// # Errors
///
/// Passes database errors up
///
/// # Returns
///
/// The index of the `dynamic_item_of_material`
pub async fn get_or_insert_dynamic_item_of_material(
    conn: &Connection,
    item_identifier: &str,
    material_identifier: &str,
) -> Result<i64, Box<dyn std::error::Error>> {
    // try to get the existing color
    if let Some(id) =
        get_dynamic_item_of_material(conn, item_identifier, material_identifier).await?
    {
        return Ok(id);
    }
    // insert it since it doesn't exist
    conn.execute(
        INSERT_DYNAMIC_ITEM_OF_MATERIAL,
        params![item_identifier, material_identifier],
    )
    .await?;
    // try to get the existing color or return an error
    (get_dynamic_item_of_material(conn, item_identifier, material_identifier).await?)
        .map_or_else(|| Err("Failed to insert item and get it back".into()), Ok)
}

/// Get an id for a `dynamic_item_of_material` by values
///
/// # Errors
///
/// Will pass any database errors along
///
/// # Returns
///
/// None if there is no `dynamic_item_of_material` matching provided values.
pub async fn get_dynamic_item_of_material(
    conn: &Connection,
    item_identifier: &str,
    material_identifier: &str,
) -> Result<Option<i64>, Box<dyn std::error::Error>> {
    // see if dynamic_item_of_material exists
    let mut id_rows = conn
        .query(
            GET_DYNAMIC_ITEM_OF_MATERIAL_ID,
            params![item_identifier, material_identifier],
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
