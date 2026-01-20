//! The CLI for the `dfraw-json-parser` crate.
//!
//! This crate is a command-line interface for the `dfraw-json-parser` crate, which parses Dwarf Fortress raw files and exports them as JSON.
//!
//! # Usage
//!
//! ```sh
//! dfraw-json-parser [OPTIONS] <dwarf-fortress-path>
//! ```
use dfraw_json_parser::{
    metadata::{ParserOptions, RawModuleLocation},
    parse,
    tags::ObjectType,
};

use std::path::{Path, PathBuf};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

const LONG_HELP: &str = "Usage: dfraw-json-parser [OPTIONS] [<dwarf-fortress-path>]

Default behavior:
    - Parse all object types
    - Print a summary of the parsed raws
    - Save the parsed raws to 'parsed-raws.json'
    - Log at the 'info' level

The following options are supported:
    -c, --creature      Parse creature raws
    -p, --plant         Parse plant raws
    -e, --entity        Parse entity raws
    -i, --inorganic     Parse inorganic raws
    -g, --graphics      Parse graphics raws

    -s, --summary       Print a summary of the parsed raws

    -P, --pretty        Pretty-print the parsed raws
        This is only used when saving the parsed raws to a file.

    -M, --metadata      Attach metadata to the parsed raws
        This includes the raws' file paths and other information about the
        raws' source.

    -o, --output PATH   Set the output path for the parsed raws
        Default value: 'parsed-raws.json'

    -r, --raw PATH      Parse a raw file
        This can be included multiple times to parse multiple raw files
        directly.

    -l, --legends PATH  Parse a legends export
        This can be included multiple times to parse multiple legends
        exports. These should be 'legends-plus' exports from DFHack.

    -m, --module PATH   Parse a raw module
        This can be included multiple times to parse multiple raw modules
        directly. This could be to specify a single raw module to parse, or
        to specify a raw module to parse in addition to the raw modules
        specified by the --vanilla, --mods, and --installed flags.

    -U, --user-data-dir PATH   Override the user data directory
        Default value: '/Volumes/LaCie/Samples/Bay_12_Games/'

    -v, --verbose       Increase the verbosity of the output
        Default log level: 'info'

        This supports up to two levels of verbosity. The first level is
        'debug', and the second level is 'trace'. (e.g. '-vv' or '--verbose --verbose')

    -q, --quiet         Decrease the verbosity of the output
        Default log level: 'info'

        This supports up to two levels of verbosity. The first level is
        'warn', and the second level is 'error'. (e.g. '-qq' or '--quiet --quiet')

    --vanilla           Parse the vanilla raws
    --mods              Parse the raws from all mods
    --installed         Parse the raws from installed mods

    --skip-info-files   Don't write the parsed 'info.txt' files to the output file.
        This is useful if you only want the raws.

    --only-info-files   Don't write raws to the output file.
        This is useful if you only want the 'info.txt' file data

    --speed-test        Do not write any output files, just parse the raws.

    -h, --help              Print this help message
    -V, --version           Print the version number";

#[derive(Debug)]
#[allow(clippy::struct_excessive_bools)]
/// The CLI arguments
struct Args {
    /// The log level to use for the logger
    pub log_level: Level,
    /// The locations to parse raws from
    pub locations: Vec<RawModuleLocation>,
    /// The object types to parse
    pub object_types: Vec<ObjectType>,
    /// Specific legends exports to parse (if any)
    pub legends_exports: Vec<PathBuf>,
    /// Whether or not to print a summary of the parsed raws
    pub print_summary: bool,
    /// Whether or not to attach metadata to the parsed raws
    pub attach_metadata: bool,
    /// Whether or not to format the parsed raws in the output file
    pub pretty_print: bool,
    /// The path to save the parsed raws to
    pub output_path: PathBuf,
    /// The path to the Dwarf Fortress directory
    pub df_path: Option<PathBuf>,
    /// Override the user data directory
    pub user_data_dir: Option<PathBuf>,
    /// Specific raw files to parse (if any)
    pub raw_file_paths: Vec<PathBuf>,
    /// Specific raw modules to parse (if any)
    pub raw_module_paths: Vec<PathBuf>,
    /// Whether or not to skip writing the parsed 'info.txt' files to the output file
    pub skip_info_files: bool,
    /// Whether or not to skip writing the parsed raws to the output file
    pub skip_raws: bool,
    /// Whether or not to skip writing any output files (useful only to benchmark how long it takes to parse the raws)
    pub speed_test: bool,
}

impl std::default::Default for Args {
    fn default() -> Self {
        Self {
            log_level: Level::INFO,
            locations: Vec::new(),
            object_types: Vec::new(),
            legends_exports: Vec::new(),
            print_summary: false,
            attach_metadata: false,
            pretty_print: false,
            skip_info_files: false,
            skip_raws: false,
            speed_test: false,
            output_path: PathBuf::from("parsed-raws.json"),
            df_path: None,
            user_data_dir: None,
            raw_file_paths: Vec::new(),
            raw_module_paths: Vec::new(),
        }
    }
}

#[allow(clippy::too_many_lines)]
fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    // Establish default values for the CLI arguments
    let mut args = Args::default();
    let mut include_graphics = false;

    let mut parser = lexopt::Parser::from_env();

    // Parse the CLI arguments
    while let Some(arg) = parser.next()? {
        match arg {
            Short('c') | Long("creature") => {
                args.object_types.push(ObjectType::Creature);
                args.object_types.push(ObjectType::CreatureVariation);
            }
            Short('p') | Long("plant") => {
                args.object_types.push(ObjectType::Plant);
            }
            Short('e') | Long("entity") => {
                args.object_types.push(ObjectType::Entity);
            }
            Short('i') | Long("inorganic") => {
                args.object_types.push(ObjectType::Inorganic);
            }

            Long("vanilla") => {
                args.locations.push(RawModuleLocation::Vanilla);
            }
            Long("mods") => {
                args.locations.push(RawModuleLocation::WorkshopMods);
            }
            Long("installed") => {
                args.locations.push(RawModuleLocation::InstalledMods);
            }

            Short('s') | Long("summary") => {
                args.print_summary = true;
            }
            Short('M') | Long("metadata") => {
                args.attach_metadata = true;
            }
            Short('P') | Long("pretty") => {
                args.pretty_print = true;
            }
            Short('g') | Long("graphics") => {
                include_graphics = true;
            }

            Short('o') | Long("output") => {
                args.output_path = PathBuf::from(parser.value()?);
            }
            Short('r') | Long("raw") => {
                args.raw_file_paths.push(PathBuf::from(parser.value()?));
            }
            Short('l') | Long("legends") => {
                args.legends_exports.push(PathBuf::from(parser.value()?));
            }
            Short('m') | Long("module") => {
                args.raw_module_paths.push(PathBuf::from(parser.value()?));
            }

            Short('v') | Long("verbose") => {
                if args.log_level == Level::INFO {
                    args.log_level = Level::DEBUG;
                } else {
                    args.log_level = Level::TRACE;
                }
            }
            Short('q') | Long("quiet") => {
                if args.log_level == Level::INFO {
                    args.log_level = Level::WARN;
                } else {
                    args.log_level = Level::ERROR;
                }
            }

            Short('h') | Long("help") => {
                println!("{LONG_HELP}");
                std::process::exit(0);
            }
            Short('V') | Long("version") => {
                println!("dfraw-json-parser v{}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }

            Long("skip-info-files") => {
                args.skip_info_files = true;
            }
            Long("skip-raws") => {
                args.skip_raws = true;
            }
            Long("speed-test") => {
                args.speed_test = true;
            }

            Short('U') | Long("user-data-dir") => {
                if let Ok(dir) = parser.value() {
                    args.user_data_dir = Some(PathBuf::from(dir));
                }
            }

            Value(val) if args.df_path.is_none() => {
                args.df_path = Some(PathBuf::from(val));
            }

            _ => {
                println!("Unknown argument: {arg:?}");
            }
        }
    }

    // If no object types were specified, parse all
    if args.object_types.is_empty() {
        args.object_types.push(ObjectType::Creature);
        args.object_types.push(ObjectType::Plant);
        args.object_types.push(ObjectType::Entity);
        args.object_types.push(ObjectType::Inorganic);
        args.object_types.push(ObjectType::CreatureVariation);
        args.object_types.push(ObjectType::MaterialTemplate);
    }
    // Include graphic types if requested
    if include_graphics {
        args.object_types.push(ObjectType::Graphics);
        args.object_types.push(ObjectType::TilePage);
    }

    for path in &mut args.raw_file_paths {
        *path = to_absolute_path(path, "raw file")?;
    }
    for path in &mut args.raw_module_paths {
        *path = to_absolute_path(path, "raw module")?;
    }
    for path in &mut args.legends_exports {
        *path = to_absolute_path(path, "legends export")?;
    }

    Ok(args)
}

fn to_absolute_path(path: &Path, descriptor: &str) -> Result<PathBuf, lexopt::Error> {
    path.canonicalize()
        .or(Err(lexopt::Error::Custom(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Unable to find {} path {}", descriptor, path.display()),
        )))))
}

fn write_output_file<P: AsRef<Path>>(
    output_path: &P,
    parse_results: &dfraw_json_parser::ParseResult,
    pretty_print: bool,
    skip_info_files: bool,
    skip_raws: bool,
) -> Result<(), lexopt::Error> {
    // Save the parsed raws to a file
    let file = match std::fs::File::create(output_path) {
        Ok(file) => file,
        Err(e) => {
            return Err(lexopt::Error::Custom(Box::new(std::io::Error::other(
                format!("Failed to create output file: {e:?}"),
            ))));
        }
    };

    tracing::info!("Opened {} for writing", output_path.as_ref().display());

    if skip_info_files && skip_raws {
        tracing::info!(
            "Specified --skip-info-files and --skip-raws, so not writing anything to the output file"
        );
        return Ok(());
    }

    if pretty_print {
        if skip_info_files {
            serde_json::to_writer_pretty(file, &parse_results.raws).map_err(|e| {
                lexopt::Error::Custom(Box::new(std::io::Error::other(format!(
                    "Failed to write output file: {e:?}"
                ))))
            })?;
        } else if skip_raws {
            serde_json::to_writer_pretty(file, &parse_results.modules).map_err(|e| {
                lexopt::Error::Custom(Box::new(std::io::Error::other(format!(
                    "Failed to write output file: {e:?}"
                ))))
            })?;
        } else {
            serde_json::to_writer_pretty(file, &parse_results).map_err(|e| {
                lexopt::Error::Custom(Box::new(std::io::Error::other(format!(
                    "Failed to write output file: {e:?}"
                ))))
            })?;
        }
    } else if skip_info_files {
        serde_json::to_writer(file, &parse_results.raws).map_err(|e| {
            lexopt::Error::Custom(Box::new(std::io::Error::other(format!(
                "Failed to write output file: {e:?}"
            ))))
        })?;
    } else if skip_raws {
        serde_json::to_writer(file, &parse_results.modules).map_err(|e| {
            lexopt::Error::Custom(Box::new(std::io::Error::other(format!(
                "Failed to write output file: {e:?}"
            ))))
        })?;
    } else {
        serde_json::to_writer(file, &parse_results).map_err(|e| {
            lexopt::Error::Custom(Box::new(std::io::Error::other(format!(
                "Failed to write output file: {e:?}"
            ))))
        })?;
    }

    if skip_info_files {
        tracing::info!(
            "Wrote {} parsed raws to {}",
            parse_results.raws.len(),
            output_path.as_ref().display(),
        );
    } else if skip_raws {
        tracing::info!(
            "Wrote {} parsed 'info.txt' files to {}",
            parse_results.modules.len(),
            output_path.as_ref().display(),
        );
    } else {
        tracing::info!(
            "Wrote {} parsed raws and {} parsed 'info.txt' files to {}",
            parse_results.raws.len(),
            parse_results.modules.len(),
            output_path.as_ref().display(),
        );
    }

    Ok(())
}

/// The main function for the CLI
///
/// # Panics
///
/// This function will panic if the logger cannot be initialized.
///
/// # Errors
///
/// This function will produce errors if the arguments cannot be parsed.
pub fn main() -> Result<(), lexopt::Error> {
    let args = parse_args()?;

    // Initialize the logger
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(args.log_level)
        // make it pretty
        .compact()
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    tracing::debug!("Parsed Args:\n{args:?}");

    // Build ParserOptions for the parser
    let mut options = ParserOptions::new();
    if let Some(df_dir) = &args.df_path {
        options.set_dwarf_fortress_directory(df_dir);
    }
    if let Some(user_dir) = &args.user_data_dir {
        options.set_user_data_directory(user_dir);
    }

    // Set locations to parse
    options.set_locations_to_parse(args.locations);

    // Set object types to parse
    options.set_object_types_to_parse(args.object_types);

    // Set raw files to parse
    options.set_raw_files_to_parse(args.raw_file_paths);

    // Set raw modules to parse
    options.set_raw_modules_to_parse(args.raw_module_paths);

    // Set legends exports to parse
    options.set_legends_exports_to_parse(args.legends_exports);

    // Set whether or not to attach metadata to the parsed raws
    if args.attach_metadata {
        options.attach_metadata_to_raws();
    }

    // Set whether to include the summary in the log or not
    if args.print_summary {
        options.log_summary();
    }

    tracing::debug!("Options sent to dfraw_parser:\n{options:?}");

    // Parse the raws
    let result = parse(&options).map_err(|e| {
        lexopt::Error::Custom(Box::new(std::io::Error::other(format!(
            "Failed to parse raws: {e:?}"
        ))))
    })?;

    // Print a summary of the parsed raws
    if args.print_summary {
        tracing::error!("Summary not implemented yet..");
    }

    if args.speed_test {
        return Ok(());
    }

    write_output_file(
        &args.output_path,
        &result,
        args.pretty_print,
        args.skip_info_files,
        args.skip_raws,
    )
}
