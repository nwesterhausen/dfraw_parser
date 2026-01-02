use rusqlite::{Connection, Error};

pub(super) fn get_current_schema_version(conn: &Connection) -> Result<i32, Error> {
    let user_version: i32 = conn.pragma_query_value(None, "user_version", |row| row.get(0))?;
    Ok(user_version)
}
