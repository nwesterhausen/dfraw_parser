use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::InfoFile;

use super::{ObjectType, RawModuleLocation};

/// The `RawMetadata` struct represents metadata about a raw module in Rust, including its name,
/// version, file path, identifier, object type, module location, and visibility status.
///
/// Properties:
///
/// * `module_name`: The name of the raw module the raw is from.
/// * `module_version`: The version of the raw module the raw is from.
/// * `raw_file_path`: The `raw_file_path` property is a string that represents the path to the file
///   containing the raw data. It specifies the location of the file on the file system.
/// * `raw_identifier`: The raw identifier is a unique identifier for the raw data. It is typically
///   found at the top of the raw text file and is used to identify and reference the specific raw data.
/// * `object_type`: The `object_type` property represents the type of the raw data. It could be a
///   creature, plant, or any other type specified in the raw text file.
/// * `raw_module_location`: The `raw_module_location` property represents the location of the owning
///   raw module. It can have one of the following values:
///
///     - `RawModuleLocation::InstalledMods`: The raw module is located in the `installed_mods` folder.
///     - `RawModuleLocation::Mods`: The raw module is located in the `mods` folder.
///     - `RawModuleLocation::Vanilla`: The raw module is located in the `vanilla` folder.
///
/// * `hidden`: The `hidden` property is a boolean value that indicates whether the raw metadata should
///   be hidden or not when exporting. By default, it is set to `true`, meaning that the raw metadata will
///   be hidden unless specified in the `ParsingOptions` struct.
#[derive(Serialize, Deserialize, Clone, Debug, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    // The object_id of the raw module
    module_object_id: String,
    // The name of the raw module the raw is from.
    module_name: String,
    // The version of the raw module the raw is from.
    module_version: String,
    // The path to the file containing the raw.
    raw_file_path: String,
    // The raw identifier (as described at the top of the raw text file).
    raw_identifier: String,
    // The type of raw (creature, plant, etc).
    // Example: [OBJECT:TYPE]
    object_type: ObjectType,
    // The location of the owning raw module
    // i.e. installed_mods, mods, or vanilla
    raw_module_location: RawModuleLocation,
    // Optionally hide or unhide from exporting
    // By default will be hidden
    #[serde(skip)]
    hidden: bool,
}

impl Metadata {
    /// Create a new `RawMetadata` instance.
    ///
    /// # Arguments
    ///
    /// * `module_info` - The `InfoFile` instance containing the module information.
    /// * `object_type` - The `ObjectType` of the raw data.
    /// * `raw_identifier` - The identifier of the raw data.
    /// * `raw_file_path` - The path to the raw file.
    /// * `attach_metadata_to_raws` - Whether to attach metadata to raws.
    ///
    /// # Returns
    ///
    /// A new `RawMetadata` instance.
    #[must_use]
    pub fn new<P: AsRef<Path>>(
        module_info: &InfoFile,
        object_type: &ObjectType,
        raw_identifier: &str,
        raw_file_path: &P,
        attach_metadata_to_raws: bool,
    ) -> Self {
        Self {
            module_name: module_info.get_name(),
            module_version: module_info.get_version(),
            raw_file_path: String::from(raw_file_path.as_ref().to_str().unwrap_or_default()),
            raw_identifier: String::from(raw_identifier),
            object_type: object_type.clone(),
            raw_module_location: module_info.get_location(),
            module_object_id: module_info.get_object_id(),
            hidden: !attach_metadata_to_raws,
        }
    }
    /// (Hidden from export) Used only for serialization
    ///
    /// # Returns
    ///
    /// * `true` if the metadata is hidden, `false` otherwise.
    #[must_use]
    pub const fn is_hidden(&self) -> bool {
        self.hidden
    }
    /// Get the identifier of the raw file the raw is from.
    ///
    /// # Returns
    ///
    /// * The identifier of the raw file as a `&str`
    #[must_use]
    pub fn get_raw_identifier(&self) -> &str {
        &self.raw_identifier
    }
    /// Get the name of the module the raw is from.
    ///
    /// # Returns
    ///
    /// * The name of the module as a `&str`
    #[must_use]
    pub fn get_module_name(&self) -> &str {
        &self.module_name
    }
    /// Get the (numeric) version of the module the raw is from.
    ///
    /// # Returns
    ///
    /// * The version of the module as a `&str`
    #[must_use]
    pub fn get_module_numerical_version(&self) -> &str {
        &self.module_version
    }
    /// Get the (string) version of the module the raw is from.
    ///
    /// # Returns
    ///
    /// * The version of the module as a `&str`
    #[must_use]
    pub fn get_module_version(&self) -> &str {
        &self.module_version
    }
    /// Get the full path to the raw file the raw is from.
    ///
    /// # Returns
    ///
    /// * The full path to the raw file as a `&str`
    #[must_use]
    pub fn get_raw_file_path(&self) -> &str {
        &self.raw_file_path
    }
    /// Get the location of the owning raw module.
    ///
    /// # Returns
    ///
    /// * `RawModuleLocation` - The location of the owning raw module
    #[must_use]
    pub const fn get_location(&self) -> RawModuleLocation {
        self.raw_module_location
    }
    /// Get the `object_id` of the owning raw module.
    ///
    /// # Returns
    ///
    /// * The `object_id` of the owning raw module as a `&str`
    #[must_use]
    pub fn get_module_object_id(&self) -> &str {
        &self.module_object_id
    }

    /// Set the `object_type` of the metadata at creation.
    ///
    /// # Arguments
    ///
    /// * `object_type` - The `ObjectType` to set
    #[must_use]
    pub const fn with_object_type(mut self, object_type: ObjectType) -> Self {
        self.object_type = object_type;
        self
    }
    /// Set the `raw_module_location` of the metadata at creation.
    ///
    /// # Arguments
    ///
    /// * `raw_module_location` - The `RawModuleLocation` to set
    #[must_use]
    pub const fn with_raw_module_location(
        mut self,
        raw_module_location: RawModuleLocation,
    ) -> Self {
        self.raw_module_location = raw_module_location;
        self
    }
    /// Set the `hidden` status of the metadata at creation.
    ///
    /// # Arguments
    ///
    /// * `hidden` - The hidden status to set
    #[must_use]
    pub const fn with_hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }
    /// Set the `raw_identifier` of the metadata at creation.
    ///
    /// # Arguments
    ///
    /// * `raw_identifier` - The raw identifier to set
    #[must_use]
    pub fn with_raw_identifier(mut self, raw_identifier: String) -> Self {
        self.raw_identifier = raw_identifier;
        self
    }
    /// Set the `raw_file_path` of the metadata at creation.
    ///
    /// # Arguments
    ///
    /// * `raw_file_path` - The raw file path to set
    #[must_use]
    pub fn with_raw_file_path(mut self, raw_file_path: String) -> Self {
        self.raw_file_path = raw_file_path;
        self
    }
    /// Set the `module_name` of the metadata at creation.
    ///
    /// # Arguments
    ///
    /// * `module_name` - The module name to set
    #[must_use]
    pub fn with_module_name(mut self, module_name: String) -> Self {
        self.module_name = module_name;
        self
    }
    /// Set the `module_version` of the metadata at creation.
    ///
    /// # Arguments
    ///
    /// * `module_version` - The module version to set
    #[must_use]
    pub fn with_module_version(mut self, module_version: String) -> Self {
        self.module_version = module_version;
        self
    }
    /// Set the `module_object_id` of the metadata at creation.
    ///
    /// # Arguments
    ///
    /// * `module_object_id` - The module object ID to set
    #[must_use]
    pub fn with_module_object_id(mut self, module_object_id: String) -> Self {
        self.module_object_id = module_object_id;
        self
    }
}
