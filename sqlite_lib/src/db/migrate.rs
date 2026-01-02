//! Provides functions for initializing and managing the database.

use super::migrations::{DOWN_MIGRATIONS, LATEST_SCHEMA_VERSION, UP_MIGRATIONS};
use super::util::get_current_schema_version;
use rusqlite::{Connection, Error};
use tracing::{error, info, warn};

pub(super) fn apply_migrations(conn: &Connection) -> Result<(), Error> {
    migrate_up(conn, LATEST_SCHEMA_VERSION)
}

pub(super) fn migrate_up(conn: &Connection, final_schema: i32) -> Result<(), Error> {
    let starting_version: i32 = get_current_schema_version(conn)?;
    info!("Migrating database schema v{starting_version:02} >> v{final_schema:02}");
    for (schema_version, up_sql) in UP_MIGRATIONS {
        if starting_version < schema_version {
            info!("Applying databse schema v{schema_version:02}");
            let current_version = get_current_schema_version(conn)?;
            if let Err(e) = conn.execute_batch(up_sql) {
                error!(
                    "Failed upgrade from {current_version:02} >> {schema_version:02}. Attempting to undo any partial changes."
                );

                if let Err(rollback_err) = migrate_down(conn, current_version) {
                    error!(
                        "CRITICAL: Failed to rollback schema to {current_version:02} after upgrade failure: {rollback_err}"
                    );
                }

                return Err(e);
            }
            conn.pragma_update(None, "user_version", schema_version)?;
        }
    }

    Ok(())
}

pub(super) fn migrate_down(conn: &Connection, final_schema: i32) -> Result<(), Error> {
    let starting_version: i32 = get_current_schema_version(conn)?;
    if starting_version < final_schema {
        return Ok(());
    }
    let mut current_version = starting_version;
    for (previous_schema_version, down_sql) in DOWN_MIGRATIONS.iter().rev() {
        if current_version > *previous_schema_version {
            warn!("Performing database downgrade {current_version} >> {previous_schema_version}");
            conn.execute_batch(down_sql)?;
            conn.pragma_update(None, "user_version", previous_schema_version)?;
            current_version = *previous_schema_version;
        }
        if current_version <= final_schema {
            return Ok(());
        }
    }

    Ok(())
}
