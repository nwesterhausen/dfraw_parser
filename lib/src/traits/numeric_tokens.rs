//! Helper trait for [`NumericToken`] usage

use crate::metadata::NumericToken;

/// Trait for retrieving [`NumericToken`]s from a raw token
pub trait NumericTokenTransform {
    /// Returns the token in a vec of [`NumericToken`] for db/search index consumption
    fn as_numeric_tokens(&self) -> Vec<NumericToken> {
        Vec::new()
    }
}
