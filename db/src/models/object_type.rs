//! Object type module
use diesel::prelude::*;

/// `ObjectType` is the object type data
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::object_type)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ObjectType {
    /// The primary key of the object type
    pub id: i32,
    /// The object type name
    pub name: String,
}
