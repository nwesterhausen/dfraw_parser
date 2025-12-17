use turso::{Connection, Rows, params};

use crate::queries::misc::{
    GET_NAME_ID_BY_SINGULAR_PLURAL, GET_NAME_ID_BY_SINGULAR_PLURAL_ADJECTIVE, INSERT_DYNAMIC_NAME,
    INSERT_DYNAMIC_NAME_WITH_ADJECTIVE,
};

/// Get the id of a specific `dynamic_name`, inserting it if it doesn't exist
///
/// # Errors
///
/// Passes database errors up
///
/// # Returns
///
/// The index of the `dynamic_name`
pub async fn get_or_insert_dynamic_name(
    conn: &Connection,
    name_singular: &str,
    name_plural: &str,
    name_adjective: Option<&str>,
) -> Result<i64, Box<dyn std::error::Error>> {
    // try to get the existing name
    if let Some(id) = get_dynamic_name(conn, name_singular, name_plural, name_adjective).await? {
        return Ok(id);
    }
    // insert it since it doesn't exist

    if let Some(adjective_name) = name_adjective {
        conn.execute(
            INSERT_DYNAMIC_NAME_WITH_ADJECTIVE,
            params![name_singular, name_plural, adjective_name],
        )
        .await?;
    } else {
        conn.execute(INSERT_DYNAMIC_NAME, params![name_singular, name_plural])
            .await?;
    }

    // try to get the existing color or return an error
    (get_dynamic_name(conn, name_singular, name_plural, name_adjective).await?)
        .map_or_else(|| Err("Failed to insert name and get it back".into()), Ok)
}

/// Get an id for a `dynamic_name` by values
///
/// # Errors
///
/// Will pass any database errors along
///
/// # Returns
///
/// None if there is no `dynamic_name` matching provided values.
pub async fn get_dynamic_name(
    conn: &Connection,
    name_singular: &str,
    name_plural: &str,
    name_adjective: Option<&str>,
) -> Result<Option<i64>, Box<dyn std::error::Error>> {
    // see if dynamic_name exists
    let mut id_rows = if let Some(adjective_name) = name_adjective {
        conn.query(
            GET_NAME_ID_BY_SINGULAR_PLURAL_ADJECTIVE,
            params![name_singular, name_plural, adjective_name],
        )
        .await?
    } else {
        conn.query(
            GET_NAME_ID_BY_SINGULAR_PLURAL,
            params![name_singular, name_plural],
        )
        .await?
    };

    match id_rows.next().await? {
        Some(row) => {
            let id: i64 = row.get(0)?;
            Ok(Some(id))
        }
        None => Ok(None), // Return None if no row exists
    }
}
