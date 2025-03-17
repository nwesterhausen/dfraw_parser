use crate::database::sqlite;
use crate::database::sqlite::{apply_sql, get_user_version};
use include_dir::{include_dir, Dir, DirEntry};

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

    let mut migrations: Vec<(i32, &str, &str, &str)> = Vec::new();

    // Sort files to ensure consistent order
    let mut dirs: Vec<(&str, Vec<&DirEntry>)> = MIGRATIONS_DIR
        .dirs()
        .map(|dir| {
            let mut entries: Vec<&DirEntry> = dir.entries().iter().collect();
            entries.sort_by_key(|f| f.path().to_owned());
            (dir.path().file_stem().unwrap().to_str().unwrap(), entries)
        })
        .collect::<Vec<(&str, Vec<&DirEntry>)>>();

    dirs.sort_by_key(|(dir_name, _)| *dir_name);

    for (dir_name, entries) in dirs {
        // Parse version and description from directory name
        let parts: Vec<&str> = dir_name.splitn(2, '_').collect();
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
                let mut up_sql: Option<&str> = None;
                let mut down_sql: Option<&str> = None;

                for entry in entries {
                    if let Some(file) = entry.as_file() {
                        if entry.path().file_name().unwrap().to_str().unwrap() == "up.sql" {
                            up_sql = file.contents_utf8();
                        } else if entry.path().file_name().unwrap().to_str().unwrap() == "down.sql"
                        {
                            down_sql = file.contents_utf8();
                        }
                    }
                }

                if up_sql.is_none() {
                    tracing::warn!("Missing up.sql file in migration directory: {}", dir_name);
                    continue;
                }

                if down_sql.is_none() {
                    tracing::warn!("Missing down.sql file in migration directory: {}", dir_name);
                    continue;
                }

                migrations.push((version, description, up_sql.unwrap(), down_sql.unwrap()));
                tracing::debug!("Found applicable migration: {:03} {}", version, description);
            } else {
                tracing::warn!(
                    "Failed to parse version from directory name: {:?}",
                    dir_name
                );
            }
        } else {
            tracing::warn!(
                "Directory name does not contain an underscore: {:?}",
                dir_name
            );
        }
    }

    // Sort by version number
    migrations.sort_by_key(|&(version, _, _, _)| version);

    tracing::info!(
        "Found {} applicable database migration{}",
        migrations.len(),
        if migrations.len() == 1 { "" } else { "s" }
    );

    for (version, description, up_sql, down_sql) in migrations {
        tracing::info!("Applying migration {:03} {}", version, description);
        if let Err(e) = apply_sql(up_sql) {
            tracing::error!(
                "Failed to apply migration {:03} {}: {}",
                version,
                description,
                e
            );

            try_rollback(version, description, down_sql)?;
            return Err(Box::new(e));
        }

        if let Err(e) = sqlite::set_user_version(version) {
            tracing::error!("Failed to update user_version to '{}': {}", version, e);

            if let Err(e) = sqlite::set_user_version(version) {
                tracing::error!("Failed to update user_version to '{}': {}", version, e);

                try_rollback(version, description, down_sql)?;
                return Err(Box::new(e));
            }
            return Err(Box::new(e));
        }
    }

    Ok(())
}

fn try_rollback(
    version: i32,
    description: &str,
    down_sql: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!(
        "Attempting to roll back migration {:03} {}",
        version,
        description
    );

    if let Err(rollback_err) = apply_sql(down_sql) {
        tracing::error!(
            "Failed to roll back migration {:03} {}: {}",
            version,
            description,
            rollback_err
        );
        return Err(Box::new(rollback_err));
    }

    tracing::info!(
        "Successfully rolled back migration {:03} {}",
        version,
        description
    );
    Ok(())
}
