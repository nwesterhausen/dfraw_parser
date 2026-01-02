use turso::{Connection, params};

use super::super::GET_REF_OBJECT_TYPE;

/// Get an id for a `ref_object_types` by values
///
/// # Errors
///
/// Will pass any database errors along
///
/// # Returns
///
/// None if there is no `ref_object_types` matching provided values.
pub async fn get_ref_object_type_id(
    conn: &Connection,
    token: &str,
) -> Result<Option<i64>, Box<dyn std::error::Error>> {
    // see if ref_object_types exists
    let mut id_rows = conn.query(GET_REF_OBJECT_TYPE, params![token]).await?;

    match id_rows.next().await? {
        Some(row) => {
            let id: i64 = row.get(0)?;
            Ok(Some(id))
        }
        None => Ok(None), // Return None if no row exists
    }
}
