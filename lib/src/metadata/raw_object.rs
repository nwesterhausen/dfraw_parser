//! A definition purely for consumption in a tauri or JSON app, when dealing
//! with the JSON versions of objects that have the RawObject trait.

use crate::metadata::RawMetadata;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A struct purely for providing type hinting when working with parsed raws (as JSON) in typescript.
#[derive(Serialize, Deserialize, Clone, Debug, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RawObject {
    /// The object identifier
    identifier: String,
    /// The metadata for this raw (includes the `ObjectType`, `RawModuleLocation` and other module info)
    metadata: RawMetadata,
    /// A deterministed uuid for the object
    object_id: Uuid,
}
