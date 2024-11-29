//! Steam data model
#![allow(clippy::module_name_repetitions)]
use diesel::prelude::*;

/// `SteamData` is the steam data that can be associated with a module
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::steam_data)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SteamData {
    /// Primary key
    pub id: i32,
    /// The steam file id. This is a `u64` but we store as text
    pub file_id: String,
    /// (Optional) title
    pub title: Option<String>,
    /// (Optional) description
    pub description: Option<String>,
    /// (Optional changelog)
    pub changelog: Option<String>,
}

/// Steam tag for attaching tags to the steam data, with or without values
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::steam_tag)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SteamTag {
    /// Primary key
    pub id: i32,
    /// The tag name
    pub name: String,
}

/// Steam tags which are attached to the steam data (with values)
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::steam_data_key_value_tag)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(SteamData, foreign_key = steam_data_id))]
#[diesel(belongs_to(SteamTag, foreign_key = steam_tag_id))]
pub struct SteamDataKeyValueTag {
    /// Primary key
    pub id: i32,
    /// The steam data ID
    pub steam_data_id: i32,
    /// The steam tag ID
    pub steam_tag_id: i32,
    /// The value of the tag
    pub value: String,
}

/// Steam tags which are attached to the steam data (no values)
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::steam_data_tag)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(SteamData, foreign_key = steam_data_id))]
#[diesel(belongs_to(SteamTag, foreign_key = steam_tag_id))]
pub struct SteamDataTag {
    /// The steam data ID
    pub steam_data_id: i32,
    /// The steam tag ID
    pub steam_tag_id: i32,
}

/// Additional string metadata to attach to the steam data
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::steam_data_metadata)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(SteamData, foreign_key = steam_data_id))]
pub struct SteamDataMetadata {
    /// Primary key
    pub id: i32,
    /// The steam data ID
    pub steam_data_id: i32,
    /// The metadata ID
    pub metadata: String,
}
