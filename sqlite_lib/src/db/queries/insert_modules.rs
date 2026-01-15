use dfraw_parser::{InfoFile, traits::RawObject};
use rusqlite::{Connection, Result, Transaction, params};
use tracing::{debug, info};

use crate::{ClientOptions, db::queries::process_raw_insertions};

use super::super::rusqlite_extensions::OptionalResultExtension;

/// Insert a raw module into the database, including its metadata and all raws that belong to it.
pub fn insert_module_data(
    conn: &mut Connection,
    options: &ClientOptions,
    info: &InfoFile,
    raws: &[Box<dyn RawObject>],
) -> Result<()> {
    let overwrite_raws = options.overwrite_raws;

    let tx = conn.transaction()?;

    // 1. Check for existing module considering identifier, version, and location_id
    let existing_module_id: Option<i64> = tx.query_row(
        "SELECT id FROM modules WHERE identifier = ?1 AND version = ?2 AND module_location_id = ?3 LIMIT 1",
        params![
            info.get_identifier(),
            i64::from(info.get_numeric_version()),
            info.get_location() as i32
        ],
        |row| row.get(0),
    ).optional()?;
    debug!(
        "existing_module_id searched '{}' '{}' '{}' => {existing_module_id:?}",
        info.get_identifier(),
        i64::from(info.get_numeric_version()),
        info.get_location() as i32
    );

    let module_db_id = if let Some(id) = existing_module_id {
        if !overwrite_raws {
            info!(
                "Module {} (v{}, loc {}) already exists. Skipping.",
                info.get_identifier(),
                info.get_numeric_version(),
                info.get_location()
            );
            return Ok(());
        }
        id
    } else {
        insert_module_record(&tx, info)?
    };

    // 2. Process Dependencies (only if module is new)
    if existing_module_id.is_none() {
        insert_module_dependencies(&tx, module_db_id, info)?;
    }

    // 3. Process Raws
    process_raw_insertions(&tx, module_db_id, info, raws, overwrite_raws)?;

    tx.commit()
}

/// Insert a module record into the `modules` table
///
/// # Error
///
/// - on database error
pub fn insert_module_record(tx: &Transaction, info: &InfoFile) -> Result<i64> {
    tx.execute(
        "INSERT INTO modules (
            name, identifier, version, display_version,
            earliest_compatible_version, earliest_compatible_display_version,
            author, description, module_directory_path, module_location_id,
            steam_file_id
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            info.get_name(),
            info.get_identifier(),
            i64::from(info.get_numeric_version()),
            info.get_version(),
            i64::from(info.get_earliest_compatible_numeric_version()),
            info.get_earliest_compatible_displayed_version(),
            info.get_author(),
            info.get_description(),
            info.get_parent_directory(),
            i64::from(i32::from(info.get_location())),
            info.get_steam_data()
                .as_ref()
                .map(|s| s.get_file_id().cast_signed())
        ],
    )?;
    Ok(tx.last_insert_rowid())
}

/// Insert a module dependency into the `module_dependencies` table
///
/// # Error
///
/// - on database error
pub fn insert_module_dependencies(
    tx: &Transaction,
    module_db_id: i64,
    info: &InfoFile,
) -> Result<()> {
    let mut dep_stmt = tx.prepare_cached(
        "INSERT INTO module_dependencies (module_id, target_identifier, restriction_type_id)
     VALUES (?1, ?2, ?3)",
    )?;
    info!(
        "Inserting module dependencies for {}",
        info.get_identifier()
    );

    if let Some(ids) = info.get_requires_ids() {
        info!("Attempting to insert required ids: {ids:?}");
        for id in ids {
            dep_stmt.execute(params![module_db_id, id, 1])?;
        }
    }
    if let Some(ids) = info.get_conflicts_with_ids() {
        info!("Attempting to insert conflicting ids: {ids:?}");
        for id in ids {
            dep_stmt.execute(params![module_db_id, id, 2])?;
        }
    }
    if let Some(ids) = info.get_requires_ids_before() {
        info!("Attempting to insert required_before ids: {ids:?}");
        for id in ids {
            dep_stmt.execute(params![module_db_id, id, 3])?;
        }
    }
    if let Some(ids) = info.get_requires_ids_after() {
        info!("Attempting to insert required_after ids: {ids:?}");
        for id in ids {
            dep_stmt.execute(params![module_db_id, id, 4])?;
        }
    }

    Ok(())
}
