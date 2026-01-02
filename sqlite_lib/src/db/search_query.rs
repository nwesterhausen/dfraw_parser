#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub name_query: Option<String>,
    pub raw_type_name: Option<String>,
    pub required_flags: Vec<String>,
    pub numeric_filters: Vec<(String, i32)>, // (Token, MinValue)
}
