//! Tests for verifying that the various search functions work.

use chrono::TimeDelta;
use dfraw_parser::{metadata::RawModuleLocation, tags::ObjectType, traits::IsEmpty};
use dfraw_parser_sqlite_lib::{NumericConstraint, NumericFilter, SearchQuery};
use dfraw_parser_test_util::{get_test_client, json_helpers::identifier_from_json_blob};

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
fn has_results_only_for_favorites() {
    const FAVORITE_RAW_ID: i64 = 1206;
    setup_tracing();
    let client_mutex = get_test_client();

    {
        let client = client_mutex.lock().expect("Failed to lock DbClient");
        client
            .add_favorite_raw(FAVORITE_RAW_ID)
            .expect("Failed to add id:1206 as favorite.");
    }

    // get all raws within only 'Vanilla' location
    let query = SearchQuery {
        favorites_only: true,
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
    assert!(
        search_results
            .results
            .iter()
            .any(|r| r.id == FAVORITE_RAW_ID),
        "Should have matched our favorite, but did not."
    );

    // Cleanup
    {
        let client = client_mutex.lock().expect("Failed to lock DbClient");
        client
            .remove_favorite_raw(FAVORITE_RAW_ID)
            .expect("Failed to add id:1206 as favorite.");
    }
    let search_results = {
        let client = client_mutex.lock().expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .expect("Failed to query the generated database")
    };

    assert!(
        !search_results
            .results
            .iter()
            .any(|r| r.id == FAVORITE_RAW_ID),
        "Should have not matched our favorite, but did."
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
        assert!(!result.data.is_empty());
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

#[test]
fn verify_get_set_delete_favorite_raws() {
    setup_tracing();
    let client_mutex = get_test_client();
    let initial_favorite_raws = client_mutex
        .lock()
        .expect("Failed to lock DbClient")
        .get_favorite_raws()
        .expect("Get favorite raws failed");
    client_mutex
        .lock()
        .expect("Failed to lock DbClient")
        .add_favorite_raw(13)
        .expect("Failed to add favorite raw 13");
    client_mutex
        .lock()
        .expect("Failed to lock DbClient")
        .add_favorite_raw(203)
        .expect("Failed to add favorite raw 203");
    let after_favorite_raws = client_mutex
        .lock()
        .expect("Failed to lock DbClient")
        .get_favorite_raws()
        .expect("Get favorite raws failed");
    client_mutex
        .lock()
        .expect("Failed to lock DbClient")
        .remove_favorite_raw(13)
        .expect("Failed to remove favorite raw 13");
    let final_favorite_raws = client_mutex
        .lock()
        .expect("Failed to lock DbClient")
        .get_favorite_raws()
        .expect("Get favorite raws failed");

    // Verify
    assert_eq!(initial_favorite_raws.len(), 0);
    assert_eq!(after_favorite_raws.len(), 2);
    assert_eq!(final_favorite_raws.len(), 1);
    assert_eq!(final_favorite_raws.first().unwrap_or(&0), &203);
}

#[test]
fn verify_previous_insertion_duration() {
    setup_tracing();
    let client_mutex = get_test_client();
    let duration = client_mutex
        .lock()
        .expect("Failed to lock DbClient")
        .get_last_insertion_duration()
        .expect("Failed to get last insertion duration")
        .expect("No insertion duration found when expected");
    assert_ne!(duration, TimeDelta::zero(), "duration should not be zero");
    tracing::info!(
        "Last insertion duration was {}ms",
        duration.num_milliseconds()
    );
}

#[test]
fn verify_previous_insertion_date() {
    setup_tracing();
    let client_mutex = get_test_client();
    let date = client_mutex
        .lock()
        .expect("Failed to lock DbClient")
        .get_last_insertion_date()
        .expect("Failed to get last insertion duration");
    assert!(!date.is_empty());
    tracing::info!("Last insertion date {date:?}");
}

#[test]
fn verify_previous_parse_date() {
    setup_tracing();
    let client_mutex = get_test_client();
    let date = client_mutex
        .lock()
        .expect("Failed to lock DbClient")
        .get_last_parse_operation_date()
        .expect("Failed to get last insertion duration");
    assert!(!date.is_empty());
    tracing::info!("Last parse operation date {date:?}");
}

#[test]
fn verify_previous_parse_duration() {
    setup_tracing();
    let client_mutex = get_test_client();
    let duration = client_mutex
        .lock()
        .expect("Failed to lock DbClient")
        .get_last_parse_duration()
        .expect("Failed to get last insertion duration")
        .expect("No insertion duration found when expected");
    assert_ne!(duration, TimeDelta::zero(), "duration should not be zero");
    tracing::info!("Last parse duration was {}ms", duration.num_milliseconds());
}

#[test]
fn verify_previous_df_dir() {
    setup_tracing();
    let client_mutex = get_test_client();
    let game_dir = client_mutex
        .lock()
        .expect("Failed to lock DbClient")
        .get_last_used_df_game_dir()
        .expect("Failed to get last insertion duration");

    assert!(!game_dir.is_empty(), "game dir shouldn't be empty");
    tracing::info!("Last df game dir was {game_dir}");
}
#[test]
fn verify_previous_user_dir() {
    setup_tracing();
    let client_mutex = get_test_client();
    let user_dir = client_mutex
        .lock()
        .expect("Failed to lock DbClient")
        .get_last_used_df_user_dir()
        .expect("Failed to get last insertion duration");

    assert!(!user_dir.is_empty(), "user dir shouldn't be empty");
    tracing::info!("Last df user dir was {user_dir}");
}

#[test]
fn get_last_used_parser_options() {
    setup_tracing();
    let client_mutex = get_test_client();
    let parser_options = client_mutex
        .lock()
        .expect("Failed to lock DbClient")
        .get_last_used_parser_options()
        .expect("Failed to get last parser options")
        .expect("Last parser options shouldn't be None");

    tracing::info!("Last used parsing options: {parser_options:?}");
}

#[test]
fn verify_preferred_search_limit() {
    setup_tracing();
    let client_mutex = get_test_client();
    let page_limit_1 = client_mutex
        .lock()
        .expect("Failed to lock DbClient")
        .get_preferred_search_limit()
        .expect("Failed to get preferred search limit");
    client_mutex
        .lock()
        .expect("Failed to lock DbClient")
        .set_preferred_search_limit(page_limit_1 + 10)
        .expect("Failed to set preferred search limit");
    let page_limit_2 = client_mutex
        .lock()
        .expect("Failed to lock DbClient")
        .get_preferred_search_limit()
        .expect("Failed to get preferred search limit");

    assert_ne!(
        page_limit_1, page_limit_2,
        "page limits should be different"
    );
    assert_ne!(page_limit_1, 0, "page limit cannot be zero");
    assert_ne!(page_limit_2, 0, "page limit cannot be zero");
}

#[test]
fn filter_numeric_min() {
    setup_tracing();
    let client_mutex = get_test_client();

    // Constraint: PETVALUE >= 500
    // Should find: JABBERER (1500)
    // Should NOT find: BIRD_BUZZARD (30)
    let query = SearchQuery {
        limit: 500,
        numeric_filters: vec![NumericFilter {
            key: "PETVALUE".into(), // Note: Uses "PETVALUE" (with underscore)
            constraint: NumericConstraint::Min(500),
        }],
        ..Default::default()
    };

    let search_results = {
        let client = client_mutex.lock().expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .expect("Failed to query database")
    };

    assert!(!search_results.results.is_empty());

    // Check for presence of high-value creature
    assert!(
        search_results
            .results
            .iter()
            .any(|r| identifier_from_json_blob(&r.data) == "JABBERER"),
        "Expected JABBERER (Value 1500) to be found with Min(500)"
    );

    // Check for absence of low-value creature
    assert!(
        !search_results
            .results
            .iter()
            .any(|r| identifier_from_json_blob(&r.data) == "BIRD_BUZZARD"),
        "Expected BIRD_BUZZARD (Value 30) to be excluded with Min(500)"
    );
}

#[test]
fn filter_numeric_max() {
    setup_tracing();
    let client_mutex = get_test_client();

    // Constraint: PETVALUE <= 100
    // Should find: BIRD_BUZZARD (30)
    // Should NOT find: SHARK_BASKING (1000)
    let query = SearchQuery {
        limit: 500,
        numeric_filters: vec![NumericFilter {
            key: "PETVALUE".into(),
            constraint: NumericConstraint::Max(100),
        }],
        ..Default::default()
    };

    let search_results = {
        let client = client_mutex.lock().expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .expect("Failed to query database")
    };

    assert!(!search_results.results.is_empty());

    assert!(
        search_results
            .results
            .iter()
            .any(|r| identifier_from_json_blob(&r.data) == "BIRD_BUZZARD"),
        "Expected BIRD_BUZZARD (Value 30) to be found with Max(100)"
    );

    assert!(
        !search_results
            .results
            .iter()
            .any(|r| identifier_from_json_blob(&r.data) == "SHARK_BASKING"),
        "Expected SHARK_BASKING (Value 1000) to be excluded with Max(100)"
    );
}

#[test]
fn filter_numeric_exact() {
    setup_tracing();
    let client_mutex = get_test_client();

    // Constraint: PETVALUE == 30
    let query = SearchQuery {
        numeric_filters: vec![NumericFilter {
            key: "PETVALUE".into(),
            constraint: NumericConstraint::Exact(30),
        }],
        ..Default::default()
    };

    let search_results = {
        let client = client_mutex.lock().expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .expect("Failed to query database")
    };

    assert!(!search_results.results.is_empty());

    assert!(
        search_results
            .results
            .iter()
            .any(|r| identifier_from_json_blob(&r.data) == "BIRD_BUZZARD"),
        "Expected BIRD_BUZZARD to be found with Exact(30)"
    );

    assert!(
        !search_results
            .results
            .iter()
            .any(|r| identifier_from_json_blob(&r.data) == "GIANT_BUZZARD"),
        "Expected GIANT_BUZZARD to be excluded with Exact(30)"
    );
}

#[test]
fn filter_numeric_range() {
    setup_tracing();
    let client_mutex = get_test_client();

    // Constraint: PETVALUE between 20 and 40
    let query = SearchQuery {
        numeric_filters: vec![NumericFilter {
            key: "PETVALUE".into(),
            constraint: NumericConstraint::Range(20, 40),
        }],
        ..Default::default()
    };

    let search_results = {
        let client = client_mutex.lock().expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .expect("Failed to query database")
    };

    assert!(
        search_results
            .results
            .iter()
            .any(|r| identifier_from_json_blob(&r.data) == "BIRD_BUZZARD"),
        "Expected BIRD_BUZZARD (30) inside Range(20, 40)"
    );

    assert!(
        !search_results
            .results
            .iter()
            .any(|r| identifier_from_json_blob(&r.data) == "GIANT_BUZZARD"),
        "Expected GIANT_BUZZARD (500) outside Range(20, 40)"
    );
}

#[test]
fn filter_numeric_multiple() {
    setup_tracing();
    let client_mutex = get_test_client();

    let query_two_constraints = SearchQuery {
        limit: 500,
        numeric_filters: vec![
            NumericFilter {
                key: "CLUTCH_SIZE_MIN".into(),
                constraint: NumericConstraint::Min(10),
            },
            NumericFilter {
                key: "CLUTCH_SIZE_MAX".into(),
                constraint: NumericConstraint::Max(30),
            },
        ],
        ..Default::default()
    };

    let search_results = {
        let client = client_mutex.lock().expect("Failed to lock DbClient");
        client
            .search_raws(&query_two_constraints)
            .expect("Failed to query database")
    };

    // If unimplemented, this will likely return ALL creatures, failing this assertion
    assert!(!search_results.results.is_empty());
    // Two-legged rhino lizard has clutch size 10:30, so it should be among the results
    assert!(
        search_results
            .results
            .iter()
            .any(|r| identifier_from_json_blob(&r.data) == "LIZARD_RHINO_TWO_LEGGED"),
        "LIZARD_RHINO_TWO_LEGGED was expected but was missing"
    );
    // Salt-water crocodile has clutch size 20:70, so it should be absent in the results
    assert!(
        !search_results
            .results
            .iter()
            .any(|r| identifier_from_json_blob(&r.data) == "CROCODILE_SALTWATER"),
        "CROCODILE_SALTWATER should have been missing but was present"
    );
}

#[test]
fn test_short_search_string_is_ignored() {
    setup_tracing();
    let client_mutex = get_test_client();

    let query = SearchQuery {
        search_string: Some("ab".to_string()),
        limit: 10,
        ..Default::default()
    };

    let search_results = {
        let client = client_mutex.lock().expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .expect("Failed short string query")
    };

    // Currently, this likely returns *everything* because the WHERE clause ignores the string.
    // Use this test to decide if that is the UX you want.
    assert!(!search_results.results.is_empty());
}

#[test]
fn test_search_query_cleaning() {
    let query = SearchQuery {
        search_string: Some(String::new()),
        identifier_query: Some(String::new()),
        ..Default::default()
    };

    let cleaned = query.clean();

    assert!(cleaned.search_string.is_none());
    assert!(cleaned.identifier_query.is_none());
}

#[test]
fn test_pagination_overflow() {
    setup_tracing();
    let client_mutex = get_test_client();

    let query = SearchQuery {
        page: 1000,
        limit: 50,
        ..Default::default()
    };

    let search_results = {
        let client = client_mutex.lock().expect("Failed to lock DbClient");
        client.search_raws(&query).expect("Failed pagination query")
    };

    assert!(search_results.results.is_empty());
    // Total count should still be accurate
    assert!(search_results.total_count > 0);
}
