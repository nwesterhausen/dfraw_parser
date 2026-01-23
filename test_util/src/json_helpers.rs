//! Helper functions for dealing with `JSON` blobs
use dfraw_parser::traits::RawObject;

/// Get the raw identifier from a raw `JSON` blob
///
/// Takes the `UTF-8` encoded `JSON`, decodes it as a [`dyn RawObject`] and gets
/// its identifier.
///
/// # Panics
///
/// This will panic if the deserialization fails
#[must_use]
pub fn identifier_from_json_blob(json_blob: &[u8]) -> String {
    let raw_object: Box<dyn RawObject> =
        serde_json::from_slice(json_blob).expect("Failed to deserialize raw object");

    raw_object.get_identifier().to_string()
}
