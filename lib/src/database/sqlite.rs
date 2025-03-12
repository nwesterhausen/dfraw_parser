use rusqlite::{Connection, Result};
use std::fs;
use std::path::Path;
use include_dir::{include_dir, Dir};

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/db/migrations");

pub fn load_database(filename: &str) -> Result<Connection> {
    let conn = Connection::open(filename)?;
    Ok(conn)
}

pub(super) fn apply_sql_file(conn: &Connection, sql: &str) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute_batch(sql)?;
    Ok(())
}

pub fn get_user_version(conn: &Connection) -> Result<i32, rusqlite::Error> {
    let mut stmt = conn.prepare("PRAGMA user_version;")?;
    let version: i32 = stmt.query_row([], |row| row.get(0))?;
    Ok(version)
}

pub(super) fn apply_migrations(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let current_version = get_user_version(conn)?;

    let mut migrations: Vec<(i32, &str, &str)> = Vec::new();
    for file in MIGRATIONS_DIR.files() {
        if let Some(extension) = file.path().extension().and_then(|s| s.to_str()) {
            if extension != "sql" {
                tracing::debug!("Ignoring non-SQL file: {}", file.path().display());
                continue;
            }
        } else {
            tracing::debug!("Ignoring file with no extension: {}", file.path().display());
            continue;
        }

        if let Some(file_stem) = file.path().file_stem().and_then(|s| s.to_str()) {
            let parts: Vec<&str> = file_stem.splitn(2, '_').collect();
            if parts.len() == 2 {
                if let Ok(version) = parts[0].parse::<i32>() {
                    let description = parts[1];
                    migrations.push((version, description, file.contents_utf8().unwrap()));
                    tracing::debug!("Detected valid migration file: {}", file.path().display());
                } else {
                    tracing::warn!("Failed to parse version from file name: {:?}", file_stem);
                }
            } else {
                tracing::warn!("File name does not contain an underscore: {:?}", file_stem);
            }
        }
    }

    migrations.sort_by_key(|&(version, _, _)| version);

    tracing::info!("Applying {} database migration{}", migrations.len(), if migrations.len() == 1 { "" } else { "s" });

    for (version, description, sql) in migrations {
        if version > current_version {
            tracing::info!("Applying migration v{} {}", version, description);
            if let Err(e) = apply_sql_file(conn, sql) {
                tracing::error!("Failed to apply migration v{} {}: {}", version, description, e);
                return Err(e);
            }
            if let Err(e) = conn.execute(&format!("PRAGMA user_version = {};", version), []) {
                tracing::error!("Failed to update user_version to v{}: {}", version, e);
                return Err(Box::new(e));
            }
        } else {
            tracing::debug!("Skipping migration v{} {} as it is not newer than current version {}", version, description, current_version);
        }
    }

    Ok(())
}