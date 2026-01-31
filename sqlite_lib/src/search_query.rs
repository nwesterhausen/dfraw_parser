use dfraw_parser::{metadata::RawModuleLocation, tokens::ObjectType};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::NumericFilter;

/// A query for searching raw objects in the database.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
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
    /// (e.g. `FLIER`, `EGG_LAYER`, `FIREIMMUNE`)
    pub required_flags: Vec<String>,
    /// Used to return only results with these token-value pairings
    ///
    /// These should be the keys (from `to_keys`) on `CreatureTag`, `CasteTag`, `PlantTag`, etc.
    /// (e.g. `LITTER_SIZE`, `POP_RATIO`, `CLUSTER_NUMBER`)
    ///
    /// The value provided will be used for (minimum/exact value, maximum value)
    pub numeric_filters: Vec<NumericFilter>,
    /// Limit the number of raws returned to this amount per page
    ///
    /// Default: `50`
    pub limit: u32,
    /// Which page to return
    ///
    /// Default: `1`
    pub page: u32,
    /// Limit results to only include favorited raws
    ///
    /// Default: false
    pub favorites_only: bool,
    /// Limit results to only be within these modules.
    ///
    /// Specify using the module's `object_id` which can be found from the raw:
    ///     `raw.metadata.module_object_it`
    pub in_modules: Vec<Uuid>,
}

impl SearchQuery {
    /// Whether the query meets the requirements for a full-text search
    #[must_use]
    pub const fn is_full_text_search(&self) -> bool {
        if let Some(s) = self.search_string.as_ref()
            && s.len() > 2
        {
            true
        } else {
            false
        }
    }
    /// Computed offset
    #[must_use]
    pub const fn offset(&self) -> u32 {
        (self.page.saturating_sub(1)) * self.limit
    }
    /// Cleans the query by setting any empty strings into None instead
    #[must_use]
    pub fn clean(&self) -> Self {
        let mut cleaned = self.clone();

        if let Some(s) = self.search_string.as_ref()
            && s.is_empty()
        {
            cleaned.search_string = None;
        }

        if let Some(s) = self.identifier_query.as_ref()
            && s.is_empty()
        {
            cleaned.identifier_query = None;
        }

        cleaned
    }
}

pub const DEFAULT_SEARCH_LIMIT: u32 = 18;

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            search_string: None,
            identifier_query: None,
            locations: Vec::new(),
            raw_types: Vec::new(),
            required_flags: Vec::new(),
            numeric_filters: Vec::new(),
            limit: DEFAULT_SEARCH_LIMIT,
            page: 1,
            favorites_only: false,
            in_modules: Vec::new(),
        }
    }
}
