#[derive(Debug, Clone)]
/// Search query options
pub struct SearchQuery {
    /// Used to return only raws with name containing this
    pub name_query: Option<String>,
    /// Used to return only raws with type matching this
    pub raw_type_name: Option<String>,
    /// Used to return only results with these token flags
    pub required_flags: Vec<String>,
    /// Used to return only results with these token-value pairings
    pub numeric_filters: Vec<(String, i32)>, // (Token, MinValue)
}
