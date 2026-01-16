//! Used to determine if a token is within a raw object.

use super::RawObject;

#[typetag::serialize]
/// The `RawObjectToken` trait is implemented by all raw object tokens. This trait is used
/// to provide a common interface for all raw object tokens, so that they can be
/// stored in a single vector. It also provides a common interface for parsing.
pub trait RawObjectToken<T: RawObject> {
    /// Check if the token is part of the given `RawObject`. This simplifies searching through
    /// a `RawObject`'s token arrays to find the token, instead letting that be implemented
    /// deliberately/specifically for each combo of token and raw object.
    fn is_within(&self, object: &T) -> bool;
    /// Retrieves the original string token key for this tag (e.g., "PET_VALUE"). This is done
    /// by reversing the string --> token mapping that already exists, providing O(1) lookup.
    fn get_key(&self) -> Option<&'static str>;
}
