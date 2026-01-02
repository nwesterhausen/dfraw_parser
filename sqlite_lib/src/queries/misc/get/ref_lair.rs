use turso::{Connection, params};

use super::super::GET_REF_LAIR_TAG_ID;

/// Get an id for a `ref_lair_token_flags` by values
///
/// # Errors
///
/// Will pass any database errors along
///
/// # Returns
///
/// None if there is no `ref_lair_token_flags` matching provided values.
pub async fn get_ref_lair_token_id(
    conn: &Connection,
    token: &str,
) -> Result<Option<i64>, Box<dyn std::error::Error>> {
    // see if ref_lair_token_flags exists
    let mut id_rows = conn.query(GET_REF_LAIR_TAG_ID, params![token]).await?;

    match id_rows.next().await? {
        Some(row) => {
            let id: i64 = row.get(0)?;
            Ok(Some(id))
        }
        None => Ok(None), // Return None if no row exists
    }
}
