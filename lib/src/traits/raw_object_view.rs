//! The `RawObject` trait is implemented by all raw objects. This module contains definitions for it
//! and the `RawObjectToAny` trait, which is used to downcast a specific raw object from `Any`.

use std::any::Any;

use uuid::Uuid;

use crate::{
    metadata::{NumericToken, RawMetadata},
    tokens::ObjectType,
};

/// The `RawObjectView` trait is implemented by all raw object views. This trait is used
/// to provide a common interface for all raw object views, so that they can be stored in
/// a single vector.
#[typetag::serde(tag = "type")]
pub trait RawObjectView: RawObjectViewToAny + Send + Sync {
    /// Get the metadata for the raw.
    fn get_metadata(&self) -> RawMetadata;
    /// Get the identifier of the raw.
    fn get_identifier(&self) -> &str;
    /// Get the type of the raw.
    fn get_type(&self) -> ObjectType;
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
    /// Get the module object id that this raw belongs to
    fn get_module_object_id(&self) -> Uuid {
        self.get_metadata().get_module_object_id()
    }
}

/// The `RawObjectViewToAny` trait is implemented by all raw object views. This trait is
/// used to be able to downcast a raw object view to `Any`, so it can be downcast to
/// a specific raw object type.
pub trait RawObjectViewToAny: 'static {
    /// Get the raw object as `Any`.
    fn as_any(&self) -> &dyn Any;
}

/// The `RawObjectViewToAny` trait is implemented by all raw object views. This trait
/// is used to be able to downcast a raw object view to `Any`, so it can be downcast
/// to a specific raw object type.
///
/// Make sure that the raw object reports to you the correct `ObjectType` that is
/// expected for the downcast.
impl<T: 'static> RawObjectViewToAny for T {
    /// Get the raw object as `Any`.
    fn as_any(&self) -> &dyn Any {
        self
    }
}
