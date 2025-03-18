//! These traits are used to provide a common interface for various parsed objects. They are used
//! to simplify the process of storing and searching for these objects in the database.
//!
//! It exists to simplify the process of inserting a new record into the database. For example,
//! the `[Creature]` struct implements this trait, so you can insert a new creature into the
//! database without having to worry about the details of how to do so.

/// This trait is used to insert a new record into the database.
///
/// It provides a method to get the SQL required to insert the record into the database.
pub trait Insertable {
    /// Returns the SQL required to insert the record into the database.
    fn to_insert_sql(&self) -> String;
}

/// This trait is used to get an object out of the database.
///
/// It provides methods to query the database and return the object.
///
/// More methods to be added if needed.
pub trait Queryable {
    /// Will query the database and return the object, by its identifier.
    fn get_by_identifier(identifier: &str) -> Option<Self>
    where
        Self: Sized;

    /// Will query the database and return the object, by its id.
    fn get_by_id(id: i32) -> Option<Self>
    where
        Self: Sized;
}
