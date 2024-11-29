//! This module contains the `InfoFile` struct, which represents the info text file at the root of every module.
#![allow(clippy::module_name_repetitions)]
use diesel::prelude::*;

/// `InfoFile` is the info text file at the root of every module
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::info_file)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Author, foreign_key = author_id))]
#[diesel(belongs_to(Location, foreign_key = location_id))]
#[diesel(belongs_to(Directory, foreign_key = parent_directory_id))]
#[diesel(belongs_to(SteamData, foreign_key = steam_data_id))]
pub struct InfoFile {
    /// The primary key of the info file
    pub id: i32,
    /// The module name
    pub name: String,
    /// The module description
    pub description: String,
    /// The author of the module
    pub author_id: i32,
    /// A special string representing this module
    pub object_id: String,
    /// The numeric version, used for compatibility checks
    pub numeric_version: i32,
    /// The display version, used for display purposes
    pub display_version: String,
    /// The earliest compatible numeric version, used for compatibility checks
    pub earliest_compat_numeric_version: i32,
    /// The earliest compatible display version, used for display purposes
    pub earliest_compat_display_version: String,
    /// The steam data ID, if this module is on the Steam Workshop
    pub steam_data_id: Option<i32>,
}

/// `InfoFileConflictsIds` describes the many-to-many relationship between `InfoFile` and `InfoFile` for conflicts.
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::info_file_conflicts_ids)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(InfoFile, foreign_key = info_file_id))]
#[diesel(belongs_to(InfoFile, foreign_key = conflicts_info_file_id))]
pub struct InfoFileConflictsIds {
    /// The ID of the info file
    pub info_file_id: i32,
    /// The ID of the conflicting info file
    pub conflicts_info_file_id: i32,
}

/// `InfoFileRequiresIds` describes the many-to-many relationship between `InfoFile` and `InfoFile` for requirements.
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::info_file_requires_ids)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(InfoFile, foreign_key = info_file_id))]
#[diesel(belongs_to(InfoFile, foreign_key = requires_info_file_id))]
pub struct InfoFileRequiresIds {
    /// The ID of the info file
    pub info_file_id: i32,
    /// The ID of the conflicting info file
    pub requires_info_file_id: i32,
}

/// This is used to ensure that the required mod is loaded before the requiring mod.
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::info_file_requires_before)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(InfoFile, foreign_key = info_file_id))]
#[diesel(belongs_to(InfoFile, foreign_key = requires_info_file_id))]
pub struct InfoFileRequiresBefore {
    /// The ID of the info file
    pub info_file_id: i32,
    /// The ID of the conflicting info file
    pub requires_info_file_id: i32,
}

/// This is used to ensure that the required mod is loaded after the requiring mod.
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::info_file_requires_after)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(InfoFile, foreign_key = info_file_id))]
#[diesel(belongs_to(InfoFile, foreign_key = requires_info_file_id))]
pub struct InfoFileRequiresAfter {
    /// The ID of the info file
    pub info_file_id: i32,
    /// The ID of the conflicting info file
    pub requires_info_file_id: i32,
}
