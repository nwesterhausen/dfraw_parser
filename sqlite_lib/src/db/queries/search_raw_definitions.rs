use dfraw_parser::{
    metadata::RawModuleLocation,
    tokens::ObjectType,
    traits::{RawObject, RawToken},
};
use itertools::Itertools as _;
use rusqlite::{Connection, Result};
use std::fmt::Write as _;
use tracing::info;

use crate::{
    ResultWithId, SearchQuery, SearchResults,
    db::{compression::DbCodec, metadata_markers::FavoriteRaws, queries},
};

/// Uses the provided `SearchQuery` to return the JSON of all matching raws defined in the database.
///
/// # Errors
///
/// - On database error
///
/// # Returns
///
/// The `SearchResults` with the results as [`Box<dyn RawObject>`]
pub fn search_raws(
    conn: &Connection,
    codec: &DbCodec,
    query: &SearchQuery,
) -> Result<SearchResults<Box<dyn RawObject>>> {
    let mut sql = String::from("FROM raw_definitions r ");
    let mut conditions = Vec::new();
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    let is_full_text_search = query.is_full_text_search();

    // Full-Text Search Join (Names & Descriptions)
    if is_full_text_search && let Some(search_text) = query.search_string.as_ref() {
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

    // Numeric Token Joins

    add_numeric_filter(query, &mut sql, &mut conditions, &mut params_vec);

    // Module Join (if module info needed)
    if !query.locations.is_empty() || !query.in_modules.is_empty() {
        sql.push_str("JOIN modules m ON r.module_id = m.id ");
    }

    // A default condition that's always true to simplify adding an unknown amount of other conditions
    sql.push_str(" WHERE 1=1 ");

    if !query.in_modules.is_empty() {
        for module_obj_id in &query.in_modules {
            conditions.push(format!("m.object_id = ?{}", params_vec.len() + 1));
            params_vec.push(Box::new(module_obj_id.as_bytes().to_vec()));
        }
    }

    if query.favorites_only {
        let favorite_raw_list =
            (queries::get_typed_metadata::<FavoriteRaws>(conn)?).unwrap_or_else(Vec::new);

        add_favorite_raw_restriction(query, &mut conditions, &favorite_raw_list);
    }

    // Identifier Filter
    add_identifier_filter(query, &mut conditions, &mut params_vec);

    add_type_filter(query, &mut conditions, &mut params_vec);

    // Location Filter
    add_location_filter(query, &mut conditions, &mut params_vec);

    // Append conditions to the SQL query using AND
    if !conditions.is_empty() {
        sql.push_str(" AND ");
        sql.push_str(&conditions.join(" AND "));
    }

    // We have to find total amount to provide pagination.
    let total_count: u32 = {
        let count_sql = format!("SELECT COUNT(DISTINCT r.id) {sql}");
        let params_ref: Vec<&dyn rusqlite::ToSql> =
            params_vec.iter().map(std::convert::AsRef::as_ref).collect();

        conn.query_row(&count_sql, &params_ref[..], |row| row.get(0))?
    };

    // Now we can set up our actual results
    let mut results_sql = format!("SELECT r.id,r.data_blob {sql}");

    // Ensure we use BM25 ranking if searching text
    if is_full_text_search {
        // Sorts by matching on text, best results at the top
        // More info: https://sqlite.org/fts5.html#the_bm25_function
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
    final_params.push(Box::new(query.offset()));

    // Prepare parementers
    let params_ref: Vec<&dyn rusqlite::ToSql> = final_params
        .iter()
        .map(std::convert::AsRef::as_ref)
        .collect();
    let mut stmt = conn.prepare(&results_sql)?;

    // Run query
    let rows = stmt.query_map(&params_ref[..], |row| {
        let id: i64 = row.get(0)?;
        let binary_blob: Vec<u8> = row.get(1)?; // Get as blob
        let raw: Box<dyn RawObject> = codec
            .decompress_record(&binary_blob[..])
            .inspect_err(|e| {
                tracing::error!("search_raws: deserialization failed for id:{id}: {e}");
            })
            .map_err(|_| rusqlite::Error::InvalidQuery)?;
        Ok(ResultWithId { id, data: raw })
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
        (total_count / query.limit) + 1
    );

    Ok(SearchResults {
        results,
        total_count,
    })
}

/// Internal function to add a LIKE clause to `conditions` to match the `query.identifier_query` string
fn add_identifier_filter(
    query: &SearchQuery,
    conditions: &mut Vec<String>,
    params_vec: &mut Vec<Box<dyn rusqlite::ToSql + 'static>>,
) {
    if let Some(ident) = query.identifier_query.as_ref() {
        conditions.push(format!("r.identifier LIKE ?{}", params_vec.len() + 1));
        params_vec.push(Box::new(format!("%{ident}%")));
    }
}

/// Internal function to add a restriction for the raws to be part of the favorites to be returned.
fn add_favorite_raw_restriction(
    query: &SearchQuery,
    conditions: &mut Vec<String>,
    favorites: &[i64],
) {
    if !query.favorites_only {
        return;
    }

    // If we *are* supposed to query for only favorites, but there are none, return nothing
    // (with 1=0 as a restriction on the query, no results will be returned)
    if favorites.is_empty() {
        conditions.push(String::from("1=0"));
        return;
    }

    conditions.push(format!("r.id IN ({})", favorites.iter().format(",")));
}

/// Internal function to add the `RawModuleLocation` filter into `params_vec` and `conditions`
///
/// Will return early if `query.locations` is empty (no locations to filter on)
///
/// 1. Creates parameter placeholders based on which param number we are on, for each location to filter for
/// 2. Appends an IN clause to conditions (which is joined with AND) to let the location filter function on OR
/// 3. Pushes the actual locations to filter on into the `params_vec` for final prepare at end of `search_raws`
fn add_location_filter(
    query: &SearchQuery,
    conditions: &mut Vec<String>,
    params_vec: &mut Vec<Box<dyn rusqlite::ToSql + 'static>>,
) {
    if query.locations.is_empty() {
        return;
    }

    // Placeholders
    let start_idx = params_vec.len() + 1;
    let location_placeholders: Vec<String> = (0..query.locations.len())
        .map(|i| format!("?{}", start_idx + i))
        .collect();

    // Use an IN clause for OR
    conditions.push(format!(
        "m.module_location_id IN (SELECT id FROM module_locations WHERE name IN ({}))",
        location_placeholders.join(", ")
    ));

    // Register the locations for insertion
    for l in &query.locations {
        // Map enum variants to exact DB strings
        let db_name = match l {
            RawModuleLocation::Vanilla => "Vanilla",
            RawModuleLocation::WorkshopMods => "Workshop Mods",
            RawModuleLocation::InstalledMods => "Installed Mods",
            _ => "Unknown",
        };
        params_vec.push(Box::new(db_name.to_string()));
    }
}

/// Internal function to add the `ObjectType` filter into `params_vec` and `conditions`
///
/// Will return early if `query.raw_types` is empty (no types to filter on)
///
/// 1. Creates parameter placeholders based on which param number we are on, for each object type to filter for
/// 2. Appends an IN clause to conditions (which is joined with AND) to let the object type filter function on OR
/// 3. Pushes the actual object types to filter on into the `params_vec` for final prepare at end of `search_raws`
fn add_type_filter(
    query: &SearchQuery,
    conditions: &mut Vec<String>,
    params_vec: &mut Vec<Box<dyn rusqlite::ToSql + 'static>>,
) {
    if query.raw_types.is_empty() {
        return;
    }

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
        // Object types stored in database by "key", i.e. all caps: CREATURE, PLANT, etc
        params_vec.push(Box::new(ObjectType::get_key(t)));
    }
}

fn add_numeric_filter(
    query: &SearchQuery,
    sql: &mut String,
    conditions: &mut Vec<String>,
    params_vec: &mut Vec<Box<dyn rusqlite::ToSql + 'static>>,
) {
    for (i, filter) in query.numeric_filters.iter().enumerate() {
        let alias = format!("nf{i}");
        let _ = write!(
            sql,
            "JOIN common_raw_flags_with_numeric_value {alias} on r.id = {alias}.raw_id "
        );

        filter.add_sql_to_params(&alias, conditions, params_vec);
    }
}
