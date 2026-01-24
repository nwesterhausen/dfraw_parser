use dfraw_parser::{Creature, Graphic, TilePage, tokens::ObjectType, traits::RawObject};
use rusqlite::{Connection, Result, params};

use crate::db::queries::get_id_for_module_location;

use super::super::rusqlite_extensions::OptionalResultExtension;

/// Returns true if the raw exists in the database.
///
/// Searches for a match based on the raw identifier and its metadata: location,
/// module name and module version.
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
    let meta = raw.get_metadata();
    let module_location_id = get_id_for_module_location(conn, meta.get_location())?;

    conn.query_row(
        "SELECT r.id FROM raw_definitions r
         JOIN modules m ON r.module_id = m.id
         WHERE r.identifier = ?1
           AND m.identifier = ?2
           AND m.version = ?3
           AND m.module_location_id = ?4
         LIMIT 1",
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

/// Creates a new raw definition and populates all associated search and graphics tables.
///
/// # Errors
///
/// - database error
#[allow(clippy::borrowed_box)]
pub fn create_raw_definition(conn: &Connection, raw: &Box<dyn RawObject>) -> Result<i64> {
    let module_id = get_module_id_from_raw(conn, raw)?;
    create_raw_definition_with_module(conn, module_id, raw)
}

/// Updates or creates a raw definition based on its identifier and module identity.
///
/// # Errors
///
/// - database error
#[allow(clippy::borrowed_box)]
pub fn upsert_raw_definition(conn: &Connection, raw: &Box<dyn RawObject>) -> Result<i64> {
    let existing_id: Option<i64> = try_get_raw_id(conn, raw)?;

    match existing_id {
        Some(id) => {
            update_raw_definition(conn, id, raw)?;
            Ok(id)
        }
        None => create_raw_definition(conn, raw),
    }
}

/// Retrieves a raw object by its database ID.
///
/// # Errors
///
/// - database error
pub fn get_raw_definition(conn: &Connection, id: i64) -> Result<Box<dyn RawObject>> {
    let json_str: String = conn.query_row(
        "SELECT json(data_blob) FROM raw_definitions WHERE id = ?1",
        params![id],
        |row| row.get(0),
    )?;
    serde_json::from_str(&json_str).map_err(|_| rusqlite::Error::InvalidQuery)
}

/// Updates the data blob and associated tables for an existing raw definition.
///
/// # Errors
///
/// - database error
#[allow(clippy::borrowed_box)]
pub fn update_raw_definition(conn: &Connection, id: i64, raw: &Box<dyn RawObject>) -> Result<()> {
    let json_payload = serde_json::to_string(&raw).map_err(|_| rusqlite::Error::InvalidQuery)?;

    conn.execute(
        "UPDATE raw_definitions SET data_blob = jsonb(?1) WHERE id = ?2",
        params![json_payload, id],
    )?;

    // Clear side tables (CASCADE handles tile_pages, sprite_graphics, raw_names, flags)
    conn.execute(
        "DELETE FROM common_raw_flags WHERE raw_id = ?1",
        params![id],
    )?;
    conn.execute("DELETE FROM raw_names WHERE raw_id = ?1", params![id])?;
    conn.execute(
        "DELETE FROM raw_search_index WHERE raw_id = ?1",
        params![id],
    )?;

    populate_side_tables(conn, id, raw)?;
    Ok(())
}

/// Deletes a raw definition. FTS5 index is cleared manually.
///
/// # Errors
///
/// - database error
pub fn delete_raw_definition(conn: &Connection, id: i64) -> Result<()> {
    // FTS5 doesn't support ON DELETE CASCADE
    conn.execute(
        "DELETE FROM raw_search_index WHERE raw_id = ?1",
        params![id],
    )?;
    conn.execute("DELETE FROM raw_definitions WHERE id = ?1", params![id])?;
    Ok(())
}

/// Retrieves the top result for a module id matching the data in the raw's metadata.
///
/// # Errors
///
/// - database error
#[allow(clippy::borrowed_box)]
pub fn get_module_id_from_raw(conn: &Connection, raw: &Box<dyn RawObject>) -> Result<i64> {
    let meta = raw.get_metadata();

    conn.query_row(
        "SELECT id FROM modules WHERE object_id = ?1 LIMIT 1",
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
pub fn create_raw_definition_with_module(
    conn: &Connection,
    module_id: i64,
    raw: &Box<dyn RawObject>,
) -> Result<i64> {
    let json_payload = serde_json::to_string(&raw).map_err(|_| rusqlite::Error::InvalidQuery)?;

    conn.execute(
        "INSERT INTO raw_definitions (raw_type_id, identifier, module_id, data_blob, object_id)
         VALUES ((SELECT id FROM raw_types WHERE name = ?1), ?2, ?3, jsonb(?4), ?5)",
        params![
            raw.get_type().to_string().to_uppercase().replace(' ', "_"),
            raw.get_identifier(),
            module_id,
            json_payload,
            raw.get_object_id().as_bytes()
        ],
    )?;

    let raw_id = conn.last_insert_rowid();
    populate_side_tables(conn, raw_id, raw)?;
    Ok(raw_id)
}

/// Helper to populate the "side tables", i.e. the flag and search tables for most raws, and the graphic tables
/// for graphics.
#[allow(clippy::borrowed_box)]
fn populate_side_tables(conn: &Connection, raw_id: i64, raw: &Box<dyn RawObject>) -> Result<()> {
    // Flags
    for flag in raw.get_searchable_tokens() {
        conn.execute(
            "INSERT INTO common_raw_flags (raw_id, token_name) VALUES (?1, ?2)",
            params![raw_id, flag],
        )?;
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
                "INSERT INTO tile_pages (raw_id, identifier, file_path, tile_width, tile_height, page_width, page_height) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![raw_id, tp.get_identifier(), tp.get_file_path().to_str(), tile_dimensions.x, tile_dimensions.y, page_dimensions.x, page_dimensions.y]
            )?;
            }
        }
        ObjectType::Graphics => {
            if let Some(g) = raw.as_any().downcast_ref::<Graphic>() {
                for s in &g.get_sprites() {
                    let s_offset = s.get_offset();
                    if let Some(s_offset_2) = s.get_offset2() {
                        conn.execute(
                        "INSERT INTO sprite_graphics (raw_id, tile_page_identifier, offset_x, offset_y, offset_x_2, offset_y_2, primary_condition, secondary_condition, target_identifier) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                        params![raw_id, s.get_tile_page_id(), s_offset.x, s_offset.y, s_offset_2.x, s_offset_2.y, &s.get_primary_condition().to_string(), g.get_identifier()]
                    )?;
                    } else {
                        conn.execute(
                    "INSERT INTO sprite_graphics (raw_id, tile_page_identifier, offset_x, offset_y, primary_condition, secondary_condition, target_identifier) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                    params![raw_id, s.get_tile_page_id(), s_offset.x, s_offset.y, &s.get_primary_condition().to_string(), g.get_identifier()]
                )?;
                    }
                }
            }
        }
        _ => {}
    }

    for n in &search_names {
        conn.execute(
            "INSERT INTO raw_names (raw_id, name) VALUES (?1, ?2)",
            params![raw_id, n],
        )?;
    }

    conn.execute(
        "INSERT INTO raw_search_index (raw_id, names, description) VALUES (?1, ?2, ?3)",
        params![
            raw_id,
            search_names.join(" "),
            search_descriptions.join(" ")
        ],
    )?;

    Ok(())
}
