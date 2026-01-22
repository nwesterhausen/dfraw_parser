/// Helper struct for providing "common" tokens and their values to consumers (for searching or other operations)
#[derive(
    Default, Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash, specta::Type,
)]
pub struct NumericToken {
    /// String key for the token, with an appended clue if more than one value exists
    ///
    /// e.g. for "CLUTCH_SIZE", use "CLUTCH_SIZE_MIN" and "CLUTCH_SIZE_MAX"; for "PETVALUE" use "PETVALUE"
    pub key: String,
    /// The value associated with the key
    pub value: i64,
}

impl NumericToken {
    /// Create a new struct for given key and value
    pub fn new(key: impl Into<String>, value: impl Into<i64>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}
