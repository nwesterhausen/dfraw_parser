use dfraw_parser::metadata::{ObjectType, RawModuleLocation};
use serde::{Deserialize, Serialize};

/// A query for searching raw objects in the database.
#[derive(Debug, Clone, Default, Serialize, Deserialize, specta::Type)]
pub struct SearchQuery {
    /// A general text search string for names and descriptions.
    pub search_string: Option<String>,
    /// Search specifically for an identifier (exact or partial).
    pub identifier_query: Option<String>,
    /// Limit search to raws found within these locations
    pub locations: Vec<RawModuleLocation>,
    /// Limit search to only be raws of this type
    pub raw_types: Vec<ObjectType>,
    /// Used to return only results with these token flags
    ///
    /// These should be the keys (from `to_keys`) on `CreatureTag`, `CasteTag`, `PlantTag`, etc.
    pub required_flags: Vec<String>,
    /// Used to return only results with these token-value pairings
    ///
    /// These should be the keys (from `to_keys`) on `CreatureTag`, `CasteTag`, `PlantTag`, etc.
    ///
    /// The value provided will be used for (minimum/exact value, maximum value)
    pub numeric_filters: Vec<(String, i32, Option<i32>)>,
}
