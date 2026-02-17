use std::str::FromStr;

use crate::traits::TagOperations;

/// Has methods for dealing with tokens and handling them.
pub trait RawToken: TagOperations + FromStr {
    /// Retrieves the original string token key for this tag (e.g., "PETVALUE"). This is done
    /// by reversing the string --> token mapping that already exists, providing O(1) lookup.
    fn get_key(&self) -> Option<&'static str>;
    /// Formats the token as its 'raw' version
    fn to_raw_token(&self) -> String {
        // A default implmentation. This does not include any values, so it is sufficient for
        // simple tokens like biomes and conditions.
        match self.get_key() {
            Some(key) => format!("[{key}]"),
            None => String::new(),
        }
    }
}
