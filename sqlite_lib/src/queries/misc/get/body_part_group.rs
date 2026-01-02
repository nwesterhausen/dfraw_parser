use turso::{Connection, params};

use crate::queries::misc::{GET_BODY_PART_GROUP_ID, INSERT_BODY_PART_GROUP};

/// Get the id of a specific `dynamic_body_part_group`, inserting it if it doesn't exist
///
/// # Errors
///
/// Passes database errors up
///
/// # Returns
///
/// The index of the `body_part_group`
pub async fn get_or_insert_body_part_group(
    conn: &Connection,
    body_part_selector: &str,
    body_part: &str,
) -> Result<i64, Box<dyn std::error::Error>> {
    if let Some(id) = get_body_part_group(conn, body_part_selector, body_part).await? {
        return Ok(id);
    }
    // insert it since it doesn't exist
    conn.execute(
        INSERT_BODY_PART_GROUP,
        params![body_part_selector, body_part],
    )
    .await?;
    // try to get the existing color or return an error
    (get_body_part_group(conn, body_part_selector, body_part).await?).map_or_else(
        || Err("Failed to insert material and get it back".into()),
        Ok,
    )
}

/// Get an id for a `dynamic_body_part_group` by values
///
/// # Errors
///
/// Will pass any database errors along
///
/// # Returns
///
/// None if there is no `dynamic_body_part_group` matching provided values.
pub async fn get_body_part_group(
    conn: &Connection,
    body_part_selector: &str,
    body_part: &str,
) -> Result<Option<i64>, Box<dyn std::error::Error>> {
    // see if dynamic_body_part_group exists
    let mut id_rows = conn
        .query(
            GET_BODY_PART_GROUP_ID,
            params![body_part_selector, body_part],
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
