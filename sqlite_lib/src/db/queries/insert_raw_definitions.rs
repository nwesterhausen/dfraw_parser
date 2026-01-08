use dfraw_parser::{InfoFile, traits::RawObject};
use rusqlite::{Result, Transaction, params};
use tracing::error;

use crate::{db::util::remove_dup_strings, search_helpers::extract_names_and_descriptions};

use super::super::rusqlite_extensions::OptionalResultExtension;

/// Inserts a batch of raws using prepared statements for efficiency.
///
/// # Errors
///
/// - Database error (will not commit transaction if error)
pub fn process_raw_insertions(
    tx: &Transaction,
    module_db_id: i64,
    info: &InfoFile,
    raws: &[Box<dyn RawObject>],
    overwrite_raws: bool,
) -> Result<()> {
    let mut error_count = 0;

    let mut check_raw_stmt = tx.prepare_cached(
        "SELECT id FROM raw_definitions WHERE module_id = ?1 AND identifier = ?2 LIMIT 1",
    )?;

    let mut insert_raw_stmt = tx.prepare_cached(
        "INSERT INTO raw_definitions (raw_type_id, identifier, module_id, data_blob)
         VALUES ((SELECT id FROM raw_types WHERE name = ?1), ?2, ?3, jsonb(?4))",
    )?;

    // Search Index Statements
    let mut insert_name_stmt =
        tx.prepare_cached("INSERT INTO raw_names (raw_id, name) VALUES (?1, ?2)")?;
    let mut insert_search_stmt = tx.prepare_cached(
        "INSERT INTO raw_search_index (raw_id, names, description) VALUES (?1, ?2, ?3)",
    )?;
    let mut clear_names_stmt = tx.prepare_cached("DELETE FROM raw_names WHERE raw_id = ?1")?;
    let mut delete_search_stmt =
        tx.prepare_cached("DELETE FROM raw_search_index WHERE raw_id = ?1")?;

    let mut update_raw_stmt =
        tx.prepare_cached("UPDATE raw_definitions SET data_blob = jsonb(?1) WHERE id = ?2")?;

    let mut insert_flag_stmt =
        tx.prepare_cached("INSERT INTO common_raw_flags (raw_id, token_name) VALUES (?1, ?2)")?;

    let mut clear_flags_stmt =
        tx.prepare_cached("DELETE FROM common_raw_flags WHERE raw_id = ?1")?;

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

        let raw_db_id = match existing_raw_id {
            Some(id) if overwrite_raws => {
                update_raw_stmt.execute(params![json_payload, id])?;
                clear_flags_stmt.execute(params![id])?;
                clear_names_stmt.execute(params![id])?;
                delete_search_stmt.execute(params![id])?;
                id
            }
            Some(_) => continue, // Skip if exists and not overwriting
            None => {
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
            }
        };

        for flag in raw.get_searchable_tokens() {
            insert_flag_stmt.execute(params![raw_db_id, flag])?;
        }

        let (search_names, search_descriptions) = extract_names_and_descriptions(raw);

        // Populate Names Table (for Exact/Partial ID lookup)
        for name in &search_names {
            insert_name_stmt.execute(params![raw_db_id, name])?;
        }

        // Populate FTS5 Index (for high-speed text search)
        insert_search_stmt.execute(params![
            raw_db_id,
            remove_dup_strings(search_names, true).join(" "),
            search_descriptions.join(" ")
        ])?;
    }

    Ok(())
}
