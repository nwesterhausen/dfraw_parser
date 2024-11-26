//! This module contains the `Location` struct, which represents a location where a module can be found.
use diesel::prelude::*;

/// `Location` refers to one of the possible locations where a module can be found.
///
/// Should roughly equat to "mods", "installed mods" and "vanilla". However, a custom location
/// could be used when loading modules manually.
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::location)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(directory::Directory, foreign_key = directory_id))]
pub struct Location {
    /// The primary key of the location
    pub id: i32,
    /// The name of the location
    pub name: String,
    /// The directory where the location is found
    pub directory_id: i32,
}
