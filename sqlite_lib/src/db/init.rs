//! Provides functions for initializing and managing the database.

use super::{LATEST_SCHEMA_VERSION, migrations};
use rusqlite::{Connection, Error};
use tracing::{debug, info};

/// Opens a connection to the database and initializes it if it doesn't exist.
///
/// This will attempt to migrate forward if possible, otherwise it will fail.
///
/// # Errors
/// - If the database cannot be opened or initialized.
/// - If a migration is not possible or fails.
pub fn init_db(path: &str) -> Result<Connection, Error> {
    let conn = Connection::open(path)?;
    info!("Database connection opened.");
    debug!("Database: {path}");
    let current_schema_version: i32 =
        conn.pragma_query_value(None, "user_version", |row| row.get(0))?;
    info!(
        "Current database schema: v{current_schema_version}, Target database schema: v{LATEST_SCHEMA_VERSION}"
    );

    if current_schema_version < LATEST_SCHEMA_VERSION {
        apply_migrations(&conn, current_schema_version)?;
    }

    Ok(conn)
}

fn apply_migrations(conn: &Connection, from_version: i32) -> Result<(), Error> {
    if from_version < 1 {
        info!("Performing migration 1");
        conn.execute_batch(migrations::SQL_001)?;
        debug!("Updating version to 1");
        conn.pragma_update(None, "user_version", 1)?;
    }

    Ok(())
}
