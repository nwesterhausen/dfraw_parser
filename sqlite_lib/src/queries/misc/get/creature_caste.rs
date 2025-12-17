use turso::{Connection, params};

use super::super::{GET_CREATURE_CASTE_TAG_ID, INSERT_CREATURE_CASTE_TAG_REFERENCE};

/// Get the id of a specific `dynamic_creature_caste_tag`, inserting it if it doesn't exist
///
/// # Errors
///
/// Passes database errors up
///
/// # Returns
///
/// The index of the `dynamic_creature_caste_tag`
pub async fn get_or_insert_dynamic_creature_caste_tag(
    conn: &Connection,
    creature_identifier: &str,
    caste_identifier: &str,
) -> Result<i64, Box<dyn std::error::Error>> {
    // try to get the existing color
    if let Some(id) =
        get_dynamic_creature_caste_tag(conn, creature_identifier, caste_identifier).await?
    {
        return Ok(id);
    }
    // insert it since it doesn't exist
    conn.execute(
        INSERT_CREATURE_CASTE_TAG_REFERENCE,
        params![creature_identifier, caste_identifier],
    )
    .await?;
    // try to get the existing color or return an error
    (get_dynamic_creature_caste_tag(conn, creature_identifier, caste_identifier).await?)
        .map_or_else(
            || Err("Failed to insert material and get it back".into()),
            Ok,
        )
}

/// Get an id for a `dynamic_creature_caste_tag` by values
///
/// # Errors
///
/// Will pass any database errors along
///
/// # Returns
///
/// None if there is no `dynamic_creature_caste_tag` matching provided values.
pub async fn get_dynamic_creature_caste_tag(
    conn: &Connection,
    creature_identifier: &str,
    caste_identifier: &str,
) -> Result<Option<i64>, Box<dyn std::error::Error>> {
    // see if dynamic_creature_caste_tag exists
    let mut id_rows = conn
        .query(
            GET_CREATURE_CASTE_TAG_ID,
            params![creature_identifier, caste_identifier],
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
