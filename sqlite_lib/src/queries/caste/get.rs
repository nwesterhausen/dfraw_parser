//! GET helper methods for the `[Caste]` struct.

use turso::{Connection, params};

use crate::{client::DbClient, queries::caste::GET_CASTE_NAME_ID_BY_CASTE_ID_AND_POSITION};

impl DbClient {}

/// Get the `caste_name` record for a specific caste id and `tag_position`
///
/// # Errors
///
/// Passes along database errors
pub async fn get_caste_name_id_by_caste_id_and_tag_position(
    conn: &Connection,
    caste_id: i64,
    tag_position: i64,
) -> Result<Option<i64>, Box<dyn std::error::Error>> {
    let mut id_rows = conn
        .query(
            GET_CASTE_NAME_ID_BY_CASTE_ID_AND_POSITION,
            params![caste_id, tag_position],
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
