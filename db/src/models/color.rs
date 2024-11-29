//! Color model
use diesel::prelude::*;

/// Color describes a tile color
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::color)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Color {
    /// The primary key of the color
    pub id: i32,
    /// The foreground color
    pub foreground: i32,
    /// The background color
    pub background: i32,
    /// The brightness of the color
    pub brightness: i32,
}
