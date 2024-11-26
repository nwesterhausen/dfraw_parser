//! The `directory` module contains the `Directory` struct, which represents a directory on the filesystem.
use diesel::prelude::*;

/// `Directory` abstracts a reference to a directory on the filesystem
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::directory)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Directory {
    /// The primary key of the directory
    pub id: i32,
    /// The name of the directory, i.e. the basename
    pub name: String,
    /// The full path of the directory
    pub full_path: String,
}
