use serde::{Deserialize, Serialize};

/// A query for searching raw objects in the database.
#[derive(Debug, Clone, Default, Serialize, Deserialize, specta::Type)]
pub struct SearchQuery {
    /// A general text search string for names and descriptions.
    pub search_string: Option<String>,
    /// Search specifically for an identifier (exact or partial).
    pub identifier_query: Option<String>,
    /// Used to return only raws with type matching this
    pub raw_type_name: Option<String>,
    /// Used to return only results with these token flags
    pub required_flags: Vec<String>,
    /// Used to return only results with these token-value pairings
    pub numeric_filters: Vec<(String, i32)>, // (Token, MinValue)
}
