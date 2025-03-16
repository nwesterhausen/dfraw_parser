use include_dir::{include_dir, Dir};
use rusqlite::{Connection, Result};

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/db/migrations");

/// Load a SQLite database from a file.
///
/// # Arguments
///
/// * `filename` - The path to the SQLite database file.
///
/// # Returns
///
/// A `rusqlite::Connection` to the database.
pub fn load_database(filename: &str) -> Result<Connection> {
    let conn = Connection::open(filename)?;
    Ok(conn)
}

/// Apply a SQL file to a SQLite database.
///
/// # Arguments
///
/// * `conn` - The `rusqlite::Connection` to the database.
/// * `sql` - The SQL to apply.
///
/// # Returns
///
/// An empty `Result` if successful.
///
/// # Errors
///
/// If the SQL fails to execute.
pub(super) fn apply_sql_file(
    conn: &Connection,
    sql: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute_batch(sql)?;
    Ok(())
}

/// Get the user version of a SQLite database, via the `PRAGMA user_version;` statement.
/// We use this to track the current version of the database schema.
///
/// # Arguments
///
/// * `conn` - The `rusqlite::Connection` to the database.
///
/// # Returns
///
/// The user version of the database.
///
/// # Errors
///
/// If the SQL fails to execute.
pub fn get_user_version(conn: &Connection) -> Result<i32, rusqlite::Error> {
    let mut stmt = conn.prepare("PRAGMA user_version;")?;
    let version: i32 = stmt.query_row([], |row| row.get(0))?;
    Ok(version)
}

/// Apply all migrations in the `db/migrations` directory to our SQLite database.
///
/// This function will read all files in the `db/migrations` directory, sort them by version number,
/// and apply each migration in turn if it is newer than the current database version. We include
/// the `db/migrations` directory with the `include_dir` crate, which allows us to embed the migrations
/// in the library binary.
///
/// # Returns
///
/// An empty `Result` if successful.
///
/// # Errors
///
/// If any migration fails to apply, or if the user version fails to update.
pub(super) fn apply_migrations() -> Result<(), Box<dyn std::error::Error>> {
    let conn = super::get_db_conn();
    let current_version = get_user_version(&conn)?;

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

    tracing::info!(
        "Applying {} database migration{}",
        migrations.len(),
        if migrations.len() == 1 { "" } else { "s" }
    );

    for (version, description, sql) in migrations {
        if version > current_version {
            tracing::info!("Applying migration v{} {}", version, description);
            if let Err(e) = apply_sql_file(&conn, sql) {
                tracing::error!(
                    "Failed to apply migration v{} {}: {}",
                    version,
                    description,
                    e
                );
                return Err(e);
            }
            if let Err(e) = conn.execute(&format!("PRAGMA user_version = {};", version), []) {
                tracing::error!("Failed to update user_version to v{}: {}", version, e);
                return Err(Box::new(e));
            }
        } else {
            tracing::debug!(
                "Skipping migration v{} {} as it is not newer than current version {}",
                version,
                description,
                current_version
            );
        }
    }

    Ok(())
}
