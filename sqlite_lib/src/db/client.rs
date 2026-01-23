use std::path::Path;

use chrono::{TimeDelta, prelude::*};
use dfraw_parser::metadata::{ParserOptions, RawModuleLocation};
use dfraw_parser::traits::RawObject;
use dfraw_parser::{ModuleInfo, ParseResult};
use rusqlite::{Connection, Result};
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::db::client_options::ClientOptions;
use crate::db::metadata_markers::{
    FavoriteRaws, LastRawsInsertion, LastRawsParsingOperation, PreferredSearchLimit,
    PreviousDwarfFortressGamePath, PreviousDwarfFortressUserPath, PreviousInsertionDuration,
    PreviousParseDuration, PreviousParserOptions, RecentSearchTerms, StoredSettings,
    UseSteamAutodetect,
};
use crate::db::migrate::{apply_migrations, migrate_down};
use crate::db::migrations::LATEST_SCHEMA_VERSION;
use crate::db::queries::{self, insert_module_and_data};
use crate::db::util::get_current_schema_version;
use crate::models::{SpriteGraphicData, TilePageData};
use crate::search_query::DEFAULT_SEARCH_LIMIT;
use crate::{SearchQuery, SearchResults};

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
    #[tracing::instrument(level = "debug", skip(self))]
    pub fn search_raws(&self, query: &SearchQuery) -> Result<SearchResults<Vec<u8>>> {
        self.add_recent_search_term(query.search_string.clone())?;
        queries::search_raws(&self.conn, query)
    }

    /// Insert the `ParseResult` returned from `[dfraw_parser::parse]` into the database.
    ///
    /// # Errors
    ///
    /// - Database errors
    /// - Issues working with downcasting raws to obtain data to insert
    pub fn insert_parse_results(&mut self, parse_results: &ParseResult) -> Result<()> {
        let start = Utc::now();

        for module in &parse_results.modules {
            info!("Inserting raws for {module}");
            insert_module_and_data(
                &mut self.conn,
                self.options.overwrite_raws,
                module,
                &parse_results.get_raws_for_module(module),
            )?;
        }

        let end = Utc::now();
        let insertion_duration = end - start;
        self.set_last_insertion_utc_datetime(&end)?;
        self.set_last_insertion_duration(&insertion_duration)?;
        Ok(())
    }

    /// Set the last used parser options
    ///
    /// If there is no saved options, returns None
    ///
    /// # Errors
    ///
    /// - database error
    /// - seialization error
    pub fn set_last_used_parser_options(&self, options: &ParserOptions) -> Result<()> {
        self.set_last_used_df_game_dir(
            options
                .locations
                .get_df_directory()
                .unwrap_or_default()
                .as_path(),
        )?;
        self.set_last_used_df_user_dir(
            options
                .locations
                .get_user_data_directory()
                .unwrap_or_default()
                .as_path(),
        )?;

        queries::set_typed_metadata::<PreviousParserOptions>(&self.conn, options)
    }

    /// Get the stored settings
    ///
    /// # Errors
    ///
    /// - database error
    pub fn get_stored_settings(&self) -> Result<String> {
        (queries::get_typed_metadata::<StoredSettings>(&self.conn)?)
            .map_or_else(|| Ok(String::new()), Ok)
    }

    /// Set the stored settings
    ///
    /// # Errors
    ///
    /// - database error
    pub fn set_stored_settings(&self, settings_json: &String) -> Result<()> {
        queries::set_typed_metadata::<StoredSettings>(&self.conn, settings_json)
    }

    /// Get the last used parser options
    ///
    /// # Errors
    ///
    /// - database error
    /// - deseialization error
    pub fn get_last_used_parser_options(&self) -> Result<Option<ParserOptions>> {
        queries::get_typed_metadata::<PreviousParserOptions>(&self.conn)
    }

    /// Set the last parse duration from a `chrono::TimeDelta`
    ///
    /// # Errors
    ///
    /// - database error
    /// - seialization error
    pub fn set_last_parse_duration(&self, duration: &TimeDelta) -> Result<()> {
        queries::set_typed_metadata::<PreviousParseDuration>(&self.conn, duration)
    }

    /// Get the last parse duration as a `chrono::TimeDelta`
    ///
    /// If there is no saved measurement, returns None
    ///
    /// # Errors
    ///
    /// - database error
    /// - deseialization error
    pub fn get_last_parse_duration(&self) -> Result<Option<TimeDelta>> {
        queries::get_typed_metadata::<PreviousParseDuration>(&self.conn)
    }

    /// Set the last raw files insertion duration from a `chrono::TimeDelta`
    ///
    /// # Errors
    ///
    /// - database error
    /// - seialization error
    pub fn set_last_insertion_duration(&self, duration: &TimeDelta) -> Result<()> {
        queries::set_typed_metadata::<PreviousInsertionDuration>(&self.conn, duration)
    }

    /// Get the last raw files insertion duration as a `chrono::TimeDelta`
    ///
    /// If there is no saved measurement, returns None
    ///
    /// # Errors
    ///
    /// - database error
    /// - deseialization error
    pub fn get_last_insertion_duration(&self) -> Result<Option<TimeDelta>> {
        queries::get_typed_metadata::<PreviousInsertionDuration>(&self.conn)
    }

    /// Add the last search term to the list of last 10 search terms
    ///
    /// # Errors
    ///
    /// - database error
    /// - de/serialization error
    pub fn add_recent_search_term(&self, search_term: Option<String>) -> Result<()> {
        let search_term = search_term.unwrap_or_default();
        if search_term.is_empty() {
            return Ok(());
        }

        let mut terms =
            queries::get_typed_metadata::<RecentSearchTerms>(&self.conn)?.unwrap_or_default();

        let mut replaced = false;
        for t in &mut terms {
            if search_term.starts_with(t.as_str()) {
                t.clone_from(&search_term);
                replaced = true;
                break;
            }
        }

        if !replaced &&
            // Check if exact term already exists to avoid duplicates
             !terms.contains(&search_term)
        {
            terms.insert(0, search_term);
        }

        // Maintain limit of 10
        terms.truncate(10);

        queries::set_typed_metadata::<RecentSearchTerms>(&self.conn, &terms)
    }

    /// Get up to the last 10 recent search terms
    ///
    /// # Errors
    ///
    /// - database error
    /// - deseialization error
    pub fn get_recent_search_terms(&self) -> Result<Vec<String>> {
        (queries::get_typed_metadata::<RecentSearchTerms>(&self.conn)?)
            .map_or_else(|| Ok(Vec::new()), Ok)
    }

    /// Set the user's preference for using the Steam-based autodetect of Dwarf Fortress
    ///
    /// # Errors
    ///
    /// - database error
    /// - serialization error
    pub fn set_use_steam_autodetect(&self, use_steam_autodetect: bool) -> Result<()> {
        queries::set_typed_metadata::<UseSteamAutodetect>(&self.conn, &use_steam_autodetect)
    }

    /// Get the user's preference for using the Steam-based autodetect of Dwarf Fortress
    ///
    /// # Errors
    ///
    /// - database error
    /// - deserialization error
    pub fn get_use_steam_autodetect(&self) -> Result<bool> {
        (queries::get_typed_metadata::<UseSteamAutodetect>(&self.conn)?)
            .map_or_else(|| Ok(false), Ok)
    }

    /// Set the last used Dwarf Fortress installation directory
    ///
    /// # Errors
    ///
    /// - database error
    /// - serialization error
    pub fn set_last_used_df_game_dir(&self, dwarf_fortress_directory: &Path) -> Result<()> {
        let path_str = dwarf_fortress_directory.to_string_lossy().to_string();
        queries::set_typed_metadata::<PreviousDwarfFortressGamePath>(&self.conn, &path_str)
    }

    /// Get the last used Dwarf Fortress installation directory
    ///
    /// # Errors
    ///
    /// - database error
    /// - deserialization error
    pub fn get_last_used_df_game_dir(&self) -> Result<String> {
        (queries::get_typed_metadata::<PreviousDwarfFortressGamePath>(&self.conn)?)
            .map_or_else(|| Ok(String::new()), Ok)
    }

    /// Set the last used Dwarf Fortress user directory
    ///
    /// # Errors
    ///
    /// - database error
    /// - serialization error
    pub fn set_last_used_df_user_dir(&self, user_data_dir: &Path) -> Result<()> {
        let path_str = user_data_dir.to_string_lossy().to_string();
        queries::set_typed_metadata::<PreviousDwarfFortressUserPath>(&self.conn, &path_str)
    }

    /// Get the last used Dwarf Fortress user directory
    ///
    /// # Errors
    ///
    /// - database error
    /// - deserialization error
    pub fn get_last_used_df_user_dir(&self) -> Result<String> {
        (queries::get_typed_metadata::<PreviousDwarfFortressUserPath>(&self.conn)?)
            .map_or_else(|| Ok(String::new()), Ok)
    }

    /// Adds a raw ID to the user's favorites list.
    ///
    /// # Errors
    ///
    /// - database error
    /// - de/serialization error
    pub fn add_favorite_raw(&self, raw_id: i64) -> Result<()> {
        let mut favorites =
            queries::get_typed_metadata::<FavoriteRaws>(&self.conn)?.unwrap_or_default();

        if !favorites.contains(&raw_id) {
            favorites.push(raw_id);
            queries::set_typed_metadata::<FavoriteRaws>(&self.conn, &favorites)?;
        }

        Ok(())
    }

    /// Removes a raw ID from the user's favorites list.
    ///
    /// # Errors
    ///
    /// - database error
    /// - deserialization error
    pub fn remove_favorite_raw(&self, raw_id: i64) -> Result<()> {
        let favorites_opt = queries::get_typed_metadata::<FavoriteRaws>(&self.conn)?;

        if let Some(mut favorites) = favorites_opt {
            let initial_len = favorites.len();
            favorites.retain(|&id| id != raw_id);

            if favorites.len() != initial_len {
                queries::set_typed_metadata::<FavoriteRaws>(&self.conn, &favorites)?;
            }
        }

        Ok(())
    }

    /// Get the user's favorited raws as ids (to be used with the database for retrival/matching).
    ///
    /// # Errors
    ///
    /// - database error
    /// - deserialization error
    pub fn get_favorite_raws(&self) -> Result<Vec<i64>> {
        (queries::get_typed_metadata::<FavoriteRaws>(&self.conn)?)
            .map_or_else(|| Ok(Vec::new()), Ok)
    }

    /// Set the user's preference for number of results per page
    ///
    /// # Errors
    ///
    /// - database error
    /// - serialization error
    pub fn set_preferred_search_limit(&self, search_limit: u32) -> Result<()> {
        queries::set_typed_metadata::<PreferredSearchLimit>(&self.conn, &search_limit)
    }

    /// Get the user's preference for number of results per page
    ///
    /// Returns the default value if not previously updated
    ///
    /// # Errors
    ///
    /// - database error
    /// - deserialization error
    pub fn get_preferred_search_limit(&self) -> Result<u32> {
        (queries::get_typed_metadata::<PreferredSearchLimit>(&self.conn)?)
            .map_or_else(|| Ok(DEFAULT_SEARCH_LIMIT), Ok)
    }

    /// Set the date of the last insertion. Expects RFC 3339 (ISO 8601) formatted string.
    ///
    /// # Errors
    ///
    /// - database error
    /// - serialization error
    pub fn set_last_insertion_date(&self, insertion_date: &str) -> Result<()> {
        queries::set_typed_metadata::<LastRawsInsertion>(&self.conn, &insertion_date.to_string())
    }
    /// Set the date of the last insertion. Expects a `DateTime` in UTC timezone.
    ///
    /// # Errors
    ///
    /// - database error
    /// - serialization error
    pub fn set_last_insertion_utc_datetime(&self, utc_date: &DateTime<Utc>) -> Result<()> {
        let str_date = utc_date.to_rfc3339();
        queries::set_typed_metadata::<LastRawsInsertion>(&self.conn, &str_date)
    }

    /// Set the date of the last insertion as `Utc::now()`.
    ///
    /// # Errors
    ///
    /// - database error
    /// - serialization error
    pub fn set_last_insertion_as_utc_now(&self) -> Result<()> {
        let str_date = Utc::now().to_rfc3339();
        queries::set_typed_metadata::<LastRawsInsertion>(&self.conn, &str_date)
    }

    /// Get the date of the last insertion.
    ///
    /// # Errors
    ///
    /// - database error
    /// - serialization error
    pub fn get_last_insertion_date(&self) -> Result<Option<String>> {
        queries::get_typed_metadata::<LastRawsInsertion>(&self.conn)
    }

    /// Set the date of the last parse operation. Expects a `DateTime` in UTC timezone.
    ///
    /// # Errors
    ///
    /// - database error
    /// - serialization error
    pub fn set_last_parse_operation_utc_datetime(&self, utc_date: &DateTime<Utc>) -> Result<()> {
        let str_date = utc_date.to_rfc3339();
        queries::set_typed_metadata::<LastRawsParsingOperation>(&self.conn, &str_date)
    }

    /// Set the date of the last parse operation as `Utc::now()`.
    ///
    /// # Errors
    ///
    /// - database error
    /// - serialization error
    pub fn set_last_parse_operation_as_utc_now(&self) -> Result<()> {
        let str_date = Utc::now().to_rfc3339();
        queries::set_typed_metadata::<LastRawsParsingOperation>(&self.conn, &str_date)
    }

    /// Set the date of the last parse operation. Expects RFC 3339 (ISO 8601) formatted string.
    ///
    /// # Errors
    ///
    /// - database error
    /// - serialization error
    pub fn set_last_parse_operation_date(&self, parse_date: &str) -> Result<()> {
        queries::set_typed_metadata::<LastRawsParsingOperation>(&self.conn, &parse_date.to_string())
    }

    /// Get the date of the last insertion.
    ///
    /// # Errors
    ///
    /// - database error
    /// - serialization error
    pub fn get_last_parse_operation_date(&self) -> Result<Option<String>> {
        queries::get_typed_metadata::<LastRawsParsingOperation>(&self.conn)
    }

    /// Retrieves a raw object by its database ID.
    ///
    /// # Errors
    ///
    /// - database error
    pub fn get_raw(&self, id: i64) -> Result<Box<dyn RawObject>> {
        queries::get_raw_definition(&self.conn, id)
    }

    /// Creates a new raw definition and populates all associated search and graphics tables.
    ///
    /// # Errors
    ///
    /// - database error
    #[allow(clippy::borrowed_box)]
    pub fn create_raw(&self, raw: &Box<dyn RawObject>) -> Result<i64> {
        queries::create_raw_definition(&self.conn, raw)
    }

    /// Updates or creates a raw definition based on its identifier and module identity.
    ///
    /// # Errors
    ///
    /// - database error
    #[allow(clippy::borrowed_box)]
    pub fn upsert_raw(&self, raw: &Box<dyn RawObject>) -> Result<i64> {
        queries::upsert_raw_definition(&self.conn, raw)
    }

    /// Updates the data blob and associated tables for an existing raw definition.
    ///
    /// # Errors
    ///
    /// - database error
    #[allow(clippy::borrowed_box)]
    pub fn update_raw(&self, id: i64, raw: &Box<dyn RawObject>) -> Result<()> {
        queries::update_raw_definition(&self.conn, id, raw)
    }

    /// Deletes a raw definition.
    ///
    /// # Errors
    ///
    /// - database error
    pub fn delete_raw(&self, id: i64) -> Result<()> {
        queries::delete_raw_definition(&self.conn, id)
    }

    /// Retrieves the top result for a module id matching the data in the raw's metadata.
    ///
    /// # Errors
    ///
    /// - database error
    #[allow(clippy::borrowed_box)]
    pub fn get_module_id_from_raw(&self, raw: &Box<dyn RawObject>) -> Result<i64> {
        queries::get_module_id_from_raw(&self.conn, raw)
    }

    /// Creates a new raw defintion with a link to a specific module
    ///
    /// # Errors
    ///
    /// - database error
    #[allow(clippy::borrowed_box)]
    pub fn create_raw_definition_with_module(
        &self,
        module_id: i64,
        raw: &Box<dyn RawObject>,
    ) -> Result<i64> {
        queries::create_raw_definition_with_module(&self.conn, module_id, raw)
    }

    /// Returns true if the raw exists in the database.
    ///
    /// Searches for a match based on the raw identifier and its metadata: location,
    /// module name and module version.
    ///
    /// # Errors
    ///
    /// - database error
    #[allow(clippy::borrowed_box)]
    pub fn exists_raw(&self, raw: &Box<dyn RawObject>) -> Result<bool> {
        queries::exists_raw(&self.conn, raw)
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
    pub fn try_get_raw_id(&self, raw: &Box<dyn RawObject>) -> Result<Option<i64>> {
        queries::try_get_raw_id(&self.conn, raw)
    }

    /// Get a tile page by its id
    ///
    /// # Errors
    ///
    /// - database error
    /// - no tile page with given `id`
    pub fn get_tile_page_by_id(&self, id: i64) -> Result<TilePageData> {
        queries::get_tile_page_by_id(&self.conn, id)
    }

    /// Get a tile page by its identifier
    ///
    /// This returns only the top result. If no results, returns None
    ///
    /// # Errors
    ///
    /// - database error
    pub fn get_tile_page_by_identifier(&self, identifier: &str) -> Result<Option<TilePageData>> {
        queries::get_tile_page_by_identifier(&self.conn, identifier)
    }

    /// Get a tile page by its linked raw id
    ///
    /// # Errors
    ///
    /// - database error
    /// - no tile page with given `raw id`
    pub fn get_tile_page_by_raw_id(&self, raw_id: i64) -> Result<TilePageData> {
        queries::get_tile_page_by_raw_id(&self.conn, raw_id)
    }

    /// Get a sprite graphic by its target identifier
    ///
    /// # Errors
    ///
    /// - database error
    /// - no sprite graphic with given `target`
    pub fn get_sprite_graphics_for_target_identifier(
        &self,
        target_identifier: &str,
    ) -> Result<Vec<SpriteGraphicData>> {
        queries::get_sprite_graphics_for_target_identifier(&self.conn, target_identifier)
    }

    /// Get a sprite graphic by its target identifier and include any caste-specific matches
    ///
    /// # Errors
    ///
    /// - database error
    /// - no sprite graphic with given `target`
    pub fn get_sprite_graphics_for_target_identifier_and_any_castes(
        &self,
        target_identifier: &str,
    ) -> Result<Vec<SpriteGraphicData>> {
        queries::get_sprite_graphics_for_target_identifier_and_any_castes(
            &self.conn,
            target_identifier,
        )
    }

    /// Get a sprite graphic by its target identifier and caste
    ///
    /// # Errors
    ///
    /// - database error
    /// - no sprite graphic with given `target`
    pub fn get_sprite_graphics_for_target_identifier_and_caste(
        &self,
        target_identifier: &str,
        target_caste: &str,
    ) -> Result<Vec<SpriteGraphicData>> {
        queries::get_sprite_graphics_for_target_identifier_and_caste(
            &self.conn,
            target_identifier,
            target_caste,
        )
    }

    /// Get a sprite graphic by its linked raw id
    ///
    /// # Errors
    ///
    /// - database error
    /// - no sprite graphic with given `raw_id`
    pub fn get_sprite_graphic_by_raw_id(&self, raw_id: i64) -> Result<SpriteGraphicData> {
        queries::get_sprite_graphic_by_raw_id(&self.conn, raw_id)
    }

    /// Get a sprite graphic by its id
    ///
    /// # Errors
    ///
    /// - database error
    /// - no sprite graphic with given `id`
    pub fn get_sprite_graphic_by_id(&self, id: i64) -> Result<SpriteGraphicData> {
        queries::get_sprite_graphic_by_id(&self.conn, id)
    }

    /// Try to get a module id by specific identifiers
    ///
    /// # Errors
    ///
    /// - database error
    pub fn try_get_module_id_by_identifiers(
        &self,
        identifier: &str,
        numeric_version: i64,
        location: RawModuleLocation,
    ) -> Result<Option<i64>> {
        queries::try_get_module_id_by_identifiers(&self.conn, identifier, numeric_version, location)
    }

    /// Returns true if the module exists, searching by specific identifiers.
    ///
    /// # Errors
    ///
    /// - database error
    pub fn exists_module_by_identifiers(
        &self,
        identifier: &str,
        numeric_version: i64,
        location: RawModuleLocation,
    ) -> Result<bool> {
        queries::exists_module_by_identifiers(&self.conn, identifier, numeric_version, location)
    }

    /// Try get module by its `object_id`
    ///
    /// # Errors
    ///
    /// - database errors
    pub fn try_get_module_id_by_object_id(&self, object_id: Uuid) -> Result<Option<i64>> {
        queries::try_get_module_id_by_object_id(&self.conn, object_id)
    }

    /// Check if a module exists by its `object_id`
    ///
    /// # Errors
    ///
    /// - database error
    pub fn exists_module_by_object_id(&self, object_id: Uuid) -> Result<bool> {
        queries::exists_module_by_object_id(&self.conn, object_id)
    }

    /// Try to get the db id for a module (uses its `object_id`)
    ///
    /// # Errors
    ///
    /// - database errors
    pub fn try_get_module_id(&self, module: &ModuleInfo) -> Result<Option<i64>> {
        queries::try_get_module_id(&self.conn, module)
    }

    /// Check if a module exists (uses its `object_id`)
    ///
    /// # Errors
    ///
    /// - datbase errors
    pub fn exists_module(&self, module: &ModuleInfo) -> Result<bool> {
        queries::exists_module(&self.conn, module)
    }

    /// Insert a module with its supporting data, returning its id in the database.
    ///
    /// This inserts the module along with its dependency chain and steam tag data.
    ///
    /// # Errors
    ///
    /// - database errors
    pub fn insert_module(&mut self, module: &ModuleInfo) -> Result<i64> {
        queries::insert_module(&mut self.conn, self.options.overwrite_raws, module)
    }
}
