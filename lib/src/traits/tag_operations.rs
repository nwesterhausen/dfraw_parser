//! This module contains the `TagOperations` trait, which is used to parse tokens from raw files.

/// The `TokenOperations` trait is used to parse tokens from raw files.
pub trait TagOperations: Sized + Sync + Send {
    /// Parse an unknown token with a key and a (possibly empty) value(s).
    fn parse(key: &str, value: &str) -> Option<Self>;
}
