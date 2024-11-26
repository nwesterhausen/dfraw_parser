//! This module contains the `InfoFile` struct, which represents the info text file at the root of every module.
use diesel::prelude::*;

/// `InfoFile` is the info text file at the root of every module
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::info_file)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Author, foreign_key = author_id))]
#[diesel(belongs_to(Location, foreign_key = location_id))]
#[diesel(belongs_to(Directory, foreign_key = parent_directory_id))]
pub struct InfoFile {
    /// The primary key of the info file
    pub id: i32,
    /// The module name
    pub name: String,
    /// The module description
    pub description: String,
    /// The location of the module
    pub location_id: i32,
    /// The parent/containing directory (i.e. the module directory)
    pub directory_id: i32,
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
}
