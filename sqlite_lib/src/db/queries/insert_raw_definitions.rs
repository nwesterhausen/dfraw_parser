use dfraw_parser::{
    Graphic, InfoFile, TilePage, metadata::ObjectType, tags::ConditionTag, traits::RawObject,
};
use rusqlite::{Result, Transaction, params};
use tracing::error;

use crate::{db::util::remove_dup_strings, search_helpers::extract_names_and_descriptions};

use super::super::rusqlite_extensions::OptionalResultExtension;

/// Inserts a batch of raws using prepared statements for efficiency.
///
/// # Errors
///
/// - Database error (will not commit transaction if error)
#[allow(clippy::too_many_lines)]
pub fn process_raw_insertions(
    tx: &Transaction,
    module_db_id: i64,
    info: &InfoFile,
    raws: &[Box<dyn RawObject>],
    overwrite_raws: bool,
) -> Result<()> {
    let mut error_count = 0;

    // Check if a raw exists already
    let mut check_raw_stmt = tx.prepare_cached(
        "SELECT id FROM raw_definitions WHERE module_id = ?1 AND identifier = ?2 LIMIT 1",
    )?;

    // Insert new raw data
    let mut insert_raw_stmt = tx.prepare_cached(
        "INSERT INTO raw_definitions (raw_type_id, identifier, module_id, data_blob)
         VALUES ((SELECT id FROM raw_types WHERE name = ?1), ?2, ?3, jsonb(?4))",
    )?;

    // Search by name
    let mut insert_name_stmt =
        tx.prepare_cached("INSERT INTO raw_names (raw_id, name) VALUES (?1, ?2)")?;

    // Search descriptions/details about raw
    let mut insert_search_stmt = tx.prepare_cached(
        "INSERT INTO raw_search_index (raw_id, names, description) VALUES (?1, ?2, ?3)",
    )?;

    // Clear search names (used when overwriting raws) - virutal table doesn't support delete cascade
    let mut clear_names_stmt = tx.prepare_cached("DELETE FROM raw_names WHERE raw_id = ?1")?;

    // Clear search descriptions (used when overwriting raws) - virutal table doesn't support delete cascade
    let mut delete_search_stmt =
        tx.prepare_cached("DELETE FROM raw_search_index WHERE raw_id = ?1")?;

    // Update raw_defintion (used when overwriting raws)
    let mut update_raw_stmt =
        tx.prepare_cached("UPDATE raw_definitions SET data_blob = jsonb(?1) WHERE id = ?2")?;

    // Insert common flags (flags without values) into searchable table
    let mut insert_flag_stmt =
        tx.prepare_cached("INSERT INTO common_raw_flags (raw_id, token_name) VALUES (?1, ?2)")?;

    // Clear common flags (if we overwrite, then reset stored flags)
    let mut clear_flags_stmt =
        tx.prepare_cached("DELETE FROM common_raw_flags WHERE raw_id = ?1")?;

    // Special case for graphics for ease of retrieval.
    let mut insert_tile_page_stmt = tx.prepare_cached(
        "INSERT INTO tile_pages
            (raw_id, identifier, file_path, tile_width, tile_height, page_width, page_height)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
    )?;

    // Regular sprite
    let mut insert_sprite_graphic_stmt = tx.prepare_cached(
        "INSERT INTO sprite_graphics
        (raw_id, tile_page_identifier, offset_x, offset_y, primary_condition, secondary_condition, target_identifier)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
    )?;

    // Large sprites
    let mut insert_large_sprite_graphic_stmt = tx.prepare_cached(
        "INSERT INTO sprite_graphics
        (raw_id, tile_page_identifier, offset_x, offset_y, offset_x_2, offset_y_2, primary_condition, secondary_condition, target_identifier)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
    )?;

    for raw in raws {
        let existing_raw_id: Option<i64> = check_raw_stmt
            .query_row(params![module_db_id, raw.get_identifier()], |row| {
                row.get(0)
            })
            .optional()?;

        // Handle Serialization with retry/exit logic
        let json_payload = match serde_json::to_string(&raw) {
            Ok(payload) => payload,
            Err(e) => {
                error_count += 1;
                error!(
                    "Failed to serialize raw '{}' in module {}: {}",
                    raw.get_identifier(),
                    info.get_identifier(),
                    e
                );

                if error_count >= 5 {
                    error!(
                        "Reached maximum serialization error threshold (5) for module {}. Aborting insertion.",
                        info.get_identifier()
                    );
                    return Err(rusqlite::Error::InvalidQuery);
                }
                continue;
            }
        };

        let mut exists_already = false;
        let raw_db_id = if let Some(id) = existing_raw_id {
            if overwrite_raws {
                update_raw_stmt
                    .execute(params![json_payload, id])
                    .inspect_err(|e| {
                        tracing::error!(
                            "Failed updating {} ({}): {e}",
                            raw.get_identifier(),
                            raw.get_type().to_string().to_uppercase().replace(' ', "_")
                        );
                    })?;
                clear_flags_stmt.execute(params![id]).inspect_err(|e| {
                    tracing::error!(
                        "Failed clearing flags for {} ({}): {e}",
                        raw.get_identifier(),
                        raw.get_type().to_string().to_uppercase().replace(' ', "_")
                    );
                })?;
                clear_names_stmt.execute(params![id]).inspect_err(|e| {
                    tracing::error!(
                        "Failed clearing names for {} ({}): {e}",
                        raw.get_identifier(),
                        raw.get_type().to_string().to_uppercase().replace(' ', "_")
                    );
                })?;
                delete_search_stmt.execute(params![id]).inspect_err(|e| {
                    tracing::error!(
                        "Failed removing search index for {} ({}): {e}",
                        raw.get_identifier(),
                        raw.get_type().to_string().to_uppercase().replace(' ', "_")
                    );
                })?;
            } else {
                // Skip if exists and not overwriting
                tracing::debug!("Avoiding overwrite of {} (id:{id})", raw.get_identifier());
            }
            exists_already = true;
            id
        } else {
            insert_raw_stmt
                .execute(params![
                    raw.get_type().to_string().to_uppercase().replace(' ', "_"),
                    raw.get_identifier(),
                    module_db_id,
                    json_payload
                ])
                .inspect_err(|e| {
                    tracing::error!(
                        "Failed inserting {} ({}): {e}",
                        raw.get_identifier(),
                        raw.get_type().to_string().to_uppercase().replace(' ', "_")
                    );
                })?;
            tx.last_insert_rowid()
        };

        // Only run flag and search updates if we are overwriting or new definition
        if !exists_already || overwrite_raws {
            for flag in raw.get_searchable_tokens() {
                insert_flag_stmt
                    .execute(params![raw_db_id, flag])
                    .inspect_err(|e| {
                        tracing::error!(
                            "Failed inserting flags for {} ({}): {e}",
                            raw.get_identifier(),
                            raw.get_type().to_string().to_uppercase().replace(' ', "_")
                        );
                    })?;
            }

            let (search_names, search_descriptions) = extract_names_and_descriptions(raw);

            // Populate Names Table (for Exact/Partial ID lookup)
            for name in &search_names {
                insert_name_stmt.execute(params![raw_db_id, name])?;
            }

            // Populate FTS5 Index (for high-speed text search)
            insert_search_stmt
                .execute(params![
                    raw_db_id,
                    remove_dup_strings(search_names, true).join(" "),
                    search_descriptions.join(" ")
                ])
                .inspect_err(|e| {
                    tracing::error!(
                        "Failed inserting search index for {} ({}): {e}",
                        raw.get_identifier(),
                        raw.get_type().to_string().to_uppercase().replace(' ', "_")
                    );
                })?;
        }

        // Handle extra graphic data
        // Portraits and other sprites are defined in two separate files, so we have to allow insertion of new
        // graphics and tile pages if possible.
        match raw.get_type() {
            ObjectType::TilePage => {
                if let Some(tp) = raw.as_any().downcast_ref::<TilePage>() {
                    let tile_dimensions = tp.get_tile_dimensions();
                    let page_dimensions = tp.get_page_dimensions();
                    insert_tile_page_stmt
                        .execute(params![
                            raw_db_id,
                            tp.get_identifier(),
                            tp.get_file_path().to_str(),
                            tile_dimensions.x,
                            tile_dimensions.y,
                            page_dimensions.x,
                            page_dimensions.y
                        ])
                        .inspect_err(|e| {
                            tracing::error!(
                                "Failed inserting tile page for {} ({}): {e}",
                                raw.get_identifier(),
                                raw.get_type().to_string().to_uppercase().replace(' ', "_")
                            );
                        })?;
                }
            }
            ObjectType::Graphics => {
                if let Some(g) = raw.as_any().downcast_ref::<Graphic>() {
                    // Insert any sprites
                    for s in &g.get_sprites() {
                        let s_offset = s.get_offset();
                        if let Some(s_offset_2) = s.get_offset2() {
                            insert_large_sprite_graphic_stmt
                                .execute(params![
                                    raw_db_id,
                                    s.get_tile_page_id(),
                                    s_offset.x,
                                    s_offset.y,
                                    s_offset_2.x,
                                    s_offset_2.y,
                                    ConditionTag::get_key(&s.get_primary_condition())
                                        .unwrap_or_default(),
                                    ConditionTag::get_key(&s.get_secondary_condition()),
                                    g.get_identifier()
                                ])
                                .inspect_err(|e| {
                                    tracing::error!(
                                        "Failed inserting sprite graphic for {} ({}): {e}",
                                        raw.get_identifier(),
                                        raw.get_type().to_string().to_uppercase().replace(' ', "_")
                                    );
                                })?;
                        } else {
                            insert_sprite_graphic_stmt
                                .execute(params![
                                    raw_db_id,
                                    s.get_tile_page_id(),
                                    s_offset.x,
                                    s_offset.y,
                                    ConditionTag::get_key(&s.get_primary_condition())
                                        .unwrap_or_default(),
                                    ConditionTag::get_key(&s.get_secondary_condition()),
                                    g.get_identifier()
                                ])
                                .inspect_err(|e| {
                                    tracing::error!(
                                        "Failed inserting sprite graphic for {} ({}): {e}",
                                        raw.get_identifier(),
                                        raw.get_type().to_string().to_uppercase().replace(' ', "_")
                                    );
                                })?;
                        }
                    }
                    // Insert _some_ layers. Specifically we care about portraits for now.
                    for l in &g.get_layers() {
                        let primary_condition = l.0.clone();
                        // layers are (NAME: [LAYER DEFINTIONS..])
                        for layer in &l.1 {
                            let s_offset = layer.get_offset();
                            if let Some(s_offset_2) = layer.get_offset2() {
                                insert_large_sprite_graphic_stmt
                                    .execute(params![
                                        raw_db_id,
                                        layer.get_tile_page_id(),
                                        s_offset.x,
                                        s_offset.y,
                                        s_offset_2.x,
                                        s_offset_2.y,
                                        primary_condition,
                                        Some(&layer.get_name()),
                                        g.get_identifier()
                                    ])
                                    .inspect_err(|e| {
                                        tracing::error!(
                                            "Failed inserting sprite layer graphic for {} ({}): {e}",
                                            raw.get_identifier(),
                                            raw.get_type()
                                                .to_string()
                                                .to_uppercase()
                                                .replace(' ', "_")
                                        );
                                    })?;
                            } else {
                                insert_sprite_graphic_stmt
                                    .execute(params![
                                        raw_db_id,
                                        layer.get_tile_page_id(),
                                        s_offset.x,
                                        s_offset.y,
                                        primary_condition,
                                        Some(&layer.get_name()),
                                        g.get_identifier()
                                    ])
                                    .inspect_err(|e| {
                                        tracing::error!(
                                            "Failed inserting sprite graphic for {} ({}): {e}",
                                            raw.get_identifier(),
                                            raw.get_type()
                                                .to_string()
                                                .to_uppercase()
                                                .replace(' ', "_")
                                        );
                                    })?;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}
