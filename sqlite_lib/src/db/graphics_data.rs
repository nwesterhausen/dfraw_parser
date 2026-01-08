/// A simplified struct for tile page data
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct TilePageData {
    /// database id for this page
    pub id: i64,
    /// linked id in database for the raw data of this tile page
    pub raw_id: i64,
    /// identifier of the tile page
    pub identifier: String,
    /// file path to tile page
    pub file_path: String,
    /// width of tiles
    pub tile_width: u32,
    /// height of tiles
    pub tile_height: u32,
    /// width of page
    pub page_width: u32,
    /// height of page
    pub page_height: u32,
}

/// A simplified struct for sprite graphic data
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SpriteGraphicData {
    /// database id for this sprite graphic
    pub id: i64,
    /// linked raw id (of graphics raw) this belongs to
    pub raw_id: i64,
    /// identifier of tile page sprite is on
    pub tile_page_identifier: String,
    /// sprite offset x1
    pub offset_x: i64,
    /// sprite offset y1
    pub offset_y: i64,
    /// for large sprites, offset x2
    pub offset_x_2: Option<i64>,
    /// for large sprites, offset y2
    pub offset_y_2: Option<i64>,
    /// primary condition for the sprite
    pub primary_condition: String,
    /// secondary condition for the sprite
    pub secondary_condition: String,
    /// the identifier of the thing this sprite displays
    pub target_identifier: String,
}
