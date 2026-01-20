//! Utility functions that are used throughout the parser.
//!
//! Include some convenience functions for working with files and directories, as well as functions
//! for validating the provided `ParserOptions` struct.

use std::{
    collections::HashMap,
    fs::File,
    hash::BuildHasher,
    io::{BufWriter, Write},
    num::ParseIntError,
    path::{Path, PathBuf},
};

use itertools::Itertools;
use lazy_regex::regex;
use tracing::{debug, error, info, trace, warn};
use uuid::Uuid;
use walkdir::WalkDir;

use crate::{
    Creature, CreatureVariation, Entity, Graphic, Inorganic, MaterialTemplate, ParserError, Plant,
    SelectCreature, TilePage,
    metadata::{ParserOptions, RawModuleLocation},
    regex::VARIATION_ARGUMENT_RE,
    tags::ObjectType,
    traits::{CreatureVariationRequirements, IsEmpty, RawObject},
};

#[tracing::instrument]
/// Get a vec of subdirectories for a given directory
///
/// Using the `WalkDir` crate:
/// 1. create a new `WalkDir` for `directory`
/// 2. limit to immediate contents (`max_depth` and `min_depth` at 1)
/// 3. as an iterator..
///     4. `filter_map` into only non-error results
///     5. filter into only directories
/// 4. collect as a vec
///
/// Arguments:
///
/// * `directory`: The directory to search in
///
/// Returns:
///
/// A vector of all subdirectories as `walkdir::DirEntry`
pub fn subdirectories(directory: PathBuf) -> Result<Vec<walkdir::DirEntry>, ParserError> {
    let root_directory = match directory.canonicalize() {
        Ok(p) => p,
        Err(e) => {
            error!(
                "subdirectories: Unable to canonicalize directory {:?} \n{:?}",
                directory, e
            );
            return Err(ParserError::Io { source: e });
        }
    };

    Ok(WalkDir::new(root_directory)
        .max_depth(1)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| match e {
            Ok(entry) => {
                if entry.path().is_dir() {
                    Some(entry)
                } else {
                    debug!("subdirectories: Entry is not a directory {:?}", entry);
                    None
                }
            }
            Err(e) => {
                error!("subdirectories: Unable to read directory entry \n{:?}", e);
                None
            }
        })
        .collect())
}

/// If the parent directory of the given path exists, return the name of the parent directory, otherwise
/// return "!Unavailable!"
///
/// Arguments:
///
/// * `full_path`: The full path of the file.
///
/// Returns:
///
/// A String
pub fn get_parent_dir_name<P: AsRef<Path>>(full_path: &P) -> String {
    full_path.as_ref().parent().map_or_else(
        || String::from("!Unavailable!"),
        |parent_dir| String::from(parent_dir.file_name().unwrap_or_default().to_string_lossy()),
    )
}

/// "Given a path to a game directory, return a `PathBuf` to that directory if it exists and is a
/// directory, otherwise return an error."
///
/// The first thing we do is create a `PathBuf` from the provided `game_path`. We then check if the path
/// exists and is a directory. If it doesn't exist, we return an error. If it does exist, but isn't a
/// directory, we return an error. If it exists and is a directory, we return the `PathBuf`
///
/// Arguments:
///
/// * `game_path`: &str
///
/// Returns:
///
/// `Result<PathBuf, String>`
///
/// # Errors
///
/// * If the path doesn't exist
pub fn path_from_game_directory<P: AsRef<Path>>(game_path: &P) -> Result<PathBuf, String> {
    //1. "validate" folder is as expected
    let game_path = Path::new(game_path.as_ref());

    // Guard against invalid path
    if !game_path.exists() {
        return Err(String::from(
            "Provided game path for parsing doesn't exist!",
        ));
    }
    if !game_path.is_dir() {
        return Err(String::from("Game path needs to be a directory"));
    }

    // warn on no gamelog.txt
    if !game_path.join("gamelog.txt").exists() {
        warn!("Unable to find gamelog.txt in game directory. Is it valid?");
    }

    Ok(game_path.to_path_buf())
}

/// Save a vector of strings to a file, one string per line.
///
/// Arguments:
///
/// * `parsed_raws_string_vec`: String
/// * `out_filepath`: Path
pub fn write_json_string_vec_to_file<P: AsRef<Path>>(strings_vec: &[String], out_filepath: &P) {
    info!(
        "write_json_string_vec_to_file: Writing {} strings to file {:?}",
        strings_vec.len(),
        out_filepath.as_ref().display()
    );

    if strings_vec.is_empty() {
        warn!("write_json_string_vec_to_file: Provided string vector is empty!");
        return;
    }

    let out_file = match File::create(out_filepath) {
        Ok(f) => f,
        Err(e) => {
            error!(
                "write_json_string_vec_to_file: Unable to open {} for writing \n{:?}",
                out_filepath.as_ref().display(),
                e
            );
            return;
        }
    };

    let mut stream = BufWriter::new(out_file);
    let write_error = &format!(
        "write_json_string_vec_to_file: Unable to write to {}",
        out_filepath.as_ref().to_string_lossy()
    );

    if strings_vec.len() == 1 {
        match writeln!(stream, "{}", strings_vec.first().unwrap_or(&String::new())) {
            Ok(_x) => (),
            Err(e) => {
                error!("write_json_string_vec_to_file: {}\n{:?}", write_error, e);
                return;
            }
        };
        match stream.flush() {
            Ok(_x) => (),
            Err(e) => {
                error!("write_json_string_vec_to_file: {}\n{:?}", write_error, e);
            }
        };
        return;
    }

    let strings_vec = strings_vec.iter();
    // Write the first value with an open bracket '[' at the beginning
    // Write all next values with a comma ',' in front
    // Finish with a closing bracket ']'
    for (i, string) in strings_vec.enumerate() {
        match i {
            0 => match write!(stream, "[{string}") {
                Ok(_x) => (),
                Err(e) => {
                    error!("write_json_string_vec_to_file: {}\n{:?}", write_error, e);
                    return;
                }
            },
            _ => match write!(stream, ",{string}") {
                Ok(_x) => (),
                Err(e) => {
                    error!("write_json_string_vec_to_file: {}\n{:?}", write_error, e);
                    return;
                }
            },
        }
    }

    match writeln!(stream, "]") {
        Ok(_x) => (),
        Err(e) => {
            error!("write_json_string_vec_to_file: {}\n{:?}", write_error, e);
            return;
        }
    };

    match stream.flush() {
        Ok(_x) => (),
        Err(e) => {
            error!("write_json_string_vec_to_file: {}\n{:?}", write_error, e);
        }
    };
}

#[allow(clippy::too_many_lines)]
#[tracing::instrument]
/// The function `validate_options` validates the provided `ParserOptions` struct.
///
/// It checks that the provided paths exist and are valid. It will also expand any relative
/// paths to absolute paths.
///
/// It returns a `ParserOptions` struct if all paths are valid, otherwise it returns `None`.
///
/// Arguments:
///
/// * `options`: The `ParserOptions` struct to validate.
///
/// Returns:
///
/// An `Option<ParserOptions>` struct. None if options were invalid.
pub fn validate_options(options: &ParserOptions) -> Result<ParserOptions, ParserError> {
    // Copy the options into a new struct, before we validate the paths
    let mut validated_options = ParserOptions {
        attach_metadata_to_raws: options.attach_metadata_to_raws,
        locations_to_parse: options.locations_to_parse.clone(),
        object_types_to_parse: options.object_types_to_parse.clone(),
        skip_apply_copy_tags_from: options.skip_apply_copy_tags_from,
        skip_apply_creature_variations: options.skip_apply_creature_variations,
        locations: options.locations.clone(),
        ..Default::default()
    };

    validated_options.locations.init(false);

    // Guard against invalid path if locations are set
    if !validated_options.locations_to_parse.is_empty() {
        if validated_options.locations.get_df_directory().is_none() {
            return Err(ParserError::InvalidOptions(
                "Dwarf Fortress directory cannot be None".to_string(),
            ));
        }
        if (validated_options
            .locations_to_parse
            .contains(&RawModuleLocation::InstalledMods)
            || validated_options
                .locations_to_parse
                .contains(&RawModuleLocation::WorkshopMods))
            && validated_options
                .locations
                .get_user_data_directory()
                .is_none()
        {
            return Err(ParserError::InvalidOptions(
                "Dwarf Fortress user data directory cannot be None".to_string(),
            ));
        }
    }

    // Validate any raw file paths
    for raw_file_path in &options.raw_files_to_parse {
        if !raw_file_path.exists() {
            warn!(
                "options_validator: Discarding non-existent raw file:\n{}",
                raw_file_path.display()
            );
        } else if !raw_file_path.is_file() {
            warn!(
                "options_validator: Discarding raw file because it isn't a file:\n{}",
                raw_file_path.display()
            );
        } else {
            // Add the canonicalized path to the raw file
            let raw_file_path = raw_file_path.canonicalize().unwrap_or_else(|e| {
                warn!(
                    "options_validator: Discarding raw file that cannot be canonicalized:\n{:?}",
                    e
                );
                raw_file_path.clone()
            });
            validated_options.raw_files_to_parse.push(raw_file_path);
        }
    }

    // Validate any raw module paths
    for raw_module_path in &options.raw_modules_to_parse {
        if !raw_module_path.exists() {
            warn!(
                "options_validator: Discarding non-existent raw module directory:\n{}",
                raw_module_path.display()
            );
        } else if !raw_module_path.is_dir() {
            warn!(
                "options_validator: Discarding raw module directory because it isn't a directory:\n{}",
                raw_module_path.display()
            );
        } else {
            // Add the canonicalized path to the module
            let raw_module_path = raw_module_path.canonicalize().unwrap_or_else(|e| {
              warn!(
                  "options_validator: Discarding raw module directory path that cannot be canonicalized:\n{:?}",
                  e
              );
              raw_module_path.clone()
          });
            validated_options.raw_modules_to_parse.push(raw_module_path);
        }
    }

    // Validate any legends export paths
    for legends_export_path in &options.legends_exports_to_parse {
        if !legends_export_path.exists() {
            warn!(
                "options_validator: Discarding non-existent legends export:\n{}",
                legends_export_path.display()
            );
        } else if !legends_export_path.is_file() {
            warn!(
                "options_validator: Discarding legends export because it isn't a file:\n{}",
                legends_export_path.display()
            );
        } else {
            // Add the canonicalized path to the legends export
            let legends_export_path = legends_export_path.canonicalize().unwrap_or_else(|e| {
              warn!(
                  "options_validator: Discarding legends export path that cannot be canonicalized\n{:?}",
                  e
              );
              legends_export_path.clone()
          });
            validated_options
                .legends_exports_to_parse
                .push(legends_export_path);
        }
    }

    // Validate any module info file paths
    for module_info_file_path in &options.module_info_files_to_parse {
        if !module_info_file_path.exists() {
            warn!(
                "options_validator: Discarding non-existent module info file:\n{}",
                module_info_file_path.display()
            );
        } else if !module_info_file_path.is_file() {
            warn!(
                "options_validator: Discarding module info file because it isn't a file:\n{}",
                module_info_file_path.display()
            );
        } else {
            // Add the canonicalized path to the module info file
            let module_info_file_path = module_info_file_path.canonicalize().unwrap_or_else(|e| {
              warn!(
                  "options_validator: Discarding module info file path that cannot be canonicalized\n{:?}",
                  e
              );
              module_info_file_path.clone()
          });
            validated_options
                .module_info_files_to_parse
                .push(module_info_file_path);
        }
    }

    Ok(validated_options)
}

/// The function `get_only_creatures_from_raws` takes a slice of `RawObject` trait objects and returns a
/// vector containing only the objects that are of type `DFCreature`.
///
/// Arguments:
///
/// * `all_raws`: A slice of boxed objects that implement the `RawObject` trait.
///
/// Returns:
///
/// a vector of `DFCreature` objects.
#[must_use]
pub fn get_only_creatures_from_raws(all_raws: &[Box<dyn RawObject>]) -> Vec<Creature> {
    all_raws
        .iter()
        .filter(|r| r.get_type() == ObjectType::Creature)
        .map(|r| r.as_any().downcast_ref::<Creature>())
        .map(|r| r.unwrap_or(&Creature::default()).clone())
        .collect::<Vec<Creature>>()
}

/// The function `get_only_select_creatures_from_raws` filters a slice of raw objects and returns a
/// vector containing only the objects of type `SelectCreature`.
///
/// Arguments:
///
/// * `all_raws`: A slice of boxed objects that implement the `RawObject` trait.
///
/// Returns:
///
/// a vector of `SelectCreature` objects.
#[must_use]
pub fn get_only_select_creatures_from_raws(all_raws: &[Box<dyn RawObject>]) -> Vec<SelectCreature> {
    all_raws
        .iter()
        .filter(|r| r.get_type() == ObjectType::SelectCreature)
        .map(|r| r.as_any().downcast_ref::<SelectCreature>())
        .map(|r| r.unwrap_or(&SelectCreature::default()).clone())
        .collect::<Vec<SelectCreature>>()
}

/// `try_get_file` attempts to open a file at the given path and returns a `File` if successful.
///
/// Arguments:
///
/// * `file_path`: A path to the file to open.
///
/// Returns:
///
/// An `Option<File>`. None if the file doesn't exist or isn't a file.
///
/// # Errors
///
/// * `ParserError::Io` if the file cannot be opened or doesn't exist
pub fn try_get_file<P: AsRef<Path>>(file_path: &P) -> Result<File, ParserError> {
    // Validate file exists
    if !file_path.as_ref().exists() {
        debug!(
            "try_get_file: Path doesn't exist:\n{}",
            file_path.as_ref().display()
        );
        return Err(ParserError::Io {
            source: std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "try_get_file: Path doesn't exist {}",
                    file_path.as_ref().display()
                ),
            ),
        });
    }
    if !file_path.as_ref().is_file() {
        debug!(
            "try_get_file: Path does not point to a file:\n{}",
            file_path.as_ref().display(),
        );
        return Err(ParserError::Io {
            source: std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "try_get_file: Path does not point to a file {}",
                    file_path.as_ref().display()
                ),
            ),
        });
    }

    // Open the file
    match File::open(file_path) {
        Ok(file) => Ok(file),
        Err(e) => {
            debug!(
                "try_get_file: Unable to open file {}\n{:?}",
                file_path.as_ref().display(),
                e
            );
            Err(ParserError::Io { source: e })
        }
    }
}

/// Create a summary of the parsed raws.
///
/// Summarizes the parsed raws by object type.
///
/// Arguments:
///
/// * `raws`: A slice of boxed objects that implement the `RawObject` trait.
///
/// Returns:
///
/// A `HashMap<ObjectType, usize>` where the key is the object type and the value is the number of
/// objects of that type.
#[must_use]
pub fn summarize_raws(raws: &[Box<dyn RawObject>]) -> HashMap<ObjectType, usize> {
    let mut summary: std::collections::HashMap<ObjectType, usize> =
        std::collections::HashMap::new();

    for raw in raws {
        let count = summary.entry(raw.get_type()).or_insert(0);
        *count += 1;
    }

    summary
}

/// Logs a summary of the parsed raws to the console via `tracing::info!`
///
/// Arguments:
///
/// * `summary`: A `HashMap<ObjectType, usize>` where the key is the object type and the value is the number of
///   objects of that type.
pub fn log_summary<S: BuildHasher>(summary: &HashMap<ObjectType, usize, S>) {
    let total = summary.values().sum::<usize>();

    info!("Summary of parsed raws:");
    for (object_type, count) in summary {
        info!("\t{count}\t{object_type}");
    }
    info!("Total: {total}");
}

/// It takes a slice of strings, parses the first two strings as unsigned 16-bit integers, and returns a
/// two-element array of unsigned 16-bit integers
///
/// Arguments:
///
/// * `split`: &[&str] - This is the array of strings that we're going to parse.
///
/// Returns:
///
/// A Result<[u16; 2], `ParseIntError`>
///
/// # Errors
///
/// Will return a `ParseIntError` if the string cannot be parsed into two unsigned 16-bit integers.
pub fn parse_min_max_range_from_vec(split: &[&str]) -> Result<[u32; 2], ParseIntError> {
    let min: u32 = match split.first().unwrap_or(&"").parse() {
        Ok(n) => n,
        Err(e) => {
            error!("min_value parsing error\n{:?}", e);
            return Err(e);
        }
    };
    let max: u32 = match split.get(1).unwrap_or(&"").parse() {
        Ok(n) => n,
        Err(e) => {
            error!("max_value parsing error\n{:?}", e);
            return Err(e);
        }
    };
    Ok([min, max])
}

/// The function `parse_min_max_range` takes a string input and splits it by a colon, then calls another
/// function to parse the resulting vector into an array of two unsigned 16-bit integers.
///
/// Arguments:
///
/// * `value`: A string representing a range of values in the format "min:max".
///
/// Returns:
///
/// The function `parse_min_max_range` returns a `Result<[u16; 2], ParseIntError>`.
///
/// # Errors
///
/// Will return a `ParseIntError` if the string cannot be parsed into two unsigned 16-bit integers.
pub fn parse_min_max_range(value: &str) -> Result<[u32; 2], ParseIntError> {
    let split: Vec<&str> = value.split(':').collect::<Vec<&str>>();
    parse_min_max_range_from_vec(&split)
}

/// The function `clone_raw_vector_with_purge` clones a vector of raw objects, excluding those with
/// specified object IDs to purge.
///
/// Arguments:
///
/// * `all_raws`: A slice of `Box<dyn RawObject>`, which represents a collection of raw objects.
/// * `object_ids_to_purge`: A slice of string references representing the object IDs that need to be
///   purged from the `all_raws` vector.
///
/// Returns:
///
/// The function `clone_raw_vector_with_purge` returns a vector of boxed dynamic objects (`Vec<Box<dyn
/// RawObject>>`).
pub fn clone_raw_vector_with_purge(
    all_raws: &[Box<dyn RawObject>],
    object_ids_to_purge: &[Uuid],
) -> Vec<Box<dyn RawObject>> {
    let mut new_raws: Vec<Box<dyn RawObject>> = Vec::new();

    for raw in all_raws {
        if object_ids_to_purge.contains(&raw.get_object_id()) {
            trace!("clone_raw_vector purging {}", raw.get_object_id());
        } else {
            // Match the object type, downcast and clone into a new box in new_raws
            new_raws.push(clone_raw_object_box(raw));
        }
    }
    new_raws
}

#[allow(clippy::borrowed_box)]
/// The function `clone_raw_object_box` clones a boxed object based on its type.
///
/// Arguments:
///
/// * `box_ref`: A reference to a boxed object implementing the `RawObject` trait.
///
/// Returns:
///
/// The function `clone_raw_object_box` returns a `Box<dyn RawObject>`.
pub fn clone_raw_object_box(box_ref: &Box<dyn RawObject>) -> Box<dyn RawObject> {
    match box_ref.get_type() {
        ObjectType::Creature => {
            let temp_creature = box_ref
                .as_any()
                .downcast_ref::<Creature>()
                .unwrap_or(&Creature::empty())
                .clone();
            Box::new(temp_creature)
        }
        ObjectType::SelectCreature => {
            let temp_select_creature = box_ref
                .as_any()
                .downcast_ref::<SelectCreature>()
                .unwrap_or(&SelectCreature::empty())
                .clone();
            Box::new(temp_select_creature)
        }
        ObjectType::CreatureVariation => {
            let temp_creature_variation = box_ref
                .as_any()
                .downcast_ref::<CreatureVariation>()
                .unwrap_or(&CreatureVariation::empty())
                .clone();
            Box::new(temp_creature_variation)
        }
        ObjectType::Plant => {
            let temp_plant = box_ref
                .as_any()
                .downcast_ref::<Plant>()
                .unwrap_or(&Plant::empty())
                .clone();
            Box::new(temp_plant)
        }
        ObjectType::Inorganic => {
            let temp_inorganic = box_ref
                .as_any()
                .downcast_ref::<Inorganic>()
                .unwrap_or(&Inorganic::empty())
                .clone();
            Box::new(temp_inorganic)
        }
        ObjectType::MaterialTemplate => {
            let temp_material_template = box_ref
                .as_any()
                .downcast_ref::<MaterialTemplate>()
                .unwrap_or(&MaterialTemplate::empty())
                .clone();
            Box::new(temp_material_template)
        }
        ObjectType::Graphics => {
            let temp_graphic = box_ref
                .as_any()
                .downcast_ref::<Graphic>()
                .unwrap_or(&Graphic::empty())
                .clone();
            Box::new(temp_graphic)
        }
        ObjectType::TilePage => {
            let temp_tile_page = box_ref
                .as_any()
                .downcast_ref::<TilePage>()
                .unwrap_or(&TilePage::empty())
                .clone();
            Box::new(temp_tile_page)
        }
        ObjectType::Entity => {
            let temp_entity = box_ref
                .as_any()
                .downcast_ref::<Entity>()
                .unwrap_or(&Entity::empty())
                .clone();
            Box::new(temp_entity)
        }
        _ => {
            warn!(
                "clone_raw_object_box has an unhandled object type: {:?}",
                box_ref.get_type()
            );
            Box::new(Creature::empty())
        }
    }
}

/// The function `with_limit_and_page` takes a slice of `RawObject` objects, a limit, and a page number,
/// and returns a new vector containing a subset of the original objects based on the limit and page
/// number.
///
/// Arguments:
///
/// * `all_raws`: A slice of boxed objects that implement the `RawObject` trait.
/// * `limit`: The `limit` parameter specifies the maximum number of items to be included in each page
///   of the result.
/// * `page`: The `page` parameter represents the page number of the data you want to retrieve. It is
///   used to calculate the starting and ending positions of the data based on the `limit` parameter. The
///   first page is represented by page number 1, so if you want to retrieve data from the first page
///
/// Returns:
///
/// a vector of boxed dynamic objects (`Vec<Box<dyn RawObject>>`).
pub fn clone_raw_vector_with_limit_and_page(
    all_raws: &[Box<dyn RawObject>],
    limit: usize,
    page: usize,
) -> Vec<Box<dyn RawObject>> {
    let mut new_raws: Vec<Box<dyn RawObject>> = Vec::new();
    // Page 0 is the first page, so we need to subtract 1 from the page number
    // But this guards against someone sending an invalid page number of 0
    let page = if page > 0 { page - 1 } else { page };
    let start = limit * page;
    let end = start + limit;

    debug!("with_limit_and_page start: {start}, end: {end}, page: {page}");

    for (pos, raw) in all_raws.iter().enumerate() {
        if pos >= start && pos < end {
            new_raws.push(clone_raw_object_box(raw));
        }
    }
    new_raws
}

/// Apply copy tags from one creature to another.
///
/// # Arguments
///
/// * `all_raws` - The list of all raw objects.
///
/// # Side Effects
///
/// Updates the list of raw objects with the applied copy tags.
///
/// # Notes
///
/// This function is called after all raw objects have been parsed and before any other processing is done.
#[allow(clippy::too_many_lines)]
pub fn apply_copy_tags_from(all_raws: &mut Vec<Box<dyn RawObject>>) {
    let untouched_raws: Vec<_> = all_raws.iter().map(clone_raw_object_box).collect();

    let creatures_with_copy_tags_from: Vec<Creature> = {
        untouched_raws
            .iter()
            .filter(|r| r.get_type() == ObjectType::Creature)
            .filter_map(|r| {
                let creature = r
                    .as_any()
                    .downcast_ref::<Creature>()
                    .unwrap_or(&Creature::empty())
                    .clone();

                if creature.get_copy_tags_from() == "" {
                    None
                } else {
                    Some(creature)
                }
            })
            .collect()
    };
    let source_creature_identifier_list: Vec<String> = creatures_with_copy_tags_from
        .iter()
        .map(|c| c.get_copy_tags_from().to_lowercase())
        .unique()
        .collect();
    info!(
        "updating {} of {} raws from {} creatures",
        creatures_with_copy_tags_from.len(),
        all_raws.len(),
        source_creature_identifier_list.len()
    );

    // Build a list of unique creature identifiers to target, based on the apply_copy_tags_from list.
    let source_creatures: Vec<Creature> = untouched_raws
        .iter()
        .filter_map(|raw| {
            if raw.get_type() == ObjectType::Creature
                && source_creature_identifier_list.contains(&raw.get_identifier().to_lowercase())
            {
                Some(
                    raw.as_any()
                        .downcast_ref::<Creature>()
                        .unwrap_or(&Creature::empty())
                        .clone(),
                )
            } else {
                None
            }
        })
        .collect::<Vec<Creature>>();

    // The outside loop iterates over the source creatures, which we will use to copy tags from
    // Inside the loop, we find which creatures have the source creature's identifier in their
    // copy_tags_from field, and then apply the source creature's tags to those creatures.
    // Then we put the updated creatures into the new_creatures vector, which will be used to
    // replace the old creatures in the all_raws vector.

    let mut new_creatures: Vec<Creature> = Vec::new();
    for source_creature in source_creatures {
        let target_creatures: Vec<Creature> = creatures_with_copy_tags_from
            .iter()
            .filter(|c| {
                c.get_copy_tags_from().to_lowercase()
                    == source_creature.get_identifier().to_lowercase()
            })
            .cloned()
            .collect::<Vec<Creature>>();

        for target_creature in target_creatures {
            new_creatures.push(Creature::copy_tags_from(&target_creature, &source_creature));
        }
    }

    info!("copied tags to {} creatures", new_creatures.len());

    let mut object_ids_to_purge: Vec<Uuid> = Vec::new();

    object_ids_to_purge.extend(new_creatures.iter().map(RawObject::get_object_id));

    let mut new_raws: Vec<Box<dyn RawObject>> =
        clone_raw_vector_with_purge(all_raws, object_ids_to_purge.as_slice());

    if all_raws.len() < new_raws.len() {
        warn!(
            "post purge has {} raws, but started with {}",
            new_raws.len(),
            all_raws.len()
        );
    } else {
        info!("purged {} raws", all_raws.len() - new_raws.len());
    }

    for creature in new_creatures {
        new_raws.push(Box::new(creature));
    }

    if all_raws.len() < new_raws.len() {
        warn!(
            "finished with {} raws, but started with {}",
            new_raws.len(),
            all_raws.len()
        );
    } else {
        info!(
            "finished with {} raws (net {} lost)",
            new_raws.len(),
            all_raws.len() - new_raws.len()
        );
    }

    *all_raws = new_raws;
}

/// Function to absorb `SELECT_CREATURE` records into the Creature records.
///
/// # Parameters
///
/// * `all_raws` - The raw objects to absorb the `SELECT_CREATURE` records into.
///
/// # Panics
///
/// This function will panic if the raw object is not a Creature.
///
/// # Notes
///
/// This function will remove the `SELECT_CREATURE` records from the raws after applying them to the Creature records.
///
/// # Side Effects
///
/// This function will modify the `all_raws` parameter.
#[allow(clippy::too_many_lines)]
pub fn absorb_select_creature(all_raws: &mut Vec<Box<dyn RawObject>>) {
    let all_select_creatures = { get_only_select_creatures_from_raws(all_raws) };

    info!(
        "looking at {} SELECT_CREATURE of {} raws",
        all_select_creatures.len(),
        all_raws.len()
    );

    let mut object_ids_to_purge: Vec<Uuid> = Vec::new();
    let mut new_creatures: Vec<Creature> = Vec::new();
    let mut target_creature_identifiers: Vec<&str> = Vec::new();

    for select_creature in all_select_creatures.as_slice() {
        if target_creature_identifiers.contains(&select_creature.get_identifier()) {
            continue;
        }
        target_creature_identifiers.push(select_creature.get_identifier());
    }

    for raw in &*all_raws {
        if raw.get_type() == ObjectType::Creature
            && target_creature_identifiers.contains(&raw.get_identifier())
        {
            let select_creature_vec: Vec<SelectCreature> = all_select_creatures
                .iter()
                .filter(|r| r.get_identifier() == raw.get_identifier())
                .cloned()
                .collect();

            if select_creature_vec.is_empty() {
                // Skip this creature if there are no select_creature records for it
                continue;
            }

            let mut temp_creature = raw
                .as_any()
                .downcast_ref::<Creature>()
                .unwrap_or(&Creature::empty())
                .clone();
            temp_creature.extend_select_creature_variation(select_creature_vec);

            let object_id = raw.get_object_id();
            object_ids_to_purge.push(object_id);

            if !temp_creature.is_empty() {
                new_creatures.push(temp_creature.clone());
            }
        }
    }

    if new_creatures.is_empty() {
        return;
    }

    object_ids_to_purge.extend(
        new_creatures
            .iter()
            .flat_map(Creature::get_child_object_ids),
    );

    let mut new_raws: Vec<Box<dyn RawObject>> =
        clone_raw_vector_with_purge(all_raws.as_slice(), object_ids_to_purge.as_slice());

    for creature in new_creatures {
        new_raws.push(Box::new(creature));
    }

    info!(
        "finished with {} raws (some {} purged)",
        new_raws.len(),
        all_raws.len() - new_raws.len()
    );

    *all_raws = new_raws;
}

/// Replaces all instances of `!ARGn` with the corresponding argument.
///
/// ## Arguments
///
/// * `string` - The string to replace the arguments in.
/// * `args` - The arguments to replace in the string.
///
/// ## Returns
///
/// * `String` - The string with the arguments replaced.
pub fn replace_args_in_string(string: &str, args: &[&str]) -> String {
    VARIATION_ARGUMENT_RE
        .replace_all(string, |caps: &regex::Captures| {
            argument_as_string(caps, args)
        })
        .to_string()
}

/// ADD or NEW tags can simply be applied by the parsing logic that already exists.
///
/// ## Arguments
///
/// * `creature` - The creature to apply the tag to.
/// * `tag` - The tag to apply.
/// * `value` - The value to apply to the tag.
pub fn apply_new_tag(creature: &mut Creature, tag: &str, value: Option<&str>) {
    (creature as &mut dyn CreatureVariationRequirements)
        .add_tag_and_value(tag, value.unwrap_or_default());
}

/// Removes a tag from a creature.
///
/// ## Arguments
///
/// * `creature` - The creature to remove the tag from.
/// * `tag` - The tag to remove.
pub fn remove_tag(creature: &mut Creature, tag: &str) {
    (creature as &mut dyn CreatureVariationRequirements).remove_tag(tag);
}

/// Converts a tag on a creature.
pub fn convert_tag(
    creature: &mut Creature,
    tag: &str,
    target: Option<&str>,
    replacement: Option<&str>,
) {
    if let Some(target) = target {
        if let Some(replacement) = replacement {
            tracing::trace!(
                "Converting tag {}:{} to {}:{} on creature {}",
                tag,
                target,
                tag,
                replacement,
                creature.get_identifier()
            );
            // Convert the tag to the target value.
            (creature as &mut dyn CreatureVariationRequirements).remove_tag_and_value(tag, target);
            (creature as &mut dyn CreatureVariationRequirements)
                .add_tag_and_value(tag, replacement);
        } else {
            tracing::trace!(
                "Converting tag {}:{} to {}:{} on creature {}",
                tag,
                target,
                replacement.unwrap_or_default(),
                target,
                creature.get_identifier(),
            );
            // Convert the tag to the target value.
            (creature as &mut dyn CreatureVariationRequirements).remove_tag_and_value(tag, target);
            (creature as &mut dyn CreatureVariationRequirements).add_tag_and_value(tag, target);
        }
    } else {
        tracing::trace!(
            "Converting tag {} to {} on creature {}",
            tag,
            replacement.unwrap_or_default(),
            creature.get_identifier()
        );
        // Convert the tag to the replacement value.
        (creature as &mut dyn CreatureVariationRequirements).remove_tag(tag);
        (creature as &mut dyn CreatureVariationRequirements)
            .add_tag(replacement.unwrap_or_default());
    }
}

/// Returns the argument which matches the given capture group.
/// This expects you to be capturing based on the regex in `VARIATION_ARGUMENT_RE`.
///
/// That way it will match `!ARGn` and `!ARGnn` and `!ARGnnn` and replace with the corresponding
/// argument.
///
/// ## Arguments
///
/// * `caps` - The capture group to get the argument name from.
/// * `args` - The arguments to get the argument from.
///
/// ## Returns
///
/// * `String` - The argument which matches the given capture group.
pub fn argument_as_string(caps: &regex::Captures, args: &[&str]) -> String {
    if let Some(index) = caps.get(1) {
        let index = index.as_str().parse::<usize>().unwrap_or_default();
        if let Some(argument_value) = args.get(index - 1) {
            return (*argument_value).to_string();
        }
    }
    if let Some(arg) = caps.get(0) {
        tracing::warn!(
            "Creature Variation Argument is invalid. Argument captured: '{}'",
            arg.as_str()
        );
        return arg.as_str().to_string();
    }
    String::new()
}

/// Apply creature variations to creatures.
///
/// # Arguments
///
/// * `all_raws` - The list of all raw objects.
///
/// # Side Effects
///
/// Updates the list of raw objects with the applied creature variations.
///
/// # Notes
///
/// This function is called after all raw objects have been parsed and before any other processing is done.
pub fn apply_creature_variations(all_raws: &mut [Box<dyn RawObject>]) {
    let creature_variations: Vec<CreatureVariation> = all_raws
        .iter()
        .filter(|r| r.get_type() == ObjectType::CreatureVariation)
        .filter_map(|r| r.as_any().downcast_ref::<CreatureVariation>())
        .cloned()
        .collect();

    let creatures: Vec<Creature> = all_raws
        .iter()
        .filter(|r| r.get_type() == ObjectType::Creature)
        .filter_map(|r| r.as_any().downcast_ref::<Creature>())
        .cloned()
        .collect();

    let mut updated_creatures: Vec<Creature> = Vec::new();

    // Go through all creatures and if they have a variation, apply it.
    for creature in creatures {
        // Check variations against known creature variations
        for variation in creature.get_variations_to_apply() {
            if let Some(updated_creature) = singularly_apply_creature_variation(
                &creature,
                variation,
                creature_variations.as_slice(),
            ) {
                updated_creatures.push(updated_creature);
            }
        }
    }

    // Replace creatures with updated creatures
    for updated_creature in updated_creatures {
        let Some(index) = all_raws
            .iter()
            .position(|r| r.get_object_id() == updated_creature.get_object_id())
        else {
            warn!(
                "Failed to find creature {} to replace with updated creature",
                updated_creature.get_object_id()
            );
            continue;
        };

        #[allow(clippy::indexing_slicing)]
        let _ = std::mem::replace(&mut all_raws[index], Box::new(updated_creature));
    }
}

/// Apply a single creature variation to a creature.
///
/// # Arguments
///
/// * `creature` - The creature to apply the variation to.
/// * `variation` - The variation to apply.
/// * `creature_variations` - The list of creature variations to apply.
///
/// # Returns
///
/// The updated creature if the variation was successfully applied, otherwise None.
#[must_use]
pub fn singularly_apply_creature_variation(
    creature: &Creature,
    variation: &str,
    creature_variations: &[CreatureVariation],
) -> Option<Creature> {
    // The variation comes back like this:
    // "STANDARD_WALK_CRAWL_GAITS:6561:6115:5683:1755:7456:8567"
    // We need to split it into the variation id and the args (if any)
    let variation_parts: Vec<&str> = variation.split(':').collect();
    let variation_identifier = *variation_parts.first().unwrap_or(&"");
    let variation_args = variation_parts.get(1..).unwrap_or(&[]);

    let Some(creature_variation) = creature_variations
        .iter()
        .find(|r| r.get_identifier().to_uppercase() == variation_identifier.to_uppercase())
        .cloned()
    else {
        warn!("Failed to find creature variation {}", variation_identifier);
        debug!("args: {:?}", variation_args);
        return None;
    };

    let mut updated_creature = creature.clone();
    debug!(
        "Applying variation {} to {}",
        variation_identifier,
        creature.get_identifier()
    );

    // Reset to `ALL` caste; some variations contain caste-specific rules but do not specify a caste
    updated_creature.select_caste("ALL");

    // Apply variation to creature
    for rule in creature_variation.get_rules() {
        rule.apply(&mut updated_creature, variation_args);
    }

    Some(updated_creature)
}
