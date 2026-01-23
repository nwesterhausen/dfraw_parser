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
