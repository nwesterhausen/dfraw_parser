//! Tests for verifying that the various search functions work.

use dfraw_parser::metadata::RawModuleLocation;
use dfraw_parser_sqlite_lib::SearchQuery;
use test_util::get_test_client;

use crate::common::setup_tracing;

mod common;

#[test]
fn has_zero_results_for_only_workshopmods_location() {
    setup_tracing();
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
    setup_tracing();
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
    setup_tracing();
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
    setup_tracing();
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
    setup_tracing();
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
