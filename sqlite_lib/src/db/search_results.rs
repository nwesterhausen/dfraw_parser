use std::fmt::Debug;

use serde::{Deserialize, Serialize};

/// A structured response for search operations, containing the requested page of data
/// and the total count of matches in the database.
#[derive(Clone, Serialize, Deserialize, specta::Type)]
pub struct SearchResults<T> {
    /// The page of results found.
    pub results: Vec<T>,
    /// The total number of matches in the database (ignoring pagination limits).
    pub total_count: u32,
}

impl<T> Debug for SearchResults<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let calculated_result_len = format!("Vec<T>::{}", &self.results.len());
        f.debug_struct("SearchResults")
            .field("results_count", &calculated_result_len)
            .field("type_t", &std::any::type_name::<T>())
            .field("total_count", &self.total_count)
            .finish()
    }
}
