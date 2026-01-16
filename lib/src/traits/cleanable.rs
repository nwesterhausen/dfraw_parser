//! trait for being able to 'clean'

use crate::traits::IsEmpty;

/// Allows for the trait to be reduced.
///
/// This allows a struct to be "reduced" if it is holding an `Option`
/// with a default value (it sets that field to `None`)
pub trait Cleanable: IsEmpty {
    /// Reduces the struct to take up less memory in-place.
    ///
    /// This should set empty Options to None and recursively clean child elements.
    fn clean(&mut self);

    /// Returns a cleaned copy of the struct
    #[must_use]
    fn cleaned(&self) -> Self
    where
        Self: Clone,
    {
        let mut cleaned = self.clone();
        cleaned.clean();
        cleaned
    }
}
