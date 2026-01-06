use serde::{Deserialize, Serialize};

/// A structured response for search operations, containing the requested page of data
/// and the total count of matches in the database.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct SearchResults<T> {
    /// The page of results found.
    pub results: Vec<T>,
    /// The total number of matches in the database (ignoring pagination limits).
    pub total_count: u32,
}
