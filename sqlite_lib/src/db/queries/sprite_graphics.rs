use rusqlite::{Connection, Result, params};

use crate::SpriteGraphicData;

const GET_SPRITE_GRAPHIC_BY_ID: &str = r"
SELECT
    id, raw_id, tile_page_identifier, offset_x, offset_y, offset_x_2, offset_y_2, primary_condition, secondary_condition, target_identifier
FROM sprite_graphics
WHERE
    id = ?1;
";
const GET_SPRITE_GRAPHIC_BY_RAW_ID: &str = r"
SELECT
    id, raw_id, tile_page_identifier, offset_x, offset_y, offset_x_2, offset_y_2, primary_condition, secondary_condition, target_identifier
FROM sprite_graphics
WHERE
    raw_id = ?1;
";
const GET_SPRITE_GRAPHICS_FOR_TARGET_IDENTIFIER: &str = r"
SELECT
    id, raw_id, tile_page_identifier, offset_x, offset_y, offset_x_2, offset_y_2, primary_condition, secondary_condition, target_identifier
FROM sprite_graphics
WHERE
    target_identifier = ?1;
";

/// Get a sprite graphic by its id
///
/// # Errors
///
/// - database error
/// - no sprite graphic with given `id`
pub fn get_sprite_graphic_by_id(conn: &Connection, id: i64) -> Result<SpriteGraphicData> {
    conn.query_row(GET_SPRITE_GRAPHIC_BY_ID, params![id], |row| {
        Ok(SpriteGraphicData {
            id: row.get(0)?,
            raw_id: row.get(1)?,
            tile_page_identifier: row.get(2)?,
            offset_x: row.get(3)?,
            offset_y: row.get(4)?,
            offset_x_2: row.get(5)?,
            offset_y_2: row.get(6)?,
            primary_condition: row.get(7)?,
            secondary_condition: row.get(8)?,
            target_identifier: row.get(9)?,
        })
    })
}

/// Get a sprite graphic by its linked raw id
///
/// # Errors
///
/// - database error
/// - no sprite graphic with given `raw_id`
pub fn get_sprite_graphic_by_raw_id(conn: &Connection, raw_id: i64) -> Result<SpriteGraphicData> {
    conn.query_row(GET_SPRITE_GRAPHIC_BY_RAW_ID, params![raw_id], |row| {
        Ok(SpriteGraphicData {
            id: row.get(0)?,
            raw_id: row.get(1)?,
            tile_page_identifier: row.get(2)?,
            offset_x: row.get(3)?,
            offset_y: row.get(4)?,
            offset_x_2: row.get(5)?,
            offset_y_2: row.get(6)?,
            primary_condition: row.get(7)?,
            secondary_condition: row.get(8)?,
            target_identifier: row.get(9)?,
        })
    })
}

/// Get a sprite graphic by its linked raw id
///
/// # Errors
///
/// - database error
/// - no sprite graphic with given `raw_id`
pub fn get_sprite_graphics_for_target_identifier(
    conn: &Connection,
    target_identifier: &str,
) -> Result<Vec<SpriteGraphicData>> {
    let mut stmt = conn.prepare(GET_SPRITE_GRAPHICS_FOR_TARGET_IDENTIFIER)?;
    let mut rows = stmt.query(params![target_identifier])?;
    let mut sprites = Vec::new();

    while let Some(row) = rows.next()? {
        sprites.push(SpriteGraphicData {
            id: row.get(0)?,
            raw_id: row.get(1)?,
            tile_page_identifier: row.get(2)?,
            offset_x: row.get(3)?,
            offset_y: row.get(4)?,
            offset_x_2: row.get(5)?,
            offset_y_2: row.get(6)?,
            primary_condition: row.get(7)?,
            secondary_condition: row.get(8)?,
            target_identifier: row.get(9)?,
        });
    }

    Ok(sprites)
}
