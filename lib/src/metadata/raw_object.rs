//! A definition purely for consumption in a tauri or JSON app, when dealing
//! with the JSON versions of objects that have the RawObject trait.

use crate::metadata::RawMetadata;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A struct purely for providing type hinting when working with parsed raws (as JSON) in typescript.
#[derive(Serialize, Deserialize, Clone, Debug, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RawObject {
    /// The raw object's identifier
    identifier: String,
    /// The raw object's metadata, which can be used to get additional information
    metadata: RawMetadata,
    /// A generated id that is used to uniquely identify this object.
    ///
    /// This is deterministic based on the following:
    /// * The raw's `identifier`
    /// * The raw's [`ObjectType`]
    /// * [`RawModuleLocation`] where the raw was found
    /// * The containing module's `numeric_version`
    object_id: Uuid,
}
