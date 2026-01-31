use dfraw_parser::{Creature, Graphic, TilePage, tokens::ObjectType, traits::RawObject};
use rusqlite::{Connection, Result, params};
use uuid::Uuid;

use crate::db::queries::get_id_for_module_location;

use super::super::rusqlite_extensions::OptionalResultExtension;
use super::table_inserts::{
    INSERT_COMMON_FLAG, INSERT_LARGE_SPRITE_GRAPHIC, INSERT_LOOKUP_NAME,
    INSERT_RAW_DEFINITION_NO_UPDATE_RETURN_ID, INSERT_SEARCH_INDEX, INSERT_SPRITE_GRAPHIC,
    INSERT_TILE_PAGE,
};

/// Returns true if the raw exists in the database.
///
/// Searches for a match based on the raw identifier and its metadata.
///
/// First tries using the raw's `object_id`, then the identifier and stored `module_object_id`
/// in its metadata, and finally using identifier, location, module name and module version.
///
/// # Errors
///
/// - database error
#[allow(clippy::borrowed_box)]
pub fn exists_raw(conn: &Connection, raw: &Box<dyn RawObject>) -> Result<bool> {
    match try_get_raw_id(conn, raw) {
        Ok(res) => Ok(res.is_some()),
        Err(e) => Err(e),
    }
}

/// Returns true if the raw exists in the database, searching by identifier in a specific module.
///
/// # Errors
///
/// - database error
pub fn exists_raw_in_module_by_object_id(
    conn: &Connection,
    identifier: &str,
    module_object_id: Uuid,
) -> Result<bool> {
    match try_get_raw_id_by_identifier_and_module_object_id(conn, identifier, module_object_id) {
        Ok(res) => Ok(res.is_some()),
        Err(e) => Err(e),
    }
}

/// Attempts to find the database ID for a specific raw searching by the raw `object_id`.
///
/// Returns `Ok(Some(id))` if it exists, or `Ok(None)` if it does not.
/// This is useful for checking existence and obtaining the key for updates
/// in a single operation.
///
/// # Errors
///
/// - database error
pub fn try_get_raw_id_by_object_id(conn: &Connection, object_id: Uuid) -> Result<Option<i64>> {
    conn.query_row(
        "SELECT r.id FROM raw_definitions r
             WHERE r.object_id = ?1
             LIMIT 1",
        params![object_id.as_bytes()],
        |row| row.get(0),
    )
    .optional()
}

/// Attempts to find the database ID for a specific raw, searching by identifier and module `object_id`
///
/// Returns `Ok(Some(id))` if it exists, or `Ok(None)` if it does not.
/// This is useful for checking existence and obtaining the key for updates
/// in a single operation.
///
/// # Errors
///
/// - database error
pub fn try_get_raw_id_by_identifier_and_module_object_id(
    conn: &Connection,
    identifier: &str,
    module_object_id: Uuid,
) -> Result<Option<i64>> {
    const GET_RAW_ID_BY_RAW_IDENTIFIER_AND_MODULE_OBJECT_ID: &str = r"
        SELECT r.id FROM raw_definitions r
             JOIN  modules m ON r.module_id = m.id
            WHERE  r.identifier = ?1
             AND   m.object_id = ?2
            LIMIT  1
    ";

    conn.query_row(
        GET_RAW_ID_BY_RAW_IDENTIFIER_AND_MODULE_OBJECT_ID,
        params![identifier, module_object_id.as_bytes(),],
        |row| row.get(0),
    )
    .optional()
}

/// Attempts to find the database ID for a specific raw, searching by identifier and module `id`
///
/// Returns `Ok(Some(id))` if it exists, or `Ok(None)` if it does not.
/// This is useful for checking existence and obtaining the key for updates
/// in a single operation.
///
/// # Errors
///
/// - database error
pub fn try_get_raw_id_by_identifier_and_module_id(
    conn: &Connection,
    identifier: &str,
    module_id: i64,
) -> Result<Option<i64>> {
    const GET_RAW_ID_BY_RAW_IDENTIFIER_AND_MODULE_ID: &str = r"
    SELECT r.id FROM raw_definitions r
         JOIN  modules m ON r.module_id = m.id
        WHERE  r.identifier = ?1
         AND   m.id = ?2
        LIMIT  1
    ";

    conn.query_row(
        GET_RAW_ID_BY_RAW_IDENTIFIER_AND_MODULE_ID,
        params![identifier, module_id],
        |row| row.get(0),
    )
    .optional()
}

/// Attempts to find the database ID for a specific raw definition.
///
/// Returns `Ok(Some(id))` if it exists, or `Ok(None)` if it does not.
/// This is useful for checking existence and obtaining the key for updates
/// in a single operation.
///
/// # Errors
///
/// - database error
#[allow(clippy::borrowed_box)]
pub fn try_get_raw_id(conn: &Connection, raw: &Box<dyn RawObject>) -> Result<Option<i64>> {
    const GET_RAW_ID_BY_IDENTIFIER_AND_MODULE_METADATA: &str = r"
        SELECT r.id FROM raw_definitions r
             JOIN modules m ON r.module_id = m.id
            WHERE r.identifier = ?1
             AND  m.identifier = ?2
             AND  m.version = ?3
             AND  m.module_location_id = ?4
            LIMIT 1
     ";

    if raw.get_object_id() != Uuid::nil() {
        try_get_raw_id_by_object_id(conn, raw.get_object_id())
    } else if raw.get_module_object_id() != Uuid::nil() {
        try_get_raw_id_by_identifier_and_module_object_id(
            conn,
            raw.get_identifier(),
            raw.get_module_object_id(),
        )
    } else {
        let meta = raw.get_metadata();
        // Fallback, search for raw by the identifier and module details
        let module_location_id = get_id_for_module_location(conn, meta.get_location())?;
        conn.query_row(
            GET_RAW_ID_BY_IDENTIFIER_AND_MODULE_METADATA,
            params![
                raw.get_identifier(),
                meta.get_module_name(),
                meta.get_module_version(),
                module_location_id
            ],
            |row| row.get(0),
        )
        .optional()
    }
}

/// Creates a new raw definition and populates all associated search and graphics tables.
///
/// # Errors
///
/// - database error
#[allow(clippy::borrowed_box)]
pub fn create_raw(conn: &Connection, raw: &Box<dyn RawObject>) -> Result<i64> {
    let module_id = get_module_id_from_raw(conn, raw)?;
    create_raw_with_module(conn, module_id, raw)
}

/// Updates or creates a raw definition based on its identifier and module identity.
///
/// # Errors
///
/// - database error
#[allow(clippy::borrowed_box)]
pub fn upsert_raw(conn: &Connection, raw: &Box<dyn RawObject>) -> Result<i64> {
    let existing_id: Option<i64> = try_get_raw_id(conn, raw)?;

    match existing_id {
        Some(id) => {
            update_raw(conn, id, raw)?;
            Ok(id)
        }
        None => create_raw(conn, raw),
    }
}

/// Retrieves a raw object by its database ID.
///
/// # Errors
///
/// - database error
pub fn get_raw(conn: &Connection, id: i64) -> Result<Box<dyn RawObject>> {
    const GET_JSON_RAW_BY_ID: &str = "SELECT json(data_blob) FROM raw_definitions WHERE id = ?1";

    let json_str: String = conn.query_row(GET_JSON_RAW_BY_ID, params![id], |row| row.get(0))?;
    serde_json::from_str(&json_str).map_err(|_| rusqlite::Error::InvalidQuery)
}

/// Retrieves a raw object by its object id.
///
/// # Errors
///
/// - database error
pub fn get_raw_by_object_id(conn: &Connection, object_id: Uuid) -> Result<Box<dyn RawObject>> {
    let Some(id) = try_get_raw_id_by_object_id(conn, object_id)? else {
        return Err(rusqlite::Error::InvalidQuery);
    };

    get_raw(conn, id)
}

/// Updates the data blob and associated tables for an existing raw definition.
///
/// # Errors
///
/// - database error
#[allow(clippy::borrowed_box)]
pub fn update_raw(conn: &Connection, id: i64, raw: &Box<dyn RawObject>) -> Result<()> {
    const UPDATE_RAW_JSONB_BY_ID: &str =
        "UPDATE raw_definitions SET data_blob = jsonb(?1) WHERE id = ?2";

    let json_payload = serde_json::to_string(&raw).map_err(|_| rusqlite::Error::InvalidQuery)?;

    conn.execute(UPDATE_RAW_JSONB_BY_ID, params![json_payload, id])?;
    clear_side_tables_for_raw_id(conn, id)?;

    populate_side_tables(conn, id, raw)?;
    Ok(())
}

/// Updates the data blob and associated tables for an existing raw definition.
///
/// # Errors
///
/// - database error
#[allow(clippy::borrowed_box)]
pub fn update_raw_by_object_id(
    conn: &Connection,
    object_id: Uuid,
    raw: &Box<dyn RawObject>,
) -> Result<()> {
    let Some(id) = try_get_raw_id_by_object_id(conn, object_id)? else {
        return Err(rusqlite::Error::InvalidQuery);
    };

    update_raw(conn, id, raw)
}

/// Clear the side tables (search indices and lookup tables) for a given raw id.
///
/// The other relations are not touched (and rely on the cascade delete); e.g. tiles or sprites.
///
/// # Errors
///
/// - database errors
pub fn clear_side_tables_for_raw_id(conn: &Connection, id: i64) -> Result<()> {
    const DELETE_COMMON_FLAGS_FOR_ID: &str = "DELETE FROM common_raw_flags WHERE raw_id = ?1";
    const DELETE_COMMON_NUMERIC_FLAGS_FOR_ID: &str =
        "DELETE FROM common_raw_flags_with_numeric_value WHERE raw_id = ?1";
    const DELETE_SEARCH_IDX_FOR_ID: &str = "DELETE FROM raw_search_index WHERE raw_id = ?1";
    const DELETE_NAME_SEARCH_IDX_FOR_ID: &str = "DELETE FROM raw_names WHERE raw_id = ?1";

    // Clear side tables (CASCADE handles tile_pages, sprite_graphics, raw_names, flags; in this
    // case we explicitly remove the raw_names and flags in order to allow them to be updated if
    // needed)
    conn.execute(DELETE_COMMON_FLAGS_FOR_ID, params![id])?;
    conn.execute(DELETE_COMMON_NUMERIC_FLAGS_FOR_ID, params![id])?;
    conn.execute(DELETE_SEARCH_IDX_FOR_ID, params![id])?;
    conn.execute(DELETE_NAME_SEARCH_IDX_FOR_ID, params![id])?;

    Ok(())
}

/// Deletes a raw definition. FTS5 index is cleared manually.
///
/// # Errors
///
/// - database error
pub fn delete_raw(conn: &Connection, id: i64) -> Result<()> {
    const DELETE_RAW_BY_ID: &str = "DELETE FROM raw_definitions WHERE id = ?1";

    clear_side_tables_for_raw_id(conn, id)?;
    conn.execute(DELETE_RAW_BY_ID, params![id])?;
    Ok(())
}

/// Deletes a raw definition. FTS5 index is cleared manually.
///
/// # Errors
///
/// - database error
pub fn delete_raw_by_object_id(conn: &Connection, object_id: Uuid) -> Result<()> {
    let Some(id) = try_get_raw_id_by_object_id(conn, object_id)? else {
        return Err(rusqlite::Error::InvalidQuery);
    };

    delete_raw(conn, id)
}

/// Retrieves the top result for a module id matching the data in the raw's metadata.
///
/// # Errors
///
/// - database error
#[allow(clippy::borrowed_box)]
pub fn get_module_id_from_raw(conn: &Connection, raw: &Box<dyn RawObject>) -> Result<i64> {
    const GET_MODULE_ID_FOR_RAW_BY_ID: &str = "SELECT id FROM modules WHERE object_id = ?1 LIMIT 1";

    let meta = raw.get_metadata();

    conn.query_row(
        GET_MODULE_ID_FOR_RAW_BY_ID,
        params![meta.get_module_object_id().as_bytes()],
        |row| row.get(0),
    )
}

/// Creates a new raw defintion with a link to a specific module
///
/// # Errors
///
/// - database error
#[allow(clippy::borrowed_box)]
pub fn create_raw_with_module(
    conn: &Connection,
    module_id: i64,
    raw: &Box<dyn RawObject>,
) -> Result<i64> {
    let json_payload = serde_json::to_string(&raw).map_err(|_| rusqlite::Error::InvalidQuery)?;

    let raw_id: i64 = conn.query_row(
        INSERT_RAW_DEFINITION_NO_UPDATE_RETURN_ID,
        params![
            u32::from(raw.get_type()),
            raw.get_identifier(),
            module_id,
            json_payload,
            raw.get_object_id().as_bytes()
        ],
        |row| row.get(0),
    )?;

    populate_side_tables(conn, raw_id, raw)?;
    Ok(raw_id)
}

/// Helper to populate the "side tables", i.e. the flag and search tables for most raws, and the graphic tables
/// for graphics.
#[allow(clippy::borrowed_box)]
fn populate_side_tables(conn: &Connection, raw_id: i64, raw: &Box<dyn RawObject>) -> Result<()> {
    // Flags
    for flag in raw.get_searchable_tokens() {
        conn.execute(INSERT_COMMON_FLAG, params![raw_id, flag])?;
    }

    let mut search_names = Vec::<&str>::new();
    let mut search_descriptions = Vec::<&str>::new();

    match raw.get_type() {
        ObjectType::Creature => {
            if let Some(c) = raw.as_any().downcast_ref::<Creature>() {
                search_names.clone_from(&c.get_all_names());
                search_descriptions.clone_from(&c.get_all_descriptions());
            }
        }
        ObjectType::TilePage => {
            if let Some(tp) = raw.as_any().downcast_ref::<TilePage>() {
                let tile_dimensions = tp.get_tile_dimensions();
                let page_dimensions = tp.get_page_dimensions();
                conn.execute(
                    INSERT_TILE_PAGE,
                    params![
                        raw_id,
                        tp.get_identifier(),
                        tp.get_file_path().to_str(),
                        tile_dimensions.x,
                        tile_dimensions.y,
                        page_dimensions.x,
                        page_dimensions.y
                    ],
                )?;
            }
        }
        ObjectType::Graphics => {
            if let Some(g) = raw.as_any().downcast_ref::<Graphic>() {
                for s in &g.get_sprites() {
                    let s_offset = s.get_offset();
                    if let Some(s_offset_2) = s.get_offset2() {
                        conn.execute(
                            INSERT_LARGE_SPRITE_GRAPHIC,
                            params![
                                raw_id,
                                s.get_tile_page_id(),
                                s_offset.x,
                                s_offset.y,
                                s_offset_2.x,
                                s_offset_2.y,
                                &s.get_primary_condition().to_string(),
                                &s.get_secondary_condition().to_string(),
                                g.get_identifier()
                            ],
                        )?;
                    } else {
                        conn.execute(
                            INSERT_SPRITE_GRAPHIC,
                            params![
                                raw_id,
                                s.get_tile_page_id(),
                                s_offset.x,
                                s_offset.y,
                                &s.get_primary_condition().to_string(),
                                &s.get_secondary_condition().to_string(),
                                g.get_identifier()
                            ],
                        )?;
                    }
                }
            }
        }
        _ => {}
    }

    for n in &search_names {
        conn.execute(INSERT_LOOKUP_NAME, params![raw_id, n])?;
    }

    conn.execute(
        INSERT_SEARCH_INDEX,
        params![
            raw_id,
            search_names.join(" "),
            search_descriptions.join(" ")
        ],
    )?;

    Ok(())
}
