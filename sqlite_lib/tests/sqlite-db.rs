//! Test for the database

use dfraw_parser::metadata::ParserOptions;
use dfraw_parser::metadata::RawModuleLocation::Vanilla;
use dfraw_parser::parse;
use dfraw_parser_sqlite_lib::ClientOptions;
use dfraw_parser_sqlite_lib::DbClient;
use dfraw_parser_sqlite_lib::SearchQuery;

const TEST_DB_NAME: &str = "test.db";
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

    let num_info_files = parse_results.info_files.len();

    client
        .insert_parse_results(parse_results)
        .unwrap_or_else(|_| panic!("Failed to insert parsed raws"));

    // verify the data with a cross-module search
    let query = SearchQuery {
        raw_type_name: Some("CREATURE".to_string()),
        required_flags: vec!["FLIER".to_string()],
        ..Default::default()
    };

    let search_results = client
        .search_raws(&query)
        .expect("Failed to query the generated database");

    assert!(
        !search_results.is_empty(),
        "Search should have found flying creatures (e.g., Peregrine Falcons) in the database."
    );

    println!(
        "Test successful: Inserted {} modules; found {} flying creatures",
        num_info_files,
        search_results.len()
    );

    // Search using the search index
    let query = SearchQuery {
        search_string: Some("dvark".to_string()),
        ..Default::default()
    };

    let search_results = client
        .search_raws(&query)
        .expect("Failed to query the generated database");

    assert!(
        !search_results.is_empty(),
        "Search should have found flying creatures (e.g., Peregrine Falcons) in the database."
    );

    println!(
        "Test successful: Inserted {} modules; found {} matches for searching 'dvark'",
        num_info_files,
        search_results.len()
    );
}
