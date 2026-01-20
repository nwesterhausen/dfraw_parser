use std::path::Path;

use tracing::{error, info};

use crate::{
    Creature, CreatureVariation, ParserError, legends_export,
    metadata::{ParserOptions, RawModuleLocation},
    parser::{parse_location, parse_module},
    reader::{UnprocessedRaw, parse_raw_file},
    tags::ObjectType,
    traits::RawObject,
    utilities::{clone_raw_object_box, log_summary, summarize_raws, validate_options},
};

use super::{ParseResult, info_file::parse_module_info_files};

#[allow(clippy::too_many_lines)]
/// Given the supplied `ParserOptions`, parse the raws and return a vector of boxed dynamic raw objects.
///
/// Note: This is unable to parse the info.txt file for a module. Use `parse_module_info_file` for that.
///
/// # Arguments
///
/// * `options` - A reference to a `ParserOptions` struct that contains the parsing options.
///
/// # Returns
///
/// A vector of boxed dynamic raw objects.
///
/// # Errors
///
/// * `ParserError::Io` - If we can't read the raws from the Dwarf Fortress directory (various reasons)
/// * `ParserError::InvalidPath` - If the path to the Dwarf Fortress directory is invalid
///
/// Other errors which are returned from the called functions within this function are not propagated, because the
/// only "full" blocker is if the Dwarf Fortress directory is invalid.
///
#[allow(clippy::cognitive_complexity)]
pub fn parse(options: &ParserOptions) -> Result<ParseResult, ParserError> {
    // Guard against invalid paths
    let options = validate_options(options)?;

    let mut results = ParseResult {
        raws: Vec::new(),
        modules: Vec::new(),
    };
    let mut unprocessed_raws: Vec<UnprocessedRaw> = Vec::new();

    // Locations can only contain the predefined locations.
    if !options.locations_to_parse.is_empty() {
        info!("{:?}", options.locations);

        // Parse each location
        if options
            .locations_to_parse
            .contains(&RawModuleLocation::Vanilla)
        {
            info!("Dispatching parse for vanilla raws");
            if let Some(vanilla_path) = options
                .locations
                .get_path_for_location(RawModuleLocation::Vanilla)
            {
                let parsed_raws = parse_location(&vanilla_path, &options)?;
                results.raws.extend(parsed_raws.parsed_raws);
                unprocessed_raws.extend(parsed_raws.unprocessed_raws);
            } else {
                error!("No valid vanilla raws path found!");
            }
        }
        if options
            .locations_to_parse
            .contains(&RawModuleLocation::InstalledMods)
        {
            info!("Dispatching parse for installed mods");
            if let Some(installed_mods_path) = options
                .locations
                .get_path_for_location(RawModuleLocation::InstalledMods)
            {
                let parsed_raws = parse_location(&installed_mods_path, &options)?;
                results.raws.extend(parsed_raws.parsed_raws);
                unprocessed_raws.extend(parsed_raws.unprocessed_raws);
            } else {
                error!("No valid installed mods path found!");
            }
        }
        if options
            .locations_to_parse
            .contains(&RawModuleLocation::WorkshopMods)
        {
            info!("Dispatching parse for workshop/downloaded mods");
            if let Some(workshop_mods_path) = options
                .locations
                .get_path_for_location(RawModuleLocation::WorkshopMods)
            {
                let parsed_raws = parse_location(&workshop_mods_path, &options)?;
                results.raws.extend(parsed_raws.parsed_raws);
                unprocessed_raws.extend(parsed_raws.unprocessed_raws);
            } else {
                error!("No valid workshop mods path found!");
            }
        }
    }

    if !options.raw_modules_to_parse.is_empty() {
        // Loop through over module and parse it.
        for raw_module in &options.raw_modules_to_parse {
            let target_path = Path::new(&raw_module);

            // Check for info.txt
            let info_txt_path = target_path.join("info.txt");
            if info_txt_path.exists() {
                info!(
                    "Dispatching parse for module {:?}",
                    target_path.file_name().unwrap_or_default()
                );
                let parsed_raws = parse_module(&target_path, &options)?;
                results.raws.extend(parsed_raws.parsed_raws);
                unprocessed_raws.extend(parsed_raws.unprocessed_raws);
            }
        }
    }

    // Next we can check if any raw files are specified
    if !options.raw_files_to_parse.is_empty() {
        // Parse all raw files that are specified.
        for raw_file in &options.raw_files_to_parse {
            let target_path = Path::new(&raw_file);
            info!(
                "Dispatching parse for raw file {:?}",
                target_path.file_name().unwrap_or_default()
            );
            let parsed_raws = parse_raw_file(&target_path, &options)?;
            results.raws.extend(parsed_raws.parsed_raws);
            unprocessed_raws.extend(parsed_raws.unprocessed_raws);
        }
    }

    // Finally we can check if any legends exports are specified
    if !options.legends_exports_to_parse.is_empty() {
        // Parse all legends exports that are specified.
        for legends_export in &options.legends_exports_to_parse {
            let target_path = Path::new(&legends_export);

            results
                .raws
                .extend(legends_export::parse(&target_path, &options)?);
        }
    }

    // Resolve the unprocessed creatures
    // Prerequisites: build a list of creature variations
    let creature_variations: Vec<CreatureVariation> = results
        .raws
        .iter()
        .filter_map(|raw| {
            if raw.get_type() == ObjectType::CreatureVariation {
                if let Some(cv) = raw
                    .as_ref()
                    .as_any()
                    .downcast_ref::<CreatureVariation>()
                    .cloned()
                {
                    return Some(cv);
                }
                error!(
                    "Matched CreatureVariation but failed to downcast for {}",
                    raw.get_identifier()
                );
            }
            None
        })
        .collect();

    info!(
        "Resolving {} unprocessed creatures using {} creature variation definitions",
        unprocessed_raws.len(),
        creature_variations.len()
    );

    // Write the unprocessed raws to a file
    // let _ = serde_json::to_writer_pretty(
    //     std::fs::File::create("unprocessed_raws.json").unwrap(),
    //     &unprocessed_raws,
    // );

    let mut simple_unprocessed: Vec<UnprocessedRaw> = Vec::new();
    let mut complex_unprocessed: Vec<UnprocessedRaw> = Vec::new();

    // Split the unprocessed raws into simple and complex
    for unprocessed_raw in unprocessed_raws {
        if unprocessed_raw.is_simple() {
            simple_unprocessed.push(unprocessed_raw);
        } else {
            complex_unprocessed.push(unprocessed_raw);
        }
    }

    // Resolve the simple creatures first
    let resolved_simple_creatures: Vec<Creature> = simple_unprocessed
        .iter_mut()
        .filter(|raw| raw.raw_type() == ObjectType::Creature)
        .filter_map(|raw| {
            match raw.resolve(creature_variations.as_slice(), results.raws.as_slice()) {
                Ok(c) => Some(c),
                Err(e) => {
                    error!(
                        "Unable to resolve simple creature {}: {:?}",
                        raw.get_identifier(),
                        e
                    );
                    None
                }
            }
        })
        .map(|c| clone_raw_object_box(&c))
        .filter_map(|c| {
            c.as_ref().as_any().downcast_ref::<Creature>().map_or_else(
                || {
                    error!("Downcast failed for simple creature {}", c.get_identifier());
                    None
                },
                |creature| Some(creature.clone()),
            )
        })
        .collect();

    info!(
        "Resolved {} simple creatures",
        resolved_simple_creatures.len()
    );

    results.raws.extend(
        resolved_simple_creatures
            .iter()
            .map(|c| Box::new(c.clone()) as Box<dyn RawObject>),
    );

    // Now we can do the second pass through the unprocessed creatures, but add the complex creatures
    // to the results.raws vector as they are resolved.
    let mut resolved_complex_creatures = 0_usize;
    for unprocessed_raw in &mut complex_unprocessed {
        if unprocessed_raw.raw_type() == ObjectType::Creature {
            match unprocessed_raw.resolve(creature_variations.as_slice(), results.raws.as_slice()) {
                Ok(c) => {
                    resolved_complex_creatures += 1;
                    results.raws.push(clone_raw_object_box(&c));
                }
                Err(e) => {
                    error!(
                        "Unable to resolve complex creature {}: {:?}",
                        unprocessed_raw.get_identifier(),
                        e
                    );
                }
            }
        }
    }

    info!("Resolved {resolved_complex_creatures} complex creatures");

    // Parse the info modules
    results.modules = parse_module_info_files(&options)?;

    // Print a summary of what we parsed (sum by ObjectType)
    if options.log_summary {
        let summary = summarize_raws(results.raws.as_slice());
        log_summary(&summary);
    }

    Ok(results)
}
