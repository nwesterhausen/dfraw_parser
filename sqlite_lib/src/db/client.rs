use dfraw_parser::metadata::ObjectType;
use dfraw_parser::traits::RawObject;
use dfraw_parser::{Creature, InfoFile, ParseResult, Plant};
use rusqlite::{Connection, Result, Transaction, params};
use std::collections::HashMap;
use std::fmt::Write as _;
use tracing::{debug, error, info, warn};

use crate::db::client_options::ClientOptions;
use crate::db::migrate::{apply_migrations, migrate_down};
use crate::db::migrations::LATEST_SCHEMA_VERSION;
use crate::db::search_query::SearchQuery;
use crate::db::util::{get_current_schema_version, remove_dup_strings};

/// A client for interacting with the database
#[derive(Debug)]
pub struct DbClient {
    conn: Connection,
    options: ClientOptions,
}

impl DbClient {
    /// Opens a connection to the database and initializes it if it doesn't exist.
    ///
    /// # Errors
    /// - database errors
    pub fn init_db(path: &str, options: ClientOptions) -> Result<Self> {
        let conn = Connection::open(path)?;
        info!("Database connection opened.");
        debug!("Database: {path}");
        let mut current_schema_version: i32 = get_current_schema_version(&conn)?;

        if options.reset_database && current_schema_version != 0 {
            warn!("Asked to reset database, will empty database.");
            migrate_down(&conn, 0)?;
            current_schema_version = get_current_schema_version(&conn)?;
        }

        info!(
            "Current database schema: v{current_schema_version}, Target database schema: v{LATEST_SCHEMA_VERSION}"
        );

        if current_schema_version < LATEST_SCHEMA_VERSION {
            apply_migrations(&conn)?;
        }

        Ok(Self { conn, options })
    }

    /// High-performance insertion for a parsed module and its raws with duplicate checking.
    ///
    /// # Errors
    /// - database error
    /// - failure to parse existing json into raw object
    fn insert_module_data(&mut self, info: &InfoFile, raws: &[Box<dyn RawObject>]) -> Result<()> {
        let overwrite_raws = self.options.overwrite_raws;

        let tx = self.conn.transaction()?;

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

    /// Uses the `SearchQuery` to query the database to find matching `raw_id`'s
    ///
    /// # Errors
    /// - database error
    ///
    /// # Returns
    /// A tuple with
    /// - json `RawObject` array (from Box dyn `RawObject`)
    /// - total results
    pub fn search_raws(&self, query: &SearchQuery) -> Result<(Vec<Vec<u8>>, u32)> {
        let mut sql = String::from("FROM raw_definitions r ");
        let mut conditions = Vec::new();
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        // Full-Text Search Join (Names & Descriptions)
        if let Some(search_text) = query.search_string.as_ref() {
            sql.push_str("JOIN raw_search_index s ON r.id = s.raw_id ");
            conditions.push(format!("raw_search_index MATCH ?{}", params_vec.len() + 1));
            params_vec.push(Box::new(search_text.clone()));
        }

        // Flags Joins
        for (i, flag) in query.required_flags.iter().enumerate() {
            let alias = format!("f{i}");
            let _ = write!(
                sql,
                "JOIN common_raw_flags {alias} ON r.id = {alias}.raw_id "
            );
            conditions.push(format!("{alias}.token_name = ?{}", params_vec.len() + 1));
            params_vec.push(Box::new(flag.clone()));
        }

        sql.push_str(" WHERE 1=1 ");

        // Identifier Filter
        if let Some(ident) = query.identifier_query.as_ref() {
            conditions.push(format!("r.identifier LIKE ?{}", params_vec.len() + 1));
            params_vec.push(Box::new(format!("%{ident}%")));
        }

        // Type Filter
        if !query.raw_types.is_empty() {
            // Build a list of placeholders: ?N, ?N+1...
            let start_idx = params_vec.len() + 1;
            let type_placeholders: Vec<String> = (0..query.raw_types.len())
                .map(|i| format!("?{}", start_idx + i))
                .collect();

            // Use IN clause to treat multiple types as OR
            conditions.push(format!(
                "r.raw_type_id IN (SELECT id FROM raw_types WHERE name IN ({}))",
                type_placeholders.join(", ")
            ));

            // Put the types into the IN clause
            for t in &query.raw_types {
                params_vec.push(Box::new(ObjectType::get_key(t)));
            }
        }

        if !conditions.is_empty() {
            sql.push_str(" AND ");
            sql.push_str(&conditions.join(" AND "));
        }

        // We have to find total amount to provide pagination.
        let total_count: u32 = {
            let count_sql = format!("SELECT COUNT(DISTINCT r.id) {sql}");
            let params_ref: Vec<&dyn rusqlite::ToSql> =
                params_vec.iter().map(std::convert::AsRef::as_ref).collect();

            self.conn
                .query_row(&count_sql, &params_ref[..], |row| row.get(0))?
        };

        // Now we can set up our actual results
        let mut results_sql = format!("SELECT json(r.data_blob) {sql}");

        // Ensure we use BM25 ranking if searching text
        if query.search_string.is_some() {
            // Specifies weights for columns in raw_search_index
            // name: weight 5
            // description: weight 1
            results_sql.push_str(" ORDER BY bm25(raw_search_index, 5.0, 1.0)");
        } else {
            // Specify a default sorting to keep pagination consistent
            results_sql.push_str(" ORDER BY r.identifier ASC, r.id ASC");
        }

        // Apply Limit and Offset
        let current_param_idx = params_vec.len();
        let _ = write!(
            results_sql,
            " LIMIT ?{} OFFSET ?{}",
            current_param_idx + 1,
            current_param_idx + 2
        );

        let mut final_params = params_vec;
        final_params.push(Box::new(query.limit));
        let offset = (query.page.saturating_sub(1)) * query.limit;
        final_params.push(Box::new(offset));

        // Prepare parementers
        let params_ref: Vec<&dyn rusqlite::ToSql> = final_params
            .iter()
            .map(std::convert::AsRef::as_ref)
            .collect();
        let mut stmt = self.conn.prepare(&results_sql)?;

        let rows = stmt.query_map(&params_ref[..], |row| {
            let json_string: String = row.get(0)?; // Get as Text
            Ok(json_string.into_bytes()) // Convert to Vec<u8> for the return type
        })?;

        let mut results = Vec::new();
        let mut rows_count = 0;
        for res in rows {
            results.push(res?);
            rows_count += 1;
        }

        info!(
            "search_raws: {rows_count}/{total_count} results, page {} of {}",
            query.page,
            (total_count / query.limit) as u32 + 1
        );
        Ok((results, total_count))
    }

    /// insert `ParseResult` from the `dfraw_parser::parse` function
    ///
    /// # Errors
    /// - if database insertion error
    pub fn insert_parse_results(&mut self, parse_results: ParseResult) -> Result<()> {
        // group Raws by Module Identity
        // We use a composite key of (name, version, location_id) to match Raws to their InfoFiles.
        // This allows us to handle multi-module parsing (Vanilla + Mods) correctly.
        let mut module_map = HashMap::new();
        for raw in parse_results.raws {
            let meta = raw.get_metadata();
            let key = (
                String::from(meta.get_module_name()),
                String::from(meta.get_module_version()),
                i32::from(meta.get_location()),
            );
            module_map.entry(key).or_insert_with(Vec::new).push(raw);
        }
        // insert into Database
        // We iterate through the parsed info files and grab the raws associated with each.
        for info in &parse_results.info_files {
            let key = (
                info.get_name(),
                info.get_version(),
                i32::from(info.get_location()),
            );
            tracing::info!("Inserting raws for {key:?}");
            if let Some(module_raws) = module_map.get(&key) {
                self.insert_module_data(info, module_raws)?;
            }
        }
        info!("Insertion Complete");
        Ok(())
    }
}

fn insert_module_record(tx: &Transaction, info: &InfoFile) -> Result<i64> {
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

fn insert_module_dependencies(tx: &Transaction, module_db_id: i64, info: &InfoFile) -> Result<()> {
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

fn process_raw_insertions(
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
    // let mut update_raw_stmt =
    //     tx.prepare_cached("UPDATE raw_definitions SET data_blob = jsonb(?1) WHERE id = ?2")?;
    // let mut insert_flag_stmt =
    //     tx.prepare_cached("INSERT INTO common_raw_flags (raw_id, token_name) VALUES (?1, ?2)")?;
    // let mut clear_flags_stmt =
    //     tx.prepare_cached("DELETE FROM common_raw_flags WHERE raw_id = ?1")?;

    // Search Index Statements
    let mut insert_name_stmt =
        tx.prepare_cached("INSERT INTO raw_names (raw_id, name) VALUES (?1, ?2)")?;
    // let mut clear_names_stmt = tx.prepare_cached("DELETE FROM raw_names WHERE raw_id = ?1")?;
    let mut insert_search_stmt = tx.prepare_cached(
        "INSERT INTO raw_search_index (raw_id, names, description) VALUES (?1, ?2, ?3)",
    )?;
    // let mut delete_search_stmt =
    //     tx.prepare_cached("DELETE FROM raw_search_index WHERE raw_id = ?1")?;

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

#[allow(clippy::borrowed_box)]
fn extract_names_and_descriptions(raw: &Box<dyn RawObject>) -> (Vec<&str>, Vec<&str>) {
    // Metadata extraction for search index
    let mut search_names = Vec::<&str>::new();
    let mut search_descriptions = Vec::<&str>::new();

    match raw.get_type() {
        ObjectType::Creature => {
            if let Some(creature) = raw.as_any().downcast_ref::<Creature>() {
                search_names.clone_from(&creature.get_all_names());
                search_descriptions.clone_from(&creature.get_all_descriptions());
            }
        }
        ObjectType::Plant => {
            if let Some(plant) = raw.as_any().downcast_ref::<Plant>() {
                search_names.clone_from(&plant.get_all_names());
                search_descriptions.clone_from(plant.get_pref_strings().as_ref());
            }
        }
        _ => {}
    }

    (search_names, search_descriptions)
}

/// Simple extension trait for Rusqlite to handle Optional rows easily.
trait OptionalExtension<T> {
    fn optional(self) -> Result<Option<T>>;
}

impl<T> OptionalExtension<T> for Result<T> {
    fn optional(self) -> Result<Option<T>> {
        match self {
            Ok(val) => Ok(Some(val)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
