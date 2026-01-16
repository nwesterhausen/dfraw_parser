//! to raw string trait

/// Provides a way to dispay the trait as it would in a
/// Dwarf Fortress raw file.
pub trait ToRawFileString {
    /// Formats the trait as its 'raw' implmentation.
    fn to_raw(&self) -> String;
}
