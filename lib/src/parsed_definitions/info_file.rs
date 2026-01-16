//! The `info.txt` file for a raw module. This file contains metadata about the module.

use std::{
    io::{BufRead, BufReader, Error, ErrorKind},
    path::Path,
};

use dfraw_parser_proc_macros::{Cleanable, IsEmpty};
use encoding_rs_io::DecodeReaderBytesBuilder;
use slug::slugify;
use tracing::{debug, error, trace, warn};

use crate::{
    ParserError,
    constants::DF_ENCODING,
    metadata::RawModuleLocation,
    regex::{NON_DIGIT_RE, RAW_TOKEN_RE},
    utilities::{get_parent_dir_name, try_get_file},
};

use super::steam_data::SteamData;

/// Represents the `info.txt` file for a raw module
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Default,
    Clone,
    Debug,
    specta::Type,
    PartialEq,
    Eq,
    IsEmpty,
    Cleanable,
)]
#[serde(rename_all = "camelCase")]
pub struct InfoFile {
    identifier: String,
    object_id: String,
    location: RawModuleLocation,
    parent_directory: String,
    numeric_version: u32,
    displayed_version: String,
    earliest_compatible_numeric_version: u32,
    earliest_compatible_displayed_version: String,
    author: String,
    name: String,
    description: String,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    requires_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    conflicts_with_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    requires_ids_before: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    requires_ids_after: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    steam_data: Option<SteamData>,
}

impl InfoFile {
    /// Creates a new `InfoFile` with the passed identifier, location, and parent directory
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier for the `InfoFile`
    /// * `location` - The location the `InfoFile` was parsed from
    /// * `parent_directory` - The directory the `InfoFile` was parsed from
    ///
    /// # Returns
    ///
    /// * The `InfoFile`
    #[must_use]
    pub fn new(id: &str, location: RawModuleLocation, parent_directory: &str) -> Self {
        Self {
            identifier: String::from(id),
            location,
            parent_directory: String::from(parent_directory),
            object_id: format!("{}-{}-{}", location, "MODULE", slugify(id)),
            ..Default::default()
        }
    }
    /// Creates a new empty `InfoFile`
    ///
    /// # Returns
    ///
    /// * The empty `InfoFile`
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }
    /// Creates a new `InfoFile` from the passed `info.txt` file path
    ///
    /// # Arguments
    ///
    /// * `full_path` - The full path to the `info.txt` file
    ///
    /// # Returns
    ///
    /// * `Result<InfoFile, ParserError>` - The parsed `InfoFile` or an error
    ///
    /// # Errors
    ///
    /// * `ParserError::FileNotFound` - If the passed file path does not exist
    /// * `ParserError::IOError` - If there is an error reading the file
    pub fn from_raw_file_path<P: AsRef<Path>>(
        full_path: &P,
        warn_on_format_issue: bool,
    ) -> Result<Self, ParserError> {
        let path = full_path.as_ref();
        // Validate that the passed file exists
        if !path.exists() {
            return Err(ParserError::Io {
                source: Error::new(
                    ErrorKind::NotFound,
                    format!(
                        "Raw file to find 'info.txt' for doesn't exist: {}",
                        path.display()
                    ),
                ),
            });
        }

        // Take the full path for the raw file and navigate up until we find the info.txt
        // It's possible to nest raws inside subdirectories in the modules, so that we may have to go up 2 or more
        // parents before we find it.
        for dir in path.ancestors().skip(1) {
            let info_file_path = dir.join("info.txt");

            if info_file_path.exists() {
                return Self::parse(&info_file_path, warn_on_format_issue);
            }
        }

        // If we reach here, we were not able to find an 'info.txt' file
        Err(ParserError::Io {
            source: Error::new(
                ErrorKind::NotFound,
                format!("'info.txt' not found in any parent of {}", path.display()),
            ),
        })
    }
    /// Parses the `info.txt` file at the passed path
    ///
    /// # Arguments
    ///
    /// * `info_file_path` - The path to the `info.txt` file
    ///
    /// # Returns
    ///
    /// * `Result<InfoFile, ParserError>` - The parsed `InfoFile` or an error
    ///
    /// # Errors
    ///
    /// * `ParserError::FileNotFound` - If the passed file path does not exist
    /// * `ParserError::IOError` - If there is an error reading the file
    #[allow(clippy::cognitive_complexity, clippy::too_many_lines)]
    pub fn parse<P: AsRef<Path>>(
        info_file_path: &P,
        warn_on_format_issue: bool,
    ) -> Result<Self, ParserError> {
        let parent_dir = get_parent_dir_name(info_file_path);
        let location = RawModuleLocation::from_path(info_file_path);

        let file = try_get_file(info_file_path).map_err(|e| {
            ParserError::InvalidRawFile(format!(
                "Unable to open raw info file {}: {}",
                info_file_path.as_ref().display(),
                e
            ))
        })?;

        let decoding_reader = DecodeReaderBytesBuilder::new()
            .encoding(Some(*DF_ENCODING))
            .build(file);
        let reader = BufReader::new(decoding_reader);

        // info.txt details
        let mut info_file_data: Self = Self::new("", location, &parent_dir);

        for (index, line) in reader.lines().enumerate() {
            if line.is_err() {
                error!("parse: Error processing {:?}:{}", parent_dir, index);
                continue;
            }
            let line = match line {
                Ok(l) => l,
                Err(e) => {
                    error!("parse:  Line-reading error");
                    debug!("{:?}", e);
                    continue;
                }
            };
            for cap in RAW_TOKEN_RE.captures_iter(&line) {
                let captured_key = match cap.get(2) {
                    Some(v) => v.as_str(),
                    _ => {
                        continue;
                    }
                };
                let captured_value = match cap.get(3) {
                    Some(v) => v.as_str(),
                    _ => {
                        continue;
                    }
                };

                trace!(
                    "ModuleInfoFile::parse: Key: {} Value: {}",
                    captured_key, captured_value
                );

                match captured_key {
                    // SECTION FOR MATCHING info.txt DATA
                    "ID" => {
                        // the [ID:identifier] tag should be the top of the info.txt file
                        info_file_data = Self::new(captured_value, location, &parent_dir);
                    }
                    "NUMERIC_VERSION" => match captured_value.parse() {
                        Ok(n) => info_file_data.numeric_version = n,
                        Err(_e) => {
                            if warn_on_format_issue {
                                warn!(
                                    "ModuleInfoFile::parse: 'NUMERIC_VERSION' should be integer '{}' in {}",
                                    captured_value,
                                    info_file_path.as_ref().display()
                                );
                            }
                            // match on \D to replace any non-digit characters with empty string
                            let digits_only =
                                NON_DIGIT_RE.replace_all(captured_value, "").to_string();
                            match digits_only.parse() {
                                Ok(n) => info_file_data.numeric_version = n,
                                Err(_e) => {
                                    error!(
                                        "ModuleInfoFile::parse: Unable to parse any numbers from '{}' for NUMERIC_VERSION",
                                        captured_value
                                    );
                                }
                            }
                        }
                    },
                    "EARLIEST_COMPATIBLE_NUMERIC_VERSION" => match captured_value.parse() {
                        Ok(n) => info_file_data.earliest_compatible_numeric_version = n,
                        Err(_e) => {
                            if warn_on_format_issue {
                                warn!(
                                    "ModuleInfoFile::parse: 'EARLIEST_COMPATIBLE_NUMERIC_VERSION' should be integer '{}' in {:?}",
                                    captured_value,
                                    info_file_path.as_ref().display()
                                );
                            }
                            // match on \D to replace any non-digit characters with empty string
                            let digits_only =
                                NON_DIGIT_RE.replace_all(captured_value, "").to_string();
                            match digits_only.parse() {
                                Ok(n) => info_file_data.earliest_compatible_numeric_version = n,
                                Err(_e) => {
                                    error!(
                                        "ModuleInfoFile::parse: Unable to parse any numbers from '{}' for EARLIEST_COMPATIBLE_NUMERIC_VERSION",
                                        captured_value
                                    );
                                }
                            }
                        }
                    },
                    "DISPLAYED_VERSION" => {
                        info_file_data.displayed_version = String::from(captured_value);
                    }
                    "EARLIEST_COMPATIBLE_DISPLAYED_VERSION" => {
                        info_file_data.earliest_compatible_displayed_version =
                            String::from(captured_value);
                    }
                    "AUTHOR" => {
                        info_file_data.author = String::from(captured_value);
                    }
                    "NAME" => {
                        info_file_data.name = String::from(captured_value);
                    }
                    "DESCRIPTION" => {
                        info_file_data.description = String::from(captured_value);
                    }
                    "REQUIRES_ID" => {
                        if info_file_data.requires_ids.is_none() {
                            info_file_data.requires_ids = Some(Vec::new());
                        }

                        if let Some(requires_ids) = info_file_data.requires_ids.as_mut() {
                            requires_ids.push(String::from(captured_value));
                        }
                    }
                    "CONFLICTS_WITH_ID" => {
                        if info_file_data.conflicts_with_ids.is_none() {
                            info_file_data.conflicts_with_ids = Some(Vec::new());
                        }

                        if let Some(conflicts_with_ids) = info_file_data.conflicts_with_ids.as_mut()
                        {
                            conflicts_with_ids.push(String::from(captured_value));
                        }
                    }
                    "REQUIRES_ID_BEFORE_ME" => {
                        if info_file_data.requires_ids_before.is_none() {
                            info_file_data.requires_ids_before = Some(Vec::new());
                        }

                        if let Some(requires_ids_before) =
                            info_file_data.requires_ids_before.as_mut()
                        {
                            requires_ids_before.push(String::from(captured_value));
                        }
                    }
                    "REQUIRES_ID_AFTER_ME" => {
                        if info_file_data.requires_ids_after.is_none() {
                            info_file_data.requires_ids_after = Some(Vec::new());
                        }

                        if let Some(requires_ids_after) = info_file_data.requires_ids_after.as_mut()
                        {
                            requires_ids_after.push(String::from(captured_value));
                        }
                    }
                    "STEAM_TITLE" => {
                        if info_file_data.steam_data.is_none() {
                            info_file_data.steam_data = Some(SteamData::default());
                        }

                        if let Some(steam_data) = info_file_data.steam_data.as_mut() {
                            steam_data.set_title(&String::from(captured_value));
                        }
                    }
                    "STEAM_DESCRIPTION" => {
                        if info_file_data.steam_data.is_none() {
                            info_file_data.steam_data = Some(SteamData::default());
                        }

                        if let Some(steam_data) = info_file_data.steam_data.as_mut() {
                            steam_data.set_description(&String::from(captured_value));
                        }
                    }
                    "STEAM_TAG" => {
                        if info_file_data.steam_data.is_none() {
                            info_file_data.steam_data = Some(SteamData::default());
                        }

                        if let Some(steam_data) = info_file_data.steam_data.as_mut() {
                            steam_data.add_tag(&String::from(captured_value));
                        }
                    }
                    "STEAM_KEY_VALUE_TAG" => {
                        if info_file_data.steam_data.is_none() {
                            info_file_data.steam_data = Some(SteamData::default());
                        }

                        if let Some(steam_data) = info_file_data.steam_data.as_mut() {
                            steam_data.add_key_value_tag(&String::from(captured_value));
                        }
                    }
                    "STEAM_METADATA" => {
                        if info_file_data.steam_data.is_none() {
                            info_file_data.steam_data = Some(SteamData::default());
                        }

                        if let Some(steam_data) = info_file_data.steam_data.as_mut() {
                            steam_data.add_metadata(&String::from(captured_value));
                        }
                    }
                    "STEAM_CHANGELOG" => {
                        if info_file_data.steam_data.is_none() {
                            info_file_data.steam_data = Some(SteamData::default());
                        }

                        if let Some(steam_data) = info_file_data.steam_data.as_mut() {
                            steam_data.set_changelog(&String::from(captured_value));
                        }
                    }
                    "STEAM_FILE_ID" => match captured_value.parse() {
                        Ok(n) => {
                            if info_file_data.steam_data.is_none() {
                                info_file_data.steam_data = Some(SteamData::default());
                            }

                            if let Some(steam_data) = info_file_data.steam_data.as_mut() {
                                steam_data.set_file_id(n);
                            }
                        }
                        Err(_e) => {
                            if warn_on_format_issue {
                                warn!(
                                    "ModuleInfoFile::parse: 'STEAM_FILE_ID' should be integer, was {} in {}",
                                    captured_value,
                                    info_file_path.as_ref().display()
                                );
                            }
                            // match on \D to replace any non-digit characters with empty string
                            let digits_only =
                                NON_DIGIT_RE.replace_all(captured_value, "").to_string();
                            match digits_only.parse() {
                                Ok(n) => {
                                    if info_file_data.steam_data.is_none() {
                                        info_file_data.steam_data = Some(SteamData::default());
                                    }

                                    if let Some(steam_data) = info_file_data.steam_data.as_mut() {
                                        steam_data.set_file_id(n);
                                    }
                                }
                                Err(_e) => {
                                    error!(
                                        "ModuleInfoFile::parse: Unable to parse any numbers from '{}' for STEAM_FILE_ID",
                                        captured_value
                                    );
                                }
                            }
                        }
                    },
                    &_ => (),
                }
            }
        }

        // Do some final checks to confirm that the name is set. Specifically in "Dark Ages V - War & Mythos" the
        // [name] Token in the info.txt is written incorrectly as "[name]X" instead of [name:X]
        if info_file_data.name.is_empty() || info_file_data.name.is_empty() {
            info_file_data.name = info_file_data.get_identifier();
        }

        // Check for 'unknown' identifier and try to provide any extra info
        if info_file_data.get_identifier() == "unknown" {
            error!(
                "Failure parsing proper info from {}",
                info_file_path.as_ref().display()
            );
        }

        Ok(info_file_data)
    }
    /// Returns the identifier for the `InfoFile`
    #[must_use]
    pub fn get_identifier(&self) -> String {
        String::from(&self.identifier)
    }
    /// Returns the location the `InfoFile` was parsed from
    #[must_use]
    pub const fn get_location(&self) -> RawModuleLocation {
        self.location
    }
    /// Returns the description for the `InfoFile`
    #[must_use]
    pub fn get_description(&self) -> String {
        String::from(&self.description)
    }
    /// Returns the name for the `InfoFile`
    #[must_use]
    pub fn get_name(&self) -> String {
        String::from(&self.name)
    }
    /// Returns the displayed version for the `InfoFile`
    #[must_use]
    pub fn get_version(&self) -> String {
        String::from(&self.displayed_version)
    }
    /// Returns the numeric version for the `InfoFile`
    #[must_use]
    pub fn get_numeric_version(&self) -> u32 {
        self.numeric_version
    }
    /// Returns the `earliest_compatible_numeric_version` for the `InfoFile`
    #[must_use]
    pub fn get_earliest_compatible_numeric_version(&self) -> u32 {
        self.earliest_compatible_numeric_version
    }
    /// Returns the `earliest_compatible_displayed_version` for the `InfoFile`
    #[must_use]
    pub fn get_earliest_compatible_displayed_version(&self) -> String {
        String::from(&self.earliest_compatible_displayed_version)
    }
    /// Returns the author for the `InfoFile`
    #[must_use]
    pub fn get_author(&self) -> String {
        String::from(&self.author)
    }
    /// Returns the module's object id
    #[must_use]
    pub fn get_object_id(&self) -> String {
        String::from(&self.object_id)
    }
    /// Returns the `SteamData` for the info file if it exists.
    #[must_use]
    pub fn get_steam_data(&self) -> Option<SteamData> {
        self.steam_data.clone()
    }
    #[must_use]
    pub fn get_requires_ids(&self) -> Option<Vec<String>> {
        self.requires_ids.clone()
    }
    #[must_use]
    pub fn get_requires_ids_before(&self) -> Option<Vec<String>> {
        self.requires_ids_before.clone()
    }
    #[must_use]
    pub fn get_requires_ids_after(&self) -> Option<Vec<String>> {
        self.requires_ids_after.clone()
    }
    #[must_use]
    pub fn get_conflicts_with_ids(&self) -> Option<Vec<String>> {
        self.conflicts_with_ids.clone()
    }
    /// Returns the directory the `InfoFile` was parsed from
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::path::Path;
    /// use dfraw_parser::{InfoFile, metadata::RawModuleLocation};
    ///
    /// let mut info_file = InfoFile::new("vanilla_creatures", RawModuleLocation::Vanilla, "vanilla_creatures");
    ///
    /// assert_eq!(info_file.get_parent_directory(), "vanilla_creatures");
    /// ```
    #[must_use]
    pub fn get_parent_directory(&self) -> String {
        String::from(&self.parent_directory)
    }
    /// Set the name of the module the `InfoFile` was parsed in
    ///
    /// # Arguments
    ///
    /// * `arg` - The name of the module
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::path::Path;
    /// use dfraw_parser::{InfoFile, metadata::RawModuleLocation};
    ///
    /// let mut info_file = InfoFile::new("vanilla_creatures", RawModuleLocation::Vanilla, "vanilla_creatures");
    ///
    /// info_file.set_module_name("vanilla_creatures_2");
    /// assert_eq!(info_file.get_name(), "vanilla_creatures_2");
    /// ```
    pub fn set_module_name(&mut self, arg: &str) {
        self.name = String::from(arg);
    }
}
