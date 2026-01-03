//! Test for the database

use dfraw_parser::metadata::ParserOptions;
use dfraw_parser::metadata::RawModuleLocation::Vanilla;
use dfraw_parser::parse;
use sqlite_lib::ClientOptions;
use sqlite_lib::DbClient;
use sqlite_lib::SearchQuery;
use std::collections::HashMap;
use std::fs;

const TEST_DB_NAME: &str = "test.db";
#[allow(dead_code)]
fn cleanup_test_db() {
    match fs::remove_file(TEST_DB_NAME) {
        Ok(()) => println!("Removed test file."),
        Err(error) => println!("{error:?}"),
    }
}
#[test]
fn test_parse_and_save_to_db() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(tracing::Level::INFO)
        // make it pretty
        .compact()
        // completes the builder.
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Setup test data (Download if missing)
    let vanilla_path = test_util::ensure_vanilla_raws();

    // initialize the DbClient
    let options = ClientOptions {
        reset_database: true,
        overwrite_raws: true,
    };
    let mut client =
        DbClient::init_db(TEST_DB_NAME, options).expect("Failed to initialize DbClient");

    // Parse the raws using dfraw_parser
    // We create dummy InfoFile details for the test
    let mut parser_options = ParserOptions::default();
    parser_options.add_location_to_parse(Vanilla);
    parser_options.set_dwarf_fortress_directory(&vanilla_path);

    let parse_results = parse(&parser_options)
        .expect("Failure to parse raws. Might be time to manually specify a directory.");

    assert!(
        !parse_results.raws.is_empty(),
        "Parser returned no objects. Is Dwarf Fortress installed in the default path?"
    );
    assert!(
        !parse_results.info_files.is_empty(),
        "Parser returned no info files."
    );
    // 5. Group Raws by Module Identity
    // We use a composite key of (name, version, location_id) to match Raws to their InfoFiles.
    // This allows us to handle multi-module parsing (Vanilla + Mods) correctly.
    let mut module_map = HashMap::new();
    for raw in parse_results.raws {
        let meta = raw.get_metadata();
        let key = (
            String::from(meta.get_module_name()),
            String::from(meta.get_module_version()),
            i32::from(meta.get_location()),
        );
        module_map.entry(key).or_insert_with(Vec::new).push(raw);
    }

    // 6. Insert into Database
    // We iterate through the parsed info files and grab the raws associated with each.
    for info in &parse_results.info_files {
        let key = (
            info.get_name(),
            info.get_version(),
            i32::from(info.get_location()),
        );
        tracing::info!("Inserting raws for {key:?}");
        if let Some(module_raws) = module_map.get(&key) {
            client
                .insert_module_data(info, module_raws)
                .unwrap_or_else(|_| {
                    panic!("Failed to insert module data for {}", info.get_identifier())
                });
        }
    }

    // 7. Verify the data with a cross-module search
    let query = SearchQuery {
        name_query: None,
        raw_type_name: Some("CREATURE".to_string()),
        required_flags: vec!["FLIER".to_string()],
        numeric_filters: vec![],
    };

    let search_results = client
        .search_raws(query)
        .expect("Failed to query the generated database");

    assert!(
        !search_results.is_empty(),
        "Search should have found flying creatures (e.g., Peregrine Falcons) in the database."
    );

    println!(
        "Test successful: Inserted {} modules and found {} flying creatures.",
        parse_results.info_files.len(),
        search_results.len()
    );
}
