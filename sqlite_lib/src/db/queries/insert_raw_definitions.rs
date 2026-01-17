#[cfg(debug_assertions)]
use chrono::Utc;
use dfraw_parser::{
    Graphic, InfoFile, TilePage,
    tags::{ConditionTag, ObjectType},
    traits::RawObject,
};
use rusqlite::{Result, Transaction, params};
use tracing::error;

use crate::{db::util::remove_dup_strings, search_helpers::extract_names_and_descriptions};

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
    // batching of expensive or extremely-numerous operations
    let mut pending_search_batch = Vec::new();
    let mut pending_sprites_batch = Vec::new();
    let mut pending_flags_batch = Vec::new();
    let mut pending_names_batch = Vec::new();

    // Insert new raw data
    // Choose the statement based on the overwrite preference
    let mut upsert_stmt = if overwrite_raws {
        // UPSERT: Insert or update and always return the ID
        tx.prepare_cached(
            "INSERT INTO raw_definitions (raw_type_id, identifier, module_id, data_blob)
                 VALUES ((SELECT id FROM raw_types WHERE name = ?1), ?2, ?3, jsonb(?4))
                 ON CONFLICT(module_id, identifier) DO UPDATE SET
                    data_blob = excluded.data_blob
                 RETURNING id",
        )?
    } else {
        // INSERT OR IGNORE: Only insert if new; RETURNING id will be empty on conflict
        tx.prepare_cached(
            "INSERT INTO raw_definitions (raw_type_id, identifier, module_id, data_blob)
                 VALUES ((SELECT id FROM raw_types WHERE name = ?1), ?2, ?3, jsonb(?4))
                 ON CONFLICT(module_id, identifier) DO NOTHING
                 RETURNING id",
        )?
    };

    // Clear out existing names/flags entries for this module
    let mut delete_existing_flag_relations_stmt = tx.prepare_cached(
        "DELETE FROM common_raw_flags
        WHERE raw_id IN (SELECT id FROM raw_definitions WHERE module_id = ?1);",
    )?;
    let mut delete_existing_name_relations_stmt = tx.prepare_cached(
        "DELETE FROM raw_names
        WHERE raw_id IN (SELECT id FROM raw_definitions WHERE module_id = ?1);",
    )?;

    // Search by name
    let mut insert_name_stmt =
        tx.prepare_cached("INSERT INTO raw_names (raw_id, name) VALUES (?1, ?2)")?;
    // Search by flag
    let mut insert_flag_stmt =
        tx.prepare_cached("INSERT INTO common_raw_flags (raw_id, token_name) VALUES (?1, ?2)")?;

    // Search descriptions/details about raw
    let mut upsert_search_stmt = tx.prepare_cached(
        "INSERT OR REPLACE INTO raw_search_index (raw_id, names, description)
        VALUES (?1, ?2, ?3);",
    )?;

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

    #[cfg(debug_assertions)]
    let mut total_serialization_time = chrono::TimeDelta::zero();
    #[cfg(debug_assertions)]
    let mut total_db_time = chrono::TimeDelta::zero();
    #[cfg(debug_assertions)]
    let mut total_search_index_time = chrono::TimeDelta::zero();
    #[cfg(debug_assertions)]
    let mut total_graphic_time = chrono::TimeDelta::zero();

    // Clear out existing names/flags if overwrite is true
    if overwrite_raws {
        delete_existing_flag_relations_stmt
            .execute(params![module_db_id])
            .inspect_err(|e| {
                tracing::error!(
                    "Failed clearing existing flag relations for module:{module_db_id}: {e}",
                );
            })?;
        delete_existing_name_relations_stmt
            .execute(params![module_db_id])
            .inspect_err(|e| {
                tracing::error!(
                    "Failed clearing existing flag relations for module:{module_db_id}: {e}",
                );
            })?;
    }

    for raw in raws {
        // Trace serialization
        #[cfg(debug_assertions)]
        let serialize_start = Utc::now();

        // Handle Serialization with retry/exit logic
        let json_payload = match serde_json::to_vec(&raw) {
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

        #[cfg(debug_assertions)]
        {
            let serialize_duration = Utc::now().signed_duration_since(serialize_start);
            total_serialization_time += serialize_duration;
        }
        // Trace the main definition insert
        #[cfg(debug_assertions)]
        let db_start = Utc::now();

        let raw_db_id: i64 = match upsert_stmt.query_row(
            params![
                raw.get_type().to_string().to_uppercase().replace(' ', "_"),
                raw.get_identifier(),
                module_db_id,
                json_payload // Bound as a BLOB
            ],
            |row| row.get(0),
        ) {
            Ok(id) => id,
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                // This happens when overwrite_raws is false and the raw already exists
                continue;
            }
            Err(e) => return Err(e),
        };

        #[cfg(debug_assertions)]
        {
            let insert_duration = Utc::now().signed_duration_since(db_start);
            total_db_time += insert_duration;
        }

        // Only run flag and search updates if we are overwriting or new definition
        for flag in raw.get_searchable_tokens() {
            pending_flags_batch.push(PendingFlag {
                raw_id: raw_db_id,
                token_name: flag.to_string(),
            });
        }

        let (search_names, search_descriptions) = extract_names_and_descriptions(raw);

        // Populate Names Table (for Exact/Partial ID lookup)
        for name in &search_names {
            pending_names_batch.push(PendingName {
                raw_id: raw_db_id,
                name: name.to_string(),
            });
        }

        pending_search_batch.push(PendingSearch {
            raw_id: raw_db_id,
            names: remove_dup_strings(search_names, true).join(" "),
            description: search_descriptions.join(" "),
        });

        // Handle extra graphic data
        // Portraits and other sprites are defined in two separate files, so we have to allow insertion of new
        // graphics and tile pages if possible.
        match raw.get_type() {
            ObjectType::TilePage => {
                if let Some(tp) = raw.as_any().downcast_ref::<TilePage>() {
                    #[cfg(debug_assertions)]
                    let graphic_start = Utc::now();
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
                    #[cfg(debug_assertions)]
                    {
                        let graphic_duration = Utc::now().signed_duration_since(graphic_start);
                        total_graphic_time += graphic_duration;
                    }
                }
            }
            ObjectType::Graphics => {
                if let Some(g) = raw.as_any().downcast_ref::<Graphic>() {
                    // Insert any sprites
                    for s in &g.get_sprites() {
                        let s_offset = s.get_offset();
                        if let Some(s_offset_2) = s.get_offset2() {
                            pending_sprites_batch.push(PendingSprite {
                                raw_id: raw_db_id,
                                tile_page_id: s.get_tile_page_id().to_string(),
                                offset: s_offset.into(),
                                offset2: Some(s_offset_2.into()),
                                primary_cond: ConditionTag::get_key(&s.get_primary_condition())
                                    .unwrap_or_default()
                                    .to_string(),
                                secondary_cond: ConditionTag::get_key(&s.get_secondary_condition())
                                    .map(String::from),
                                target_id: g.get_identifier().to_string(),
                            });
                        } else {
                            pending_sprites_batch.push(PendingSprite {
                                raw_id: raw_db_id,
                                tile_page_id: s.get_tile_page_id().to_string(),
                                offset: s_offset.into(),
                                offset2: None,
                                primary_cond: ConditionTag::get_key(&s.get_primary_condition())
                                    .unwrap_or_default()
                                    .to_string(),
                                secondary_cond: ConditionTag::get_key(&s.get_secondary_condition())
                                    .map(String::from),
                                target_id: g.get_identifier().to_string(),
                            });
                        }
                    }
                    // Insert _some_ layers. Specifically we care about portraits for now.
                    for l in &g.get_layers() {
                        let primary_condition = l.0.clone();
                        // layers are (NAME: [LAYER DEFINTIONS..])
                        for layer in &l.1 {
                            let s_offset = layer.get_offset();
                            if let Some(s_offset_2) = layer.get_offset2() {
                                pending_sprites_batch.push(PendingSprite {
                                    raw_id: raw_db_id,
                                    tile_page_id: layer.get_tile_page_id().to_string(),
                                    offset: s_offset.into(),
                                    offset2: Some(s_offset_2.into()),
                                    primary_cond: primary_condition.clone(),
                                    secondary_cond: Some(String::from(&layer.get_name())),
                                    target_id: g.get_identifier().to_string(),
                                });
                            } else {
                                pending_sprites_batch.push(PendingSprite {
                                    raw_id: raw_db_id,
                                    tile_page_id: layer.get_tile_page_id().to_string(),
                                    offset: s_offset.into(),
                                    offset2: None,
                                    primary_cond: primary_condition.clone(),
                                    secondary_cond: Some(String::from(&layer.get_name())),
                                    target_id: g.get_identifier().to_string(),
                                });
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        // Check if we have >= 5000 pending graphics and insert them
        if pending_sprites_batch.len() >= 5000 {
            #[cfg(debug_assertions)]
            let graphic_start = Utc::now();
            for s in pending_sprites_batch {
                let (x1, y1) = s.offset;
                if let Some((x2, y2)) = s.offset2 {
                    insert_large_sprite_graphic_stmt
                        .execute(params![
                            s.raw_id,
                            s.tile_page_id,
                            x1,
                            y1,
                            x2,
                            y2,
                            s.primary_cond,
                            s.secondary_cond,
                            s.target_id
                        ])
                        .inspect_err(|e| {
                            tracing::error!(
                                "Failed inserting sprite graphic for raw_id:{} target:{}: {e}",
                                s.raw_id,
                                s.target_id
                            );
                        })?;
                } else {
                    insert_sprite_graphic_stmt
                        .execute(params![
                            s.raw_id,
                            s.tile_page_id,
                            x1,
                            y1,
                            s.primary_cond,
                            s.secondary_cond,
                            s.target_id
                        ])
                        .inspect_err(|e| {
                            tracing::error!(
                                "Failed inserting sprite graphic for raw_id:{} target:{}: {e}",
                                s.raw_id,
                                s.target_id
                            );
                        })?;
                }
            }
            #[cfg(debug_assertions)]
            {
                let graphic_duration = Utc::now().signed_duration_since(graphic_start);
                total_graphic_time += graphic_duration;
            }
            // reset batch
            pending_sprites_batch = Vec::new();
        }
    }

    // Insert pending search batches
    #[cfg(debug_assertions)]
    let search_start = Utc::now();
    for s in pending_search_batch {
        // Populate FTS5 Index (for high-speed text search)
        upsert_search_stmt
            .execute(params![s.raw_id, s.names, s.description,])
            .inspect_err(|e| {
                tracing::error!("Failed inserting search index for raw_id:{}: {e}", s.raw_id,);
            })?;
    }
    // Insert pending name batch
    for n in pending_names_batch {
        insert_name_stmt
            .execute(params![n.raw_id, n.name])
            .inspect_err(|e| {
                tracing::error!("Failed inserting names for raw_id:{}: {e}", n.raw_id,);
            })?;
    }
    // Insert pending flag batch
    for f in pending_flags_batch {
        insert_flag_stmt
            .execute(params![f.raw_id, f.token_name])
            .inspect_err(|e| {
                tracing::error!("Failed inserting flags for raw_id:{}: {e}", f.raw_id,);
            })?;
    }
    #[cfg(debug_assertions)]
    {
        let search_duration = Utc::now().signed_duration_since(search_start);
        total_search_index_time += search_duration;
    }

    // Insert remaining graphics
    #[cfg(debug_assertions)]
    let graphic_start = Utc::now();
    for s in pending_sprites_batch {
        let (x1, y1) = s.offset;
        if let Some((x2, y2)) = s.offset2 {
            insert_large_sprite_graphic_stmt
                .execute(params![
                    s.raw_id,
                    s.tile_page_id,
                    x1,
                    y1,
                    x2,
                    y2,
                    s.primary_cond,
                    s.secondary_cond,
                    s.target_id
                ])
                .inspect_err(|e| {
                    tracing::error!(
                        "Failed inserting sprite graphic for raw_id:{} target:{}: {e}",
                        s.raw_id,
                        s.target_id
                    );
                })?;
        } else {
            insert_sprite_graphic_stmt
                .execute(params![
                    s.raw_id,
                    s.tile_page_id,
                    x1,
                    y1,
                    s.primary_cond,
                    s.secondary_cond,
                    s.target_id
                ])
                .inspect_err(|e| {
                    tracing::error!(
                        "Failed inserting sprite graphic for raw_id:{} target:{}: {e}",
                        s.raw_id,
                        s.target_id
                    );
                })?;
        }
    }
    #[cfg(debug_assertions)]
    {
        let graphic_duration = Utc::now().signed_duration_since(graphic_start);
        total_graphic_time += graphic_duration;

        tracing::info!(
            "Module {} Summary: Serialize: {}μs, Core DB: {}μs, Search/FTS5: {}μs, Graphics (all): {}μs",
            info.get_identifier(),
            total_serialization_time
                .num_microseconds()
                .unwrap_or_default(),
            total_db_time.num_microseconds().unwrap_or_default(),
            total_search_index_time
                .num_microseconds()
                .unwrap_or_default(),
            total_graphic_time.num_microseconds().unwrap_or_default(),
        );
    }
    Ok(())
}

struct PendingSearch {
    raw_id: i64,
    names: String,
    description: String,
}

struct PendingFlag {
    raw_id: i64,
    token_name: String,
}

struct PendingName {
    raw_id: i64,
    name: String,
}

struct PendingSprite {
    raw_id: i64,
    tile_page_id: String,
    offset: (i64, i64),
    offset2: Option<(i64, i64)>,
    primary_cond: String,
    secondary_cond: Option<String>,
    target_id: String,
}
