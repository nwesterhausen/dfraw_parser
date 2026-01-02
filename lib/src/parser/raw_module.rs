use std::ffi::OsStr;
use std::path::Path;

use tracing::{debug, info};
use walkdir::WalkDir;

use crate::{
    metadata::{ObjectType, ParserOptions},
    reader::{parse_raw_file, FileParseResult, UnprocessedRaw},
    traits::RawObject,
    InfoFile, ParserError,
};

#[allow(clippy::too_many_lines)]
/// The `parse_module` function parses raw files from a module directory and returns a vector of parsed
/// objects.
///
/// Arguments:
///
/// * `module_path`: the path to the module directory that contains the raw files to parse.
/// * `options`: The parsing options which determine what and how to parse the raw files.
///
/// Returns:
///
/// The function `parse_module` returns a vector of boxed dynamic objects (`Vec<Box<dyn RawObject>>`).
///
/// # Errors
///
/// * `ParserError::Io` - If we can't read the raws from the Dwarf Fortress directory (various reasons)
pub fn parse_module<P: AsRef<Path>>(
    module_path: &P,
    options: &ParserOptions,
) -> Result<FileParseResult, ParserError> {
    // Get information from the module info file
    let module_info_file_path = module_path.as_ref().join("info.txt");
    let module_info_file = match InfoFile::parse(
        &module_info_file_path,
        options.include_warnings_for_info_file_format,
    ) {
        Ok(info_file) => info_file,
        Err(e) => {
            return Err(e);
        }
    };

    // Get a list of all raw files in the module
    let objects_path = module_path.as_ref().join("objects");
    let graphics_path = module_path.as_ref().join("graphics");

    let mut parse_objects = true;
    let mut parse_graphics = options
        .object_types_to_parse
        .contains(&ObjectType::Graphics);

    if !objects_path.exists() {
        debug!(
            "Ignoring objects directory in {:?} because it does not exist",
            module_path.as_ref().file_name().unwrap_or_default(),
        );
        parse_objects = false;
    }

    if parse_objects && !objects_path.is_dir() {
        debug!(
            "Ignoring objects directory in {:?} because it is not a directory",
            module_path.as_ref().file_name().unwrap_or_default(),
        );
        parse_objects = false;
    }

    if !graphics_path.exists() {
        debug!(
            "Ignoring graphics directory in {:?} because it does not exist",
            module_path.as_ref().file_name().unwrap_or_default(),
        );
        parse_graphics = false;
    }

    if parse_graphics && !graphics_path.is_dir() {
        debug!(
            "Ignoring graphics directory in {:?} because it is not a directory",
            module_path.as_ref().file_name().unwrap_or_default(),
        );
        parse_graphics = false;
    }

    // Guard against having nothing to parse.
    if !parse_graphics && !parse_objects {
        return Ok(FileParseResult {
            parsed_raws: Vec::new(),
            unprocessed_raws: Vec::new(),
        });
    }

    let mut results: Vec<Box<dyn RawObject>> = Vec::new();
    let mut unprocessed_raws: Vec<UnprocessedRaw> = Vec::new();

    // Parse the objects
    if parse_objects {
        info!(
            "Parsing objects for {} v{}",
            module_info_file.get_identifier(),
            module_info_file.get_version(),
        );
        for entry in WalkDir::new(objects_path)
            .into_iter()
            .filter_map(std::result::Result::ok)
        {
            if entry.file_type().is_file() {
                let file_path = entry.path();
                let file_name = file_path.file_name().unwrap_or_default();
                let file_name_str = file_name.to_str().unwrap_or_default();

                if Path::new(file_name_str).extension() == Some(OsStr::new("txt")) {
                    match parse_raw_file(&file_path, options) {
                        Ok(mut file_parse_results) => {
                            results.append(&mut file_parse_results.parsed_raws);
                            unprocessed_raws.append(&mut file_parse_results.unprocessed_raws);
                        }
                        Err(e) => {
                            debug!("Skipping parsing objects: {:?}", e);
                        }
                    }
                }
            }
        }
    }

    // Parse the graphics
    if parse_graphics {
        info!(
            "Parsing graphics for {} v{}",
            module_info_file.get_identifier(),
            module_info_file.get_version(),
        );
        for entry in WalkDir::new(graphics_path)
            .into_iter()
            .filter_map(std::result::Result::ok)
        {
            if entry.file_type().is_file() {
                let file_path = entry.path();
                let file_name = file_path.file_name().unwrap_or_default();
                let file_name_str = file_name.to_str().unwrap_or_default();

                if Path::new(file_name_str).extension() == Some(OsStr::new("txt")) {
                    match parse_raw_file(&file_path, options) {
                        Ok(mut graphics) => {
                            results.append(&mut graphics.parsed_raws);
                            unprocessed_raws.append(&mut graphics.unprocessed_raws);
                        }
                        Err(e) => {
                            debug!("Skipping parsing graphics: {:?}", e);
                        }
                    }
                }
            }
        }
    }

    Ok(FileParseResult {
        parsed_raws: results,
        unprocessed_raws,
    })
}
