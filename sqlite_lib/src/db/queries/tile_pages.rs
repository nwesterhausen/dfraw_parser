use rusqlite::{Connection, Result, params};

use crate::models::TilePageData;

use super::super::rusqlite_extensions::OptionalResultExtension;

/// Get a tile page by its id
///
/// # Errors
///
/// - database error
/// - no tile page with given `id`
pub fn get_tile_page_by_id(conn: &Connection, id: i64) -> Result<TilePageData> {
    const GET_TILE_PAGE_BY_ID: &str = r"
    SELECT
        id, raw_id, identifier, file_path, tile_width, tile_height, page_width, page_height
    FROM tile_pages
    WHERE id = ?1;
    ";

    conn.query_row(GET_TILE_PAGE_BY_ID, params![id], |row| {
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
    const GET_TILE_PAGE_BY_LINKED_RAW_ID: &str = r"
    SELECT
        id, raw_id, identifier, file_path, tile_width, tile_height, page_width, page_height
    FROM tile_pages
    WHERE raw_id = ?1;
    ";

    conn.query_row(GET_TILE_PAGE_BY_LINKED_RAW_ID, params![raw_id], |row| {
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
    const GET_TILE_PAGE_BY_IDENTIFIER: &str = r"
    SELECT
        id, raw_id, identifier, file_path, tile_width, tile_height, page_width, page_height
    FROM tile_pages
    WHERE identifier = ?1;
    ";

    conn.query_row(GET_TILE_PAGE_BY_IDENTIFIER, params![identifier], |row| {
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
