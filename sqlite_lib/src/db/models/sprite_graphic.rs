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
