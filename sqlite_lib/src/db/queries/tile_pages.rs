use rusqlite::{Connection, Result, params};

use crate::models::TilePageData;

use super::super::rusqlite_extensions::OptionalResultExtension;

const TILE_PAGE_BY_ID_QUERY: &str = r"
SELECT
    id, raw_id, identifier, file_path, tile_width, tile_height, page_width, page_height
FROM tile_pages
WHERE id = ?1;
";

const TILE_PAGE_BY_IDENTIFIER_QUERY: &str = r"
SELECT
    id, raw_id, identifier, file_path, tile_width, tile_height, page_width, page_height
FROM tile_pages
WHERE identifier = ?1;
";

const TILE_PAGE_BY_RAW_ID_QUERY: &str = r"
SELECT
    id, raw_id, identifier, file_path, tile_width, tile_height, page_width, page_height
FROM tile_pages
WHERE raw_id = ?1;
";

/// Get a tile page by its id
///
/// # Errors
///
/// - database error
/// - no tile page with given `id`
pub fn get_tile_page_by_id(conn: &Connection, id: i64) -> Result<TilePageData> {
    conn.query_row(TILE_PAGE_BY_ID_QUERY, params![id], |row| {
        Ok(TilePageData {
            id: row.get(0)?,
            raw_id: row.get(1)?,
            identifier: row.get(2)?,
            file_path: row.get(3)?,
            tile_width: row.get(4)?,
            tile_height: row.get(5)?,
            page_width: row.get(6)?,
            page_height: row.get(7)?,
        })
    })
}

/// Get a tile page by its linked raw id
///
/// # Errors
///
/// - database error
/// - no tile page with given `raw id`
pub fn get_tile_page_by_raw_id(conn: &Connection, raw_id: i64) -> Result<TilePageData> {
    conn.query_row(TILE_PAGE_BY_RAW_ID_QUERY, params![raw_id], |row| {
        Ok(TilePageData {
            id: row.get(0)?,
            raw_id: row.get(1)?,
            identifier: row.get(2)?,
            file_path: row.get(3)?,
            tile_width: row.get(4)?,
            tile_height: row.get(5)?,
            page_width: row.get(6)?,
            page_height: row.get(7)?,
        })
    })
}

/// Get a tile page by its identifier
///
/// This returns only the top result
///
/// # Errors
///
/// - database error
/// - no tile page with given `id`
pub fn get_tile_page_by_identifier(
    conn: &Connection,
    identifier: &str,
) -> Result<Option<TilePageData>> {
    conn.query_row(TILE_PAGE_BY_IDENTIFIER_QUERY, params![identifier], |row| {
        Ok(TilePageData {
            id: row.get(0)?,
            raw_id: row.get(1)?,
            identifier: row.get(2)?,
            file_path: row.get(3)?,
            tile_width: row.get(4)?,
            tile_height: row.get(5)?,
            page_width: row.get(6)?,
            page_height: row.get(7)?,
        })
    })
    .optional()
}
