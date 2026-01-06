use rusqlite::Result;

/// Simple extension trait for Rusqlite to handle Optional rows easily.
pub trait OptionalResultExtension<T> {
    fn optional(self) -> Result<Option<T>>;
}

impl<T> OptionalResultExtension<T> for Result<T> {
    fn optional(self) -> Result<Option<T>> {
        match self {
            Ok(val) => Ok(Some(val)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
