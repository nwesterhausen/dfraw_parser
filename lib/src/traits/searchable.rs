/// Provides easy accessors to get values to search on.
pub trait Searchable {
    /// Get a slice of all names in this raw
    fn get_all_names(&self) -> Vec<String>;
    /// Get a slice of all descriptions in this raw
    fn get_all_descriptions(&self) -> Vec<String>;
}
