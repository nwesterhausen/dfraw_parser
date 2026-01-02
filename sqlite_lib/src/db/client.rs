use dfraw_parser::InfoFile;
use dfraw_parser::traits::RawObject;
use rusqlite::{Connection, Result, Transaction, params};
use std::fmt::Write as _;
use tracing::{debug, error, info, warn};

use crate::db::client_options::ClientOptions;
use crate::db::migrate::{apply_migrations, migrate_down};
use crate::db::migrations::LATEST_SCHEMA_VERSION;
use crate::db::search_query::SearchQuery;
use crate::db::util::get_current_schema_version;

/// A client for interacting with the database
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
    pub fn insert_module_data(
        &mut self,
        info: &InfoFile,
        raws: &[Box<dyn RawObject>],
    ) -> Result<()> {
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

    /// Combined search query logic to find raws by type, identifier, or flags.
    ///
    /// # Errors
    /// - database error
    pub fn search_raws(&self, query: SearchQuery) -> Result<Vec<Vec<u8>>> {
        let mut sql = String::from("SELECT r.data_blob FROM raw_definitions r ");
        let mut conditions = Vec::new();
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

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

        if let Some(name) = query.name_query {
            conditions.push(format!("r.identifier LIKE ?{}", params_vec.len() + 1));
            params_vec.push(Box::new(format!("%{name}%")));
        }

        if let Some(type_name) = query.raw_type_name {
            conditions.push(format!(
                "r.raw_type_id = (SELECT id FROM raw_types WHERE name = ?{})",
                params_vec.len() + 1
            ));
            params_vec.push(Box::new(type_name));
        }

        if !conditions.is_empty() {
            sql.push_str(" AND ");
            sql.push_str(&conditions.join(" AND "));
        }

        let params_ref: Vec<&dyn rusqlite::ToSql> =
            params_vec.iter().map(std::convert::AsRef::as_ref).collect();
        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt.query_map(&params_ref[..], |row| row.get(0))?;

        let mut results = Vec::new();
        for res in rows {
            results.push(res?);
        }
        Ok(results)
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
            info.get_location() as i32,
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

    if let Some(ids) = info.get_requires_ids() {
        for id in ids {
            dep_stmt.execute(params![module_db_id, id, 1])?;
        }
    }
    if let Some(ids) = info.get_conflicts_with_ids() {
        for id in ids {
            dep_stmt.execute(params![module_db_id, id, 2])?;
        }
    }
    if let Some(ids) = info.get_requires_ids_before() {
        for id in ids {
            dep_stmt.execute(params![module_db_id, id, 3])?;
        }
    }
    if let Some(ids) = info.get_requires_ids_after() {
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

    let mut update_raw_stmt =
        tx.prepare_cached("UPDATE raw_definitions SET data_blob = jsonb(?1) WHERE id = ?2")?;

    // let mut insert_flag_stmt =
    //    tx.prepare_cached("INSERT INTO common_raw_flags (raw_id, token_name) VALUES (?1, ?2)")?;

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

        let _raw_db_id = match existing_raw_id {
            Some(id) if overwrite_raws => {
                update_raw_stmt.execute(params![json_payload, id])?;
                clear_flags_stmt.execute(params![id])?;
                id
            }
            Some(_) => continue, // Skip if exists and not overwriting
            None => {
                insert_raw_stmt.execute(params![
                    raw.get_type().to_string(),
                    raw.get_identifier(),
                    module_db_id,
                    json_payload
                ])?;
                tx.last_insert_rowid()
            }
        };

        // Todo: add searchable tokens (flag tokens) to raw object trait
        // for flag in raw.get_searchable_tokens() {
        //     insert_flag_stmt.execute(params![raw_db_id, flag])?;
        // }
    }

    Ok(())
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
