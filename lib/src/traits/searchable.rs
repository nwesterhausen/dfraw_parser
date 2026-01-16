//! Searchable Trait

/// The `Searchable` trait is used to define a method that returns a vector of strings that can be
/// used to search for an object. This is used to search for raws in the raws database.
pub trait Searchable {
    /// The `get_search_vec` function returns a vector of strings that can be used to search for an
    /// object.
    fn get_search_vec(&self) -> Vec<String>;
}
