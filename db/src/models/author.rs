//! The `author` module contains the `Author` struct.
use diesel::prelude::*;

/// `Author` is the author of a module.
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::author)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Author {
    /// The primary key of the author
    pub id: i32,
    /// The name of the author
    pub name: String,
}
