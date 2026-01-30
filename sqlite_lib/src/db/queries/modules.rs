use dfraw_parser::{ModuleInfo, metadata::RawModuleLocation, traits::RawObject};
use rusqlite::{Connection, Result, Transaction, params};
use tracing::info;
use uuid::Uuid;

use super::super::rusqlite_extensions::OptionalResultExtension;

/// Returns true if the module exists in the database.
///
/// Searches based on key identifiers: identifier, version and location
///
/// # Errors
///
/// - database error
pub fn exists_module_by_identifiers(
    conn: &Connection,
    identifier: &str,
    numeric_version: i64,
    location: RawModuleLocation,
) -> Result<bool> {
    match try_get_module_id_by_identifiers(conn, identifier, numeric_version, location) {
        Ok(res) => Ok(res.is_some()),
        Err(e) => Err(e),
    }
}

/// Attempts to find the database ID for a specific module.
///
/// Searches to find an existing module in the database based on key identifying
/// factors: identifier, version and location.
///
/// # Errors
///
/// - database error
pub fn try_get_module_id_by_identifiers(
    conn: &Connection,
    identifier: &str,
    numeric_version: i64,
    location: RawModuleLocation,
) -> Result<Option<i64>> {
    conn.query_row(
        "SELECT id FROM modules
         WHERE identifier = ?1
           AND version = ?2
           AND module_location_id = ?3
         LIMIT 1",
        params![identifier, numeric_version, location as i32],
        |row| row.get(0),
    )
    .optional()
}

/// Try get module by its `object_id`
///
/// # Errors
///
/// - database errors
pub fn try_get_module_id_by_object_id(conn: &Connection, object_id: Uuid) -> Result<Option<i64>> {
    conn.query_row(
        "SELECT id FROM modules WHERE object_id = ?1 LIMIT 1",
        params![object_id.as_bytes()],
        |row| row.get(0),
    )
    .optional()
}

/// Check if a module exists by its `object_id`
///
/// # Errors
///
/// - database error
pub fn exists_module_by_object_id(conn: &Connection, object_id: Uuid) -> Result<bool> {
    match try_get_module_id_by_object_id(conn, object_id) {
        Ok(res) => Ok(res.is_some()),
        Err(e) => Err(e),
    }
}

/// A helper: insert a module record into the `modules` table, returning the ID
///
/// This only inserts the module and the data for the `modules` table, nothing complex.
/// Other data in the module will need to be inserted using the other insert methods.
///
/// # Error
///
/// - on database error
pub fn insert_module_record(conn: &Connection, info: &ModuleInfo) -> Result<i64> {
    conn.execute(
        "INSERT INTO modules (
            name, identifier, version, display_version,
            earliest_compatible_version, earliest_compatible_display_version,
            author, description, module_directory_path, module_location_id,
            steam_file_id, steam_title, steam_description, steam_changelog,
            object_id
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
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
            i64::from(u32::from(info.get_location())),
            info.get_steam_data()
                .as_ref()
                .map(|s| s.get_file_id().cast_signed()),
            info.get_steam_data()
                .as_ref()
                .map(dfraw_parser::SteamData::get_title),
            info.get_steam_data()
                .as_ref()
                .map(dfraw_parser::SteamData::get_description),
            info.get_steam_data()
                .as_ref()
                .map(dfraw_parser::SteamData::get_changelog),
            info.get_object_id().as_bytes()
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Try to get the db id for a module (uses its `object_id`)
///
/// # Errors
///
/// - database errors
pub fn try_get_module_id(conn: &Connection, module: &ModuleInfo) -> Result<Option<i64>> {
    try_get_module_id_by_object_id(conn, module.get_object_id())
}

/// Try to get the db id for a module (uses its `object_id`)
///
/// # Errors
///
/// - database errors
pub fn exists_module(conn: &Connection, module: &ModuleInfo) -> Result<bool> {
    exists_module_by_object_id(conn, module.get_object_id())
}

/// Insert a raw module into the database, including its metadata and all raws that belong to it.
///
/// # Errors
///
/// - database errors
pub fn insert_module_and_data(
    conn: &mut Connection,
    overwrite_raws: bool,
    module: &ModuleInfo,
    data: &[&dyn RawObject],
) -> Result<()> {
    let module_db_id = insert_module(conn, overwrite_raws, module)?;

    super::process_raw_insertions(conn, module_db_id, module, data, overwrite_raws)
}

/// Insert a module with its supporting data, returning its id in the database.
///
/// This inserts the module along with its dependency chain and steam tag data.
///
/// # Errors
///
/// - database errors
pub fn insert_module(
    conn: &mut Connection,
    overwrite_raws: bool,
    module: &ModuleInfo,
) -> Result<i64> {
    let existing_module_id = try_get_module_id(conn, module)?;
    let tx = conn.transaction()?;
    let module_db_id = if let Some(id) = existing_module_id {
        if overwrite_raws {
            info!(
                "Module {} ({}v{} in {}) already exists in database. Skipping.",
                module.get_name(),
                module.get_identifier(),
                module.get_numeric_version(),
                module.get_location()
            );
            return Ok(id);
        }
        id
    } else {
        insert_module_record(&tx, module)?
    };

    // 2. Process Dependencies (only if module is new)
    if existing_module_id.is_none() {
        insert_module_dependencies(&tx, module_db_id, module)?;
    }

    tx.commit()?;
    Ok(module_db_id)
}

/// Insert a module dependency into the `module_dependencies` table
///
/// # Error
///
/// - on database error
pub fn insert_module_dependencies(
    tx: &Transaction,
    module_db_id: i64,
    info: &ModuleInfo,
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

/// Clear the side tables (search indices and lookup tables) for a given raw id.
///
/// The other relations are not touched (and rely on the cascade delete); e.g. tiles or sprites.
///
/// # Errors
///
/// - database errors
pub fn clear_side_tables_for_module_id(conn: &Connection, id: i64) -> Result<()> {
    const DELETE_COMMON_FLAGS_FOR_ID: &str = "DELETE FROM common_raw_flags WHERE raw_id IN (SELECT id FROM raw_definitions WHERE module_id = ?1)";
    const DELETE_COMMON_NUMERIC_FLAGS_FOR_ID: &str = "DELETE FROM common_raw_flags_with_numeric_value WHERE raw_id IN (SELECT id FROM raw_definitions WHERE module_id = ?1)";
    const DELETE_SEARCH_IDX_FOR_ID: &str = "DELETE FROM raw_search_index WHERE raw_id IN (SELECT id FROM raw_definitions WHERE module_id = ?1)";
    const DELETE_NAME_SEARCH_IDX_FOR_ID: &str = "DELETE FROM raw_names WHERE raw_id IN (SELECT id FROM raw_definitions WHERE module_id = ?1)";

    // Clear side tables (CASCADE handles tile_pages, sprite_graphics, raw_names, flags; in this
    // case we explicitly remove the raw_names and flags in order to allow them to be updated if
    // needed)
    conn.execute(DELETE_COMMON_FLAGS_FOR_ID, params![id])?;
    conn.execute(DELETE_COMMON_NUMERIC_FLAGS_FOR_ID, params![id])?;
    conn.execute(DELETE_SEARCH_IDX_FOR_ID, params![id])?;
    conn.execute(DELETE_NAME_SEARCH_IDX_FOR_ID, params![id])?;

    Ok(())
}
