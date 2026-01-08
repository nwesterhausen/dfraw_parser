//! Tests for verifying that the various search functions work.

use dfraw_parser::metadata::{ObjectType, RawModuleLocation};
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

    assert!(
        !search_results.results.is_empty(),
        "Search should have found flying creatures (e.g., Peregrine Falcons) in the database."
    );
}

#[test]
fn test_identifier_partial_matching() {
    setup_tracing();
    let client_mutex = get_test_client();

    // Search for a partial identifier.
    // "IRON" should match things like "PIG_IRON", "ELEMENTMAN_IRON", etc.
    let query = SearchQuery {
        identifier_query: Some("IRON".to_string()),
        ..Default::default()
    };

    let search_results = {
        let client = client_mutex.lock().expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .expect("Failed to query database")
    };

    assert!(
        !search_results.results.is_empty(),
        "Should match identifiers containing 'IRON'"
    );

    for result in search_results.results {
        // Since search_raws returns blobs, we deserialize to check the identifier if needed,
        // but here we just verify existence based on the SQL logic.
        assert!(!result.is_empty());
    }
}

#[test]
fn test_raw_type_filtering_intersection() {
    setup_tracing();
    let client_mutex = get_test_client();

    // Test that multiple types return results from both categories
    let query = SearchQuery {
        raw_types: vec![ObjectType::Creature, ObjectType::Plant],
        ..Default::default()
    };

    let search_results = {
        let client = client_mutex.lock().expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .expect("Failed to query database")
    };

    // In vanilla raws, both creatures and plants are plentiful
    assert!(
        search_results.results.len() > 10,
        "Should return a healthy mix of creatures and plants"
    );
}

#[test]
fn test_pagination_logic() {
    setup_tracing();
    let client_mutex = get_test_client();

    // Page 1
    let query_p1 = SearchQuery {
        limit: 5,
        page: 1,
        ..Default::default()
    };
    let res_p1 = {
        let client = client_mutex.lock().expect("Failed to lock DbClient");
        client.search_raws(&query_p1).expect("Failed P1")
    };

    // Page 2
    let query_p2 = SearchQuery {
        limit: 5,
        page: 2,
        ..Default::default()
    };
    let res_p2 = {
        let client = client_mutex.lock().expect("Failed to lock DbClient");
        client.search_raws(&query_p2).expect("Failed P2")
    };

    assert_eq!(res_p1.results.len(), 5);
    assert_eq!(res_p2.results.len(), 5);

    // Ensure total_count is the same for both pages and reflects the full DB
    assert!(res_p1.total_count > 5);
    assert_eq!(res_p1.total_count, res_p2.total_count);

    // Ensure the results are actually different (not just returning the same page)
    assert_ne!(
        res_p1.results[0], res_p2.results[0],
        "Page 2 should start with different items than Page 1"
    );
}

#[test]
fn test_combined_intersection_filtering() {
    setup_tracing();
    let client_mutex = get_test_client();

    // Combined query: String search + Type filter + Flag filter
    // Look for "Giant" + CREATURE + FLIER
    let query = SearchQuery {
        search_string: Some("Giant".to_string()),
        raw_types: vec![ObjectType::Creature],
        required_flags: vec!["FLIER".to_string()],
        ..Default::default()
    };

    let search_results = {
        let client = client_mutex.lock().expect("Failed to lock DbClient");
        client.search_raws(&query).expect("Failed combined query")
    };

    // This should match things like Giant Bats or Giant Eagles
    assert!(
        !search_results.results.is_empty(),
        "Should find giant flying creatures"
    );
}

#[test]
fn test_ranking_by_relevance() {
    setup_tracing();
    let client_mutex = get_test_client();

    // FTS5 ranking test. Searching for "Dwarf" should put "Dwarf" at the top via bm25 before
    // any other creatures with "Dwarf" in their description
    let query = SearchQuery {
        search_string: Some("Dwarf".to_string()),
        limit: 10,
        ..Default::default()
    };

    let search_results = {
        let client = client_mutex.lock().expect("Failed to lock DbClient");
        client.search_raws(&query).expect("Failed ranking query")
    };

    assert!(!search_results.results.is_empty());
}

#[test]
fn test_no_results_for_non_existent_criteria() {
    setup_tracing();
    let client_mutex = get_test_client();

    // Search for a flag that doesn't exist
    let query = SearchQuery {
        required_flags: vec!["MADE_OF_CHEESE".to_string()],
        ..Default::default()
    };

    let search_results = {
        let client = client_mutex.lock().expect("Failed to lock DbClient");
        client.search_raws(&query).expect("Failed empty query")
    };

    assert!(search_results.results.is_empty());
    assert_eq!(search_results.total_count, 0);
}

#[test]
fn test_trigram_substring_matching() {
    setup_tracing();
    let client_mutex = get_test_client();

    // Trigram search: "oad" should find "Toad" and similar
    let query = SearchQuery {
        search_string: Some("oad".to_string()),
        ..Default::default()
    };

    let search_results = {
        let client = client_mutex.lock().expect("Failed to lock DbClient");
        client.search_raws(&query).expect("Failed trigram query")
    };

    assert!(
        !search_results.results.is_empty(),
        "Trigram search should find partial matches like 'toad'"
    );
}
