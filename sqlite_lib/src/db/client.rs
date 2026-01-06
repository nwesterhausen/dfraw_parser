use dfraw_parser::ParseResult;
use rusqlite::{Connection, Result};
use tracing::{debug, info, warn};

use crate::SearchResults;
use crate::db::client_options::ClientOptions;
use crate::db::migrate::{apply_migrations, migrate_down};
use crate::db::migrations::LATEST_SCHEMA_VERSION;
use crate::db::queries;
use crate::db::search_query::SearchQuery;
use crate::db::util::get_current_schema_version;

/// A client for interacting with a database to contain details about parsed raws.
#[derive(Debug)]
pub struct DbClient {
    conn: Connection,
    options: ClientOptions,
}

impl DbClient {
    /// Opens a connection to the database and initializes it if it doesn't exist.
    ///
    /// Path is relative to directory the application is run in.
    ///
    /// # Errors
    ///
    /// - Issue creating/opening database
    /// - Issue performing migrations
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

    /// Uses the provided `SearchQuery` to return the JSON of all matching raws defined in the database.
    ///
    /// # Errors
    ///
    /// - On database error
    /// # Returns
    ///
    /// The `SearchResults` with the results as the JSON strings as byte arrays.
    pub fn search_raws(&self, query: &SearchQuery) -> Result<SearchResults<Vec<u8>>> {
        queries::search_raws(&self.conn, query)
    }

    /// Insert the `ParseResult` returned from `[dfraw_parser::parse]` into the database.
    ///
    /// # Errors
    ///
    /// - Database errors
    /// - Issues working with downcasting raws to obtain data to insert
    pub fn insert_parse_results(&mut self, parse_results: ParseResult) -> Result<()> {
        queries::insert_parse_results(&mut self.conn, &self.options, parse_results)
    }
}
