use std::fmt::Debug;

/// A structured response for search operations, containing the requested page of data
/// and the total count of matches in the database.
#[derive(serde::Serialize, serde::Deserialize, Clone, Default, specta::Type)]
pub struct SearchResults<T = serde_json::Value> {
    /// The page of results found.
    pub results: Vec<ResultWithId<T>>,
    /// The total number of matches in the database (ignoring pagination limits).
    pub total_count: u32,
}

/// A carrier struct for passing the database id along with the object we retrieved.
#[derive(serde::Serialize, serde::Deserialize, Clone, Default, specta::Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ResultWithId<T = serde_json::Value> {
    /// id of the object in the database on its respective table
    pub id: i64,
    /// the object retrieved
    pub data: T,
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

#[allow(clippy::missing_fields_in_debug)]
impl<T> Debug for ResultWithId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResultWithId")
            .field("id", &self.id)
            .field("data_type", &std::any::type_name::<T>())
            .finish()
    }
}
