//! Module model
use diesel::prelude::*;

/// `Module` is the module data
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::module)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Location, foreign_key = location_id))]
#[diesel(belongs_to(InfoFile, foreign_key = info_file_id))]
#[diesel(belongs_to(Directory, foreign_key = directory_id))]
pub struct Module {
    /// The primary key of the module
    pub id: i32,
    /// The module name
    pub name: String,
    /// The module description
    pub description: String,
    /// The location of the module
    pub location_id: i32,
    /// The info file ID
    pub info_file_id: Option<i32>,
    /// The directory the module is in
    pub directory_id: i32,
}
