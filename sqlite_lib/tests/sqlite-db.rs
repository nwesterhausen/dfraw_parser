//! Test for the database

use std::sync::{Arc, Mutex, OnceLock};

use dfraw_parser::metadata::ParserOptions;
use dfraw_parser::metadata::RawModuleLocation;
use dfraw_parser::metadata::RawModuleLocation::Vanilla;
use dfraw_parser::parse;
use dfraw_parser_sqlite_lib::ClientOptions;
use dfraw_parser_sqlite_lib::DbClient;
use dfraw_parser_sqlite_lib::SearchQuery;

const TEST_DB_NAME: &str = "test.db";

// We store a Result so that tests can check if setup worked.
// We use Arc so multiple tests can own a reference to the client.
static SHARED_CLIENT: OnceLock<Result<Arc<Mutex<DbClient>>, String>> = OnceLock::new();

fn get_test_client() -> Arc<Mutex<DbClient>> {
    // get_or_init ensures the setup runs exactly once
    let result = SHARED_CLIENT.get_or_init(|| {
        // 1. One-time tracing setup
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .compact()
            .try_init();

        // 2. Setup test data
        let vanilla_path = test_util::ensure_vanilla_raws();

        // 3. Initialize the DbClient
        let options = ClientOptions {
            reset_database: true,
            overwrite_raws: true,
        };

        let mut client =
            DbClient::init_db(TEST_DB_NAME, options).map_err(|e| format!("DB Init Error: {e}"))?;

        // 4. Parse and Insert
        let mut parser_options = ParserOptions::default();
        parser_options.add_location_to_parse(Vanilla);
        parser_options.set_dwarf_fortress_directory(&vanilla_path);

        let parse_results = parse(&parser_options).map_err(|e| format!("Parse Error: {e}"))?;
        let num_info_files = parse_results.info_files.len();

        client
            .insert_parse_results(parse_results)
            .map_err(|e| format!("DB Insert Error: {e}"))?;

        println!("Sucessfully inserted {num_info_files} modules.");
        Ok(Arc::new(Mutex::new(client)))
    });

    match result {
        Ok(client_mutex) => Arc::clone(client_mutex),
        Err(e) => panic!("Global test setup failed: {e}"),
    }
}

#[test]
fn has_zero_results_for_only_workshopmods_location() {
    let client_mutex = get_test_client();

    // get all raws within only 'WorkshopMods' location
    let query = SearchQuery {
        locations: vec![RawModuleLocation::WorkshopMods],
        ..Default::default()
    };

    let search_results = {
        let client = client_mutex.lock().expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .expect("Failed to query the generated database")
    };

    println!("{search_results:?} for only 'WorkshopMods' location");

    // We expect no results
    assert!(
        search_results.results.is_empty(),
        "Should have had no results, but had some."
    );
}

#[test]
fn has_results_for_only_vanilla_location() {
    let client_mutex = get_test_client();

    // get all raws within only 'Vanilla' location
    let query = SearchQuery {
        locations: vec![RawModuleLocation::Vanilla],
        ..Default::default()
    };

    let search_results = {
        let client = client_mutex.lock().expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .expect("Failed to query the generated database")
    };

    println!("{search_results:?} for only 'Vanilla' location");

    assert!(
        !search_results.results.is_empty(),
        "Should have had results, but had none."
    );
}

#[test]
fn has_results_for_vanilla_or_workshopmods_locations() {
    let client_mutex = get_test_client();

    // get all raws within either 'Vanilla' or 'WorkshopMods' locations
    let query = SearchQuery {
        locations: vec![RawModuleLocation::Vanilla, RawModuleLocation::WorkshopMods],
        ..Default::default()
    };

    let search_results = {
        let client = client_mutex.lock().expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .expect("Failed to query the generated database")
    };

    println!("{search_results:?} for either 'Vanilla' or 'WorkshopMods' locations");

    assert!(
        !search_results.results.is_empty(),
        "Should have had results, but had none."
    );
}

#[test]
fn has_results_when_using_search_index() {
    let client_mutex = get_test_client();

    // Search using the search index.
    // Searching 'dvark' should return 3 results on vanilla raws: aardvark, aardvark man, giant aardvark
    let query = SearchQuery {
        search_string: Some("dvark".to_string()),
        ..Default::default()
    };

    let search_results = {
        let client = client_mutex.lock().expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .expect("Failed to query the generated database")
    };

    println!("{search_results:?} for searching 'dvark'");

    assert!(
        !search_results.results.is_empty(),
        "Search should have found some matches for 'dvark'"
    );
}

#[test]
fn has_results_for_required_flag() {
    let client_mutex = get_test_client();

    // get all raws with the `[FLIER]` tag
    let query = SearchQuery {
        required_flags: vec!["FLIER".to_string()],
        ..Default::default()
    };

    let search_results = {
        let client = client_mutex.lock().expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .expect("Failed to query the generated database")
    };

    println!("{search_results:?} with [FLIER] flag");

    assert!(
        !search_results.results.is_empty(),
        "Search should have found flying creatures (e.g., Peregrine Falcons) in the database."
    );
}
