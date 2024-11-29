//! The metadata model.

use diesel::prelude::*;

/// `Metadata` is attached to every object in the database.
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::metadata)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(ObjectType, foreign_key = object_type_id))]
#[diesel(belongs_to(Module, foreign_key = module_id))]
pub struct Metadata {
    /// The primary key of the metadata
    pub id: i32,
    /// The object type ID
    pub object_type_id: i32,
    /// The module ID
    pub module_id: i32,
    /// special `object_id` for this object
    pub object_id: String,
    /// The name of the object
    pub name: String,
    /// the raw identifier
    pub raw_identifier: String,
    /// the raw file path
    pub raw_file_path: String,
    /// the version
    pub version: String,
}
