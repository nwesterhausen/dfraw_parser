use std::path::{Path, PathBuf};

use tracing::{debug, error, info};
use walkdir::DirEntry;

use crate::{
    metadata::{ParserOptions, RawModuleLocation},
    utilities::subdirectories,
    InfoFile, ParserError,
};

/// The function `parse_module_info_files` parses module information files based on the provided options.
///
/// # Arguments:
///
/// * `options`: A reference to a `ParserOptions` struct, which contains various options for parsing
///   module information.
///
/// # Returns:
///
/// The function `parse_module_info_files` returns a `Vec<InfoFile>`.
///
/// # Errors
///
/// * `ParserError::Io` - If the `info.txt` file cannot be read, doesn't exist, or is an invalid `info.txt` file
///
pub fn parse_module_info_files(options: &ParserOptions) -> Result<Vec<InfoFile>, ParserError> {
    let mut results = Vec::new();

    if !options.locations_to_parse.is_empty() {
        // Parse each location
        if options
            .locations_to_parse
            .contains(&RawModuleLocation::Vanilla)
        {
            info!("Dispatching info.txt parse for vanilla raws");
            if let Some(vanilla_path) = options
                .locations
                .get_path_for_location(RawModuleLocation::Vanilla)
            {
                results.extend(parse_module_info_files_at_location(
                    &vanilla_path,
                    options.include_warnings_for_info_file_format,
                )?);
            } else {
                error!("No valid vanilla raws path found!");
            }
        }
        if options
            .locations_to_parse
            .contains(&RawModuleLocation::InstalledMods)
        {
            info!("Dispatching info.txt parse for vanilla raws");
            if let Some(installed_mods_path) = options
                .locations
                .get_path_for_location(RawModuleLocation::InstalledMods)
            {
                results.extend(parse_module_info_files_at_location(
                    &installed_mods_path,
                    options.include_warnings_for_info_file_format,
                )?);
            } else {
                error!("No valid vanilla raws path found!");
            }
        }
        if options
            .locations_to_parse
            .contains(&RawModuleLocation::Mods)
        {
            info!("Dispatching info.txt parse for workshop mod raws");
            if let Some(workshop_mods_path) = options
                .locations
                .get_path_for_location(RawModuleLocation::Mods)
            {
                results.extend(parse_module_info_files_at_location(
                    &workshop_mods_path,
                    options.include_warnings_for_info_file_format,
                )?);
            } else {
                error!("No valid workshop mod raws path found!");
            }
        }
    }

    // Parse any raw modules that are specified
    if !options.raw_modules_to_parse.is_empty() {
        // Parse all raw modules that are specified.
        for raw_module_path in options.raw_modules_to_parse.as_slice() {
            results.push(parse_module_info_file_in_module(
                raw_module_path,
                options.include_warnings_for_info_file_format,
            )?);
        }
    }

    // Parse any module info files that are specified directly
    if !options.module_info_files_to_parse.is_empty() {
        // Parse all module info files that are specified.
        for module_info_file_path in options.module_info_files_to_parse.as_slice() {
            results.push(InfoFile::parse(
                module_info_file_path,
                options.include_warnings_for_info_file_format,
            )?);
        }
    }

    Ok(results)
}

/// Parse the `info.txt` file at the `module_path` provided. Returns a `InfoFile` if successful.
///
/// Arguments:
///
/// * `module_path`: A reference to a path that points to the module directory.
///
/// Returns:
///
/// A `InfoFile` or `ParserError`
///
/// ## Errors
///
/// * `ParserError::Io` - If the `info.txt` file cannot be read, doesn't exist, or is an invalid `info.txt` file
pub fn parse_module_info_file_in_module<P: AsRef<Path>>(
    module_path: &P,
    warn_on_format_issue: bool,
) -> Result<InfoFile, ParserError> {
    let module_path: PathBuf = module_path.as_ref().to_path_buf();
    let module_info_file_path = module_path.join("info.txt");
    InfoFile::parse(&module_info_file_path, warn_on_format_issue)
}

/// The function `parse_module_info_files_at_location` takes a location path as input, retrieves a list
/// of subdirectories at that location, and parses each subdirectory's "info.txt" file into a
/// `InfoFile` struct, returning a vector of these structs.
///
/// # Arguments:
///
/// * `location_path`: the path to the directory where the module info files are.
///
/// # Returns:
///
/// The function `parse_module_info_files_at_location` returns a vector of `InfoFile` objects.
///
/// # Errors
///
/// * `ParserError::Io` - If we can't read the `info.txt` file properly
pub fn parse_module_info_files_at_location<P: AsRef<Path>>(
    location_path: &P,
    warn_on_format_issue: bool,
) -> Result<Vec<InfoFile>, ParserError> {
    let location_path: PathBuf = location_path.as_ref().to_path_buf();

    // Get a list of all subdirectories in the location
    let raw_modules_in_location: Vec<DirEntry> = subdirectories(location_path.clone())?;

    info!(
        "Found {} raw modules in {:?}",
        raw_modules_in_location.len(),
        location_path.file_name().unwrap_or_default(),
    );

    Ok(raw_modules_in_location
        .iter()
        .filter_map(|raw_module| {
            match parse_module_info_file_in_module(&raw_module.path(), warn_on_format_issue) {
                Ok(info_file) => Some(info_file),
                Err(e) => {
                    debug!("Skipping parsing module info file: {:?}", e);
                    None
                }
            }
        })
        .collect::<Vec<InfoFile>>())
}
