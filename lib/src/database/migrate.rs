use crate::database::sqlite;
use crate::database::sqlite::{apply_sql, get_user_version};
use include_dir::{include_dir, Dir};

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/db/migrations");

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
    let current_version = get_user_version()?;

    tracing::info!("Current database version is {:03}", current_version);

    let mut migrations: Vec<(i32, &str, &str)> = Vec::new();

    // Sort files to ensure consistent order
    let mut files = MIGRATIONS_DIR.files().collect::<Vec<_>>();
    files.sort_by_key(|f| f.path().to_owned());

    for file in files {
        // Skip non-SQL files
        if !file.path().to_string_lossy().ends_with(".sql") {
            tracing::debug!("Ignoring non-SQL file: {}", file.path().display());
            continue;
        }

        // Parse version and description
        if let Some(file_stem) = file.path().file_stem().and_then(|s| s.to_str()) {
            let parts: Vec<&str> = file_stem.splitn(2, '_').collect();
            if parts.len() == 2 {
                if let Ok(version) = parts[0].parse::<i32>() {
                    // Skip migrations that are already applied
                    if version <= current_version {
                        tracing::debug!(
                            "Skipping migration {:03} {} (current version: {})",
                            version,
                            parts[1],
                            current_version
                        );
                        continue;
                    }

                    let description = parts[1];
                    migrations.push((version, description, file.contents_utf8().unwrap()));
                    tracing::debug!("Found applicable migration: {:03} {}", version, description);
                } else {
                    tracing::warn!("Failed to parse version from file name: {:?}", file_stem);
                }
            } else {
                tracing::warn!("File name does not contain an underscore: {:?}", file_stem);
            }
        }
    }

    // Sort by version number
    migrations.sort_by_key(|&(version, _, _)| version);

    tracing::info!(
        "Found {} applicable database migration{}",
        migrations.len(),
        if migrations.len() == 1 { "" } else { "s" }
    );

    for (version, description, sql) in migrations {
        tracing::info!("Applying migration {:03} {}", version, description);
        if let Err(e) = apply_sql(sql) {
            tracing::error!(
                "Failed to apply migration {:03} {}: {}",
                version,
                description,
                e
            );
            return Err(Box::new(e));
        }
        if let Err(e) = sqlite::set_user_version(version) {
            tracing::error!("Failed to update user_version to '{}': {}", version, e);
            return Err(Box::new(e));
        }
    }

    Ok(())
}
