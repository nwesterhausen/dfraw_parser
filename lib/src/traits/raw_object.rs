//! The `RawObject` trait is implemented by all raw objects. This module contains definitions for it
//! and the `RawObjectToAny` trait, which is used to downcast a specific raw object from `Any`.

use std::any::Any;

use uuid::Uuid;

use crate::{
    metadata::{NumericToken, RawMetadata},
    tokens::ObjectType,
    traits::{Cleanable, ToRawFileString},
};

#[allow(clippy::module_name_repetitions)]
#[typetag::serde(tag = "type")]
/// The `RawObject` trait is implemented by all raw objects. This trait is used
/// to provide a common interface for all raw objects, so that they can be
/// stored in a single vector. It also provides a common interface for parsing.
pub trait RawObject: RawObjectToAny + ToRawFileString + Send + Sync + Cleanable {
    /// Get the metadata for the raw.
    fn get_metadata(&self) -> RawMetadata;
    /// Get the identifier of the raw.
    fn get_identifier(&self) -> &str;
    /// Get the type of the raw.
    fn get_type(&self) -> ObjectType;
    /// Parse a new tag from the raw file into this raw object.
    ///
    /// Arguments:
    ///
    /// * `key`: The key of the tag. The first part of a tag, before the colon.
    /// * `value`: The value of the tag. The second part of a tag, after the colon.
    /// The `value` might be empty, if there is no value after the colon.
    fn parse_tag(&mut self, key: &str, value: &str);
    /// Get the object ID of the raw.
    fn get_object_id(&self) -> Uuid;
    /// Get the name of the raw (if it has one).
    /// If no name is found, the identifier is returned instead.
    /// This is used for searching.
    fn get_name(&self) -> &str;
    /// Function to return "flag" tokens (as strings) for things like `[FLIER]` or `[INTELLIGENT]`, etc
    fn get_searchable_tokens(&self) -> Vec<&str> {
        Vec::new()
    }
    /// Function to return "numeric flag" tokens for things like `[PET_VALUE:50]` or `[CLUTCH_SIZE:2:6]`.
    ///
    /// Returns a vector of [`NumericToken`].
    fn get_numeric_flags(&self) -> Vec<NumericToken> {
        Vec::new()
    }
    /// Function to "clean" the raw. This is used to remove any empty list or strings,
    /// and to remove any default values. By "removing" it means setting the value to None.
    ///
    /// This also will remove the metadata if is_metadata_hidden is true.
    fn clean_self(&mut self) {
        self.clean()
    }
    /// Get the module object id that this raw belongs to
    fn get_module_object_id(&self) -> Uuid;
}

/// The `RawObjectToAny` trait is implemented by all raw objects. This trait is
/// used to be able to downcast a raw object to `Any`, so it can be downcast to
/// a specific raw object type.
pub trait RawObjectToAny: 'static {
    /// Get the raw object as `Any`.
    fn as_any(&self) -> &dyn Any;
}

/// The `RawObjectToAnyImpl` trait is implemented by all raw objects. This trait
/// is used to be able to downcast a raw object to `Any`, so it can be downcast
/// to a specific raw object type.
///
/// Make sure that the raw object reports to you the correct `ObjectType` that is
/// expected for the downcast.
impl<T: 'static> RawObjectToAny for T {
    /// Get the raw object as `Any`.
    fn as_any(&self) -> &dyn Any {
        self
    }
}
