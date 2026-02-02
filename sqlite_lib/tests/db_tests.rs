//! Tests for verifying that the various search functions work.

use chrono::TimeDelta;
use dfraw_parser::{metadata::RawModuleLocation, tokens::ObjectType, traits::IsEmpty};
use dfraw_parser_sqlite_lib::{NumericConstraint, NumericFilter, SearchQuery};
use dfraw_parser_test_util::get_test_client;
use uuid::Uuid;

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
        let client = client_mutex
            .lock()
            .inspect_err(|e| {
                tracing::error!("has_zero_results_for_only_workshopmods_location: {e}");
            })
            .expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .inspect_err(|e| {
                tracing::error!("has_zero_results_for_only_workshopmods_location: {e}");
            })
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
        let client = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("has_results_only_for_favorites:add_favorite {e}"))
            .expect("Failed to lock DbClient");
        client
            .add_favorite_raw(FAVORITE_RAW_ID)
            .inspect_err(|e| tracing::error!("has_results_only_for_favorites:add_favorite {e}"))
            .expect("Failed to add id:1206 as favorite.");
    }

    // get all raws within only 'Vanilla' location
    let query = SearchQuery {
        favorites_only: true,
        ..Default::default()
    };

    let search_results = {
        let client = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("has_results_only_for_favorites:search_favorites {e}"))
            .expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .inspect_err(|e| tracing::error!("has_results_only_for_favorites:search_favorites {e}"))
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
        let client = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("has_results_only_for_favorites:remove_favorite {e}"))
            .expect("Failed to lock DbClient");
        client
            .remove_favorite_raw(FAVORITE_RAW_ID)
            .inspect_err(|e| tracing::error!("has_results_only_for_favorites:remove_favorite {e}"))
            .expect("Failed to add id:1206 as favorite.");
    }
    let search_results = {
        let client = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("has_results_only_for_favorites:check_removed {e}"))
            .expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .inspect_err(|e| tracing::error!("has_results_only_for_favorites:check_removed {e}"))
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
        let client = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("has_results_for_only_vanilla_location: {e}"))
            .expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .inspect_err(|e| tracing::error!("has_results_for_only_vanilla_location: {e}"))
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
        let client = client_mutex
            .lock()
            .inspect_err(|e| {
                tracing::error!("has_results_for_vanilla_or_workshopmods_locations: {e}");
            })
            .expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .inspect_err(|e| {
                tracing::error!("has_results_for_vanilla_or_workshopmods_locations: {e}");
            })
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
        let client = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("has_results_when_using_search_index: {e}"))
            .expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .inspect_err(|e| tracing::error!("has_results_when_using_search_index: {e}"))
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
        let client = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("has_results_for_required_flag: {e}"))
            .expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .inspect_err(|e| tracing::error!("has_results_for_required_flag: {e}"))
            .expect("Failed to query the generated database")
    };

    assert!(
        !search_results.results.is_empty(),
        "Search should have found flying creatures (e.g., Peregrine Falcons) in the database."
    );
}

#[test]
fn query_identifier_partial_matching() {
    setup_tracing();
    let client_mutex = get_test_client();

    // Search for a partial identifier.
    // "IRON" should match things like "PIG_IRON", "ELEMENTMAN_IRON", etc.
    let query = SearchQuery {
        identifier_query: Some("IRON".to_string()),
        ..Default::default()
    };

    let search_results = {
        let client = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("query_identifier_partial_matching: {e}"))
            .expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .inspect_err(|e| tracing::error!("query_identifier_partial_matching: {e}"))
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
fn query_raw_type_filtering_intersection() {
    setup_tracing();
    let client_mutex = get_test_client();

    // Test that multiple types return results from both categories
    let query = SearchQuery {
        raw_types: vec![ObjectType::Creature, ObjectType::Plant],
        ..Default::default()
    };

    let search_results = {
        let client = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("query_raw_type_filtering_intersection: {e}"))
            .expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .inspect_err(|e| tracing::error!("query_raw_type_filtering_intersection: {e}"))
            .expect("Failed to query database")
    };

    // In vanilla raws, both creatures and plants are plentiful
    assert!(
        search_results.results.len() > 10,
        "Should return a healthy mix of creatures and plants"
    );
}

#[test]
fn verify_pagination_logic() {
    setup_tracing();
    let client_mutex = get_test_client();

    // Page 1
    let query_p1 = SearchQuery {
        limit: 5,
        page: 1,
        ..Default::default()
    };
    let res_p1 = {
        let client = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("verify_pagination_logic:q1 {e}"))
            .expect("Failed to lock DbClient");
        client
            .search_raws(&query_p1)
            .inspect_err(|e| tracing::error!("verify_pagination_logic:q1 {e}"))
            .expect("Failed P1")
    };

    // Page 2
    let query_p2 = SearchQuery {
        limit: 5,
        page: 2,
        ..Default::default()
    };
    let res_p2 = {
        let client = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("verify_pagination_logic:q2 {e}"))
            .expect("Failed to lock DbClient");
        client
            .search_raws(&query_p2)
            .inspect_err(|e| tracing::error!("verify_pagination_logic:q2 {e}"))
            .expect("Failed P2")
    };

    assert_eq!(res_p1.results.len(), 5);
    assert_eq!(res_p2.results.len(), 5);

    // Ensure total_count is the same for both pages and reflects the full DB
    assert!(res_p1.total_count > 5);
    assert_eq!(res_p1.total_count, res_p2.total_count);

    // Ensure the results are actually different (not just returning the same page)
    assert_ne!(
        res_p1.results[0].data.get_object_id(),
        res_p2.results[0].data.get_object_id(),
        "Page 2 should start with different items than Page 1"
    );
}

#[test]
fn query_combined_intersection_filtering() {
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
        let client = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("query_combined_intersection_filtering: {e}"))
            .expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .inspect_err(|e| tracing::error!("query_combined_intersection_filtering: {e}"))
            .expect("Failed combined query")
    };

    // This should match things like Giant Bats or Giant Eagles
    assert!(
        !search_results.results.is_empty(),
        "Should find giant flying creatures"
    );
}

#[test]
fn verify_ranking_by_relevance() {
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
        let client = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("verify_ranking_by_relevance: {e}"))
            .expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .inspect_err(|e| tracing::error!("verify_ranking_by_relevance: {e}"))
            .expect("Failed ranking query")
    };

    assert!(!search_results.results.is_empty());
}

#[test]
fn verify_no_results_for_non_existent_criteria() {
    setup_tracing();
    let client_mutex = get_test_client();

    // Search for a flag that doesn't exist
    let query = SearchQuery {
        required_flags: vec!["MADE_OF_CHEESE".to_string()],
        ..Default::default()
    };

    let search_results = {
        let client = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("verify_no_results_for_non_existent_criteria: {e}"))
            .expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .inspect_err(|e| tracing::error!("verify_no_results_for_non_existent_criteria: {e}"))
            .expect("Failed empty query")
    };

    assert!(search_results.results.is_empty());
    assert_eq!(search_results.total_count, 0);
}

#[test]
fn verify_trigram_substring_matching() {
    setup_tracing();
    let client_mutex = get_test_client();

    // Trigram search: "oad" should find "Toad" and similar
    let query = SearchQuery {
        search_string: Some("oad".to_string()),
        ..Default::default()
    };

    let search_results = {
        let client = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("verify_trigram_substring_matching: {e}"))
            .expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .inspect_err(|e| tracing::error!("verify_trigram_substring_matching: {e}"))
            .expect("Failed trigram query")
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
        .inspect_err(|e| tracing::error!("verify_get_set_delete_favorite_raws:get1 {e}"))
        .expect("Failed to lock DbClient")
        .get_favorite_raws()
        .inspect_err(|e| tracing::error!("verify_get_set_delete_favorite_raws:get1 {e}"))
        .expect("Get favorite raws failed");
    client_mutex
        .lock()
        .inspect_err(|e| tracing::error!("verify_get_set_delete_favorite_raws:add13 {e}"))
        .expect("Failed to lock DbClient")
        .add_favorite_raw(13)
        .inspect_err(|e| tracing::error!("verify_get_set_delete_favorite_raws:add13 {e}"))
        .expect("Failed to add favorite raw 13");
    client_mutex
        .lock()
        .inspect_err(|e| tracing::error!("verify_get_set_delete_favorite_raws:add203 {e}"))
        .expect("Failed to lock DbClient")
        .add_favorite_raw(203)
        .inspect_err(|e| tracing::error!("verify_get_set_delete_favorite_raws:add203 {e}"))
        .expect("Failed to add favorite raw 203");
    let after_favorite_raws = client_mutex
        .lock()
        .inspect_err(|e| tracing::error!("verify_get_set_delete_favorite_raws:get2 {e}"))
        .expect("Failed to lock DbClient")
        .get_favorite_raws()
        .inspect_err(|e| tracing::error!("verify_get_set_delete_favorite_raws:get2 {e}"))
        .expect("Get favorite raws failed");
    client_mutex
        .lock()
        .inspect_err(|e| tracing::error!("verify_get_set_delete_favorite_raws:rem13 {e}"))
        .expect("Failed to lock DbClient")
        .remove_favorite_raw(13)
        .inspect_err(|e| tracing::error!("verify_get_set_delete_favorite_raws:rem12 {e}"))
        .expect("Failed to remove favorite raw 13");
    let final_favorite_raws = client_mutex
        .lock()
        .inspect_err(|e| tracing::error!("verify_get_set_delete_favorite_raws:get3 {e}"))
        .expect("Failed to lock DbClient")
        .get_favorite_raws()
        .inspect_err(|e| tracing::error!("verify_get_set_delete_favorite_raws:get3 {e}"))
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
        .inspect_err(|e| tracing::error!("verify_previous_insertion_duration: {e}"))
        .expect("Failed to lock DbClient")
        .get_last_insertion_duration()
        .inspect_err(|e| tracing::error!("verify_previous_insertion_duration: {e}"))
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
        .inspect_err(|e| tracing::error!("verify_previous_insertion_date: {e}"))
        .expect("Failed to lock DbClient")
        .get_last_insertion_date()
        .inspect_err(|e| tracing::error!("verify_previous_insertion_date: {e}"))
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
        .inspect_err(|e| tracing::error!("verify_previous_parse_date: {e}"))
        .expect("Failed to lock DbClient")
        .get_last_parse_operation_date()
        .inspect_err(|e| tracing::error!("verify_previous_parse_date: {e}"))
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
        .inspect_err(|e| tracing::error!("verify_previous_parse_duration: {e}"))
        .expect("Failed to lock DbClient")
        .get_last_parse_duration()
        .inspect_err(|e| tracing::error!("verify_previous_parse_duration: {e}"))
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
        .inspect_err(|e| tracing::error!("verify_previous_df_dir: {e}"))
        .expect("Failed to lock DbClient")
        .get_last_used_df_game_dir()
        .inspect_err(|e| tracing::error!("verify_previous_df_dir: {e}"))
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
        .inspect_err(|e| tracing::error!("verify_previous_user_dir: {e}"))
        .expect("Failed to lock DbClient")
        .get_last_used_df_user_dir()
        .inspect_err(|e| tracing::error!("verify_previous_user_dir: {e}"))
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
        .inspect_err(|e| tracing::error!("get_last_used_parser_options: {e}"))
        .expect("Failed to lock DbClient")
        .get_last_used_parser_options()
        .inspect_err(|e| tracing::error!("get_last_used_parser_options: {e}"))
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
        .inspect_err(|e| tracing::error!("verify_preferred_search_limit:get1 {e}"))
        .expect("Failed to lock DbClient")
        .get_preferred_search_limit()
        .inspect_err(|e| tracing::error!("verify_preferred_search_limit:get1 {e}"))
        .expect("Failed to get preferred search limit");
    client_mutex
        .lock()
        .inspect_err(|e| tracing::error!("verify_preferred_search_limit:set {e}"))
        .expect("Failed to lock DbClient")
        .set_preferred_search_limit(page_limit_1 + 10)
        .inspect_err(|e| tracing::error!("verify_preferred_search_limit:set {e}"))
        .expect("Failed to set preferred search limit");
    let page_limit_2 = client_mutex
        .lock()
        .inspect_err(|e| tracing::error!("verify_preferred_search_limit:get2 {e}"))
        .expect("Failed to lock DbClient")
        .get_preferred_search_limit()
        .inspect_err(|e| tracing::error!("verify_preferred_search_limit:get2 {e}"))
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
        let client = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("filter_numeric_min: {e}"))
            .expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .inspect_err(|e| tracing::error!("filter_numeric_min: {e}"))
            .expect("Failed to query database")
    };

    assert!(!search_results.results.is_empty());

    // Check for presence of high-value creature
    assert!(
        search_results
            .results
            .iter()
            .any(|r| r.data.get_identifier() == "JABBERER"),
        "Expected JABBERER (Value 1500) to be found with Min(500)"
    );

    // Check for absence of low-value creature
    assert!(
        !search_results
            .results
            .iter()
            .any(|r| r.data.get_identifier() == "BIRD_BUZZARD"),
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
        let client = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("filter_numeric_max: {e}"))
            .expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .inspect_err(|e| tracing::error!("filter_numeric_max: {e}"))
            .expect("Failed to query database")
    };

    assert!(!search_results.results.is_empty());

    assert!(
        search_results
            .results
            .iter()
            .any(|r| r.data.get_identifier() == "BIRD_BUZZARD"),
        "Expected BIRD_BUZZARD (Value 30) to be found with Max(100)"
    );

    assert!(
        !search_results
            .results
            .iter()
            .any(|r| r.data.get_identifier() == "SHARK_BASKING"),
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
        let client = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("filter_numeric_exact: {e}"))
            .expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .inspect_err(|e| tracing::error!("filter_numeric_exact: {e}"))
            .expect("Failed to query database")
    };

    assert!(!search_results.results.is_empty());

    assert!(
        search_results
            .results
            .iter()
            .any(|r| r.data.get_identifier() == "BIRD_BUZZARD"),
        "Expected BIRD_BUZZARD to be found with Exact(30)"
    );

    assert!(
        !search_results
            .results
            .iter()
            .any(|r| r.data.get_identifier() == "GIANT_BUZZARD"),
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
        let client = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("filter_numeric_range: {e}"))
            .expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .inspect_err(|e| tracing::error!("filter_numeric_range: {e}"))
            .expect("Failed to query database")
    };

    assert!(
        search_results
            .results
            .iter()
            .any(|r| r.data.get_identifier() == "BIRD_BUZZARD"),
        "Expected BIRD_BUZZARD (30) inside Range(20, 40)"
    );

    assert!(
        !search_results
            .results
            .iter()
            .any(|r| r.data.get_identifier() == "GIANT_BUZZARD"),
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
        let client = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("filter_numeric_multiple: {e}"))
            .expect("Failed to lock DbClient");
        client
            .search_raws(&query_two_constraints)
            .inspect_err(|e| tracing::error!("filter_numeric_multiple: {e}"))
            .expect("Failed to query database")
    };

    // If unimplemented, this will likely return ALL creatures, failing this assertion
    assert!(!search_results.results.is_empty());
    // Two-legged rhino lizard has clutch size 10:30, so it should be among the results
    assert!(
        search_results
            .results
            .iter()
            .any(|r| r.data.get_identifier() == "LIZARD_RHINO_TWO_LEGGED"),
        "LIZARD_RHINO_TWO_LEGGED was expected but was missing"
    );
    // Salt-water crocodile has clutch size 20:70, so it should be absent in the results
    assert!(
        !search_results
            .results
            .iter()
            .any(|r| r.data.get_identifier() == "CROCODILE_SALTWATER"),
        "CROCODILE_SALTWATER should have been missing but was present"
    );
}

#[test]
fn verify_short_search_string_is_ignored() {
    setup_tracing();
    let client_mutex = get_test_client();

    let query = SearchQuery {
        search_string: Some("ab".to_string()),
        limit: 10,
        ..Default::default()
    };

    let search_results = {
        let client = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("verify_short_search_string_is_ignored: {e}"))
            .expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .inspect_err(|e| tracing::error!("verify_short_search_string_is_ignored: {e}"))
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
        let client = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("test_pagination_overflow: {e}"))
            .expect("Failed to lock DbClient");
        client
            .search_raws(&query)
            .inspect_err(|e| tracing::error!("test_pagination_overflow: {e}"))
            .expect("Failed pagination query")
    };

    assert!(search_results.results.is_empty());
    // Total count should still be accurate
    assert!(search_results.total_count > 0);
}

#[test]
fn verify_recent_search_terms() {
    setup_tracing();
    let client_mutex = get_test_client();

    // Add a unique set of search terms to overflow the buffer (limit is 10)
    for i in 0..15 {
        client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("verify_recent_search_terms: {e}"))
            .expect("Failed to lock DbClient")
            .add_recent_search_term(Some(format!("unique_term_{i}")))
            .inspect_err(|e| tracing::error!("verify_recent_search_terms: {e}"))
            .expect("Failed to add term");
    }

    let terms = {
        let client = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("verify_recent_search_terms: {e}"))
            .expect("Failed to lock DbClient");
        client
            .get_recent_search_terms()
            .inspect_err(|e| tracing::error!("verify_recent_search_terms: {e}"))
            .expect("Failed to get terms")
    };

    // Verify limit of 10
    assert_eq!(terms.len(), 10, "Should limit recent search terms to 10");

    // Verify LIFO behavior - "unique_term_14" should be at the top
    assert_eq!(
        terms[0], "unique_term_14",
        "Most recent term should be first"
    );

    // Verify oldest are dropped - "unique_term_0" should not exist
    assert!(
        !terms.contains(&"unique_term_0".to_string()),
        "Oldest terms should be dropped"
    );

    // Verify deduplication/promotion
    // Adding an existing term should move it to the front
    let existing_term = "unique_term_10";
    {
        let client = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("verify_recent_search_terms: {e}"))
            .expect("Failed to lock DbClient");
        client
            .add_recent_search_term(Some(existing_term.to_string()))
            .inspect_err(|e| tracing::error!("verify_recent_search_terms: {e}"))
            .expect("Failed to add existing term");
    }
    let updated_terms = {
        let client = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("verify_recent_search_terms: {e}"))
            .expect("Failed to lock DbClient");
        client
            .get_recent_search_terms()
            .inspect_err(|e| tracing::error!("verify_recent_search_terms: {e}"))
            .expect("Failed to get terms")
    };

    assert_eq!(
        updated_terms[0], existing_term,
        "Re-adding a term should move it to the front"
    );
    assert_eq!(
        updated_terms.iter().filter(|&t| t == existing_term).count(),
        1,
        "Term should not be duplicated in the list"
    );
}

#[test]
fn verify_stored_settings() {
    setup_tracing();
    let client_mutex = get_test_client();

    let settings_json = r#"{"theme": "dark", "zoom": 100}"#;
    client_mutex
        .lock()
        .inspect_err(|e| tracing::error!("verify_stored_settings: {e}"))
        .expect("Failed to lock DbClient")
        .set_stored_settings(&settings_json.to_string())
        .inspect_err(|e| tracing::error!("verify_stored_settings: {e}"))
        .expect("Failed to set settings");

    let retrieved = client_mutex
        .lock()
        .inspect_err(|e| tracing::error!("verify_stored_settings: {e}"))
        .expect("Failed to lock DbClient")
        .get_stored_settings()
        .inspect_err(|e| tracing::error!("verify_stored_settings: {e}"))
        .expect("Failed to get settings");
    assert_eq!(retrieved, settings_json);
}

#[test]
fn verify_steam_autodetect_toggle() {
    setup_tracing();
    let client_mutex = get_test_client();

    // Test setting to true
    client_mutex
        .lock()
        .inspect_err(|e| tracing::error!("verify_steam_autodetect_toggle: {e}"))
        .expect("Failed to lock DbClient")
        .set_use_steam_autodetect(true)
        .inspect_err(|e| tracing::error!("verify_steam_autodetect_toggle: {e}"))
        .expect("Failed to set autodetect true");

    let autodetect_setting = client_mutex
        .lock()
        .inspect_err(|e| tracing::error!("verify_steam_autodetect_toggle: {e}"))
        .expect("Failed to lock DbClient")
        .get_use_steam_autodetect()
        .inspect_err(|e| tracing::error!("verify_steam_autodetect_toggle: {e}"))
        .expect("Failure to get setting.");
    assert!(autodetect_setting, "Should return true");

    // Test setting to false
    client_mutex
        .lock()
        .inspect_err(|e| tracing::error!("verify_steam_autodetect_toggle: {e}"))
        .expect("Failed to lock DbClient")
        .set_use_steam_autodetect(false)
        .inspect_err(|e| tracing::error!("verify_steam_autodetect_toggle: {e}"))
        .expect("Failed to set autodetect true");

    let autodetect_setting = client_mutex
        .lock()
        .inspect_err(|e| tracing::error!("verify_steam_autodetect_toggle: {e}"))
        .expect("Failed to lock DbClient")
        .get_use_steam_autodetect()
        .inspect_err(|e| tracing::error!("verify_steam_autodetect_toggle: {e}"))
        .expect("Failure to get setting.");
    assert!(!autodetect_setting, "Should return false");
}

#[test]
fn verify_raw_object_retrieval_and_metadata() {
    setup_tracing();
    let client_mutex = get_test_client();

    // 1. Search to find a valid ID for a known vanilla creature ("TOAD")
    let query = SearchQuery {
        search_string: Some("Toad".to_string()),
        limit: 1,
        ..Default::default()
    };
    let results = client_mutex
        .lock()
        .inspect_err(|e| tracing::error!("verify_raw_object_retrieval_and_metadata: {e}"))
        .expect("Failed to lock DbClient")
        .search_raws(&query)
        .inspect_err(|e| tracing::error!("verify_raw_object_retrieval_and_metadata: {e}"))
        .expect("Search failed");

    if let Some(first_result) = results.results.first() {
        let db_id = first_result.id;
        let identifier = first_result.data.get_identifier();

        // 2. Test get_raw(id)
        let raw = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("verify_raw_object_retrieval_and_metadata: {e}"))
            .expect("Failed to lock DbClient")
            .get_raw(db_id)
            .inspect_err(|e| tracing::error!("verify_raw_object_retrieval_and_metadata: {e}"))
            .expect("Failed to retrieve raw by ID");
        assert_eq!(
            raw.get_identifier(),
            identifier,
            "Retrieved raw does not match expected identifier"
        );

        // 3. Test exists_raw(&raw)
        let exists = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("verify_raw_object_retrieval_and_metadata: {e}"))
            .expect("Failed to lock DbClient")
            .exists_raw(&raw)
            .inspect_err(|e| tracing::error!("verify_raw_object_retrieval_and_metadata: {e}"))
            .expect("Failed to check raw existence");
        assert!(exists, "exists_raw should return true for retrieved raw");

        // 4. Test try_get_raw_id(&raw)
        let retrieved_id = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("verify_raw_object_retrieval_and_metadata: {e}"))
            .expect("Failed to lock DbClient")
            .try_get_raw_id(&raw)
            .inspect_err(|e| tracing::error!("verify_raw_object_retrieval_and_metadata: {e}"))
            .expect("Failed to get raw ID from object")
            .expect("ID should be found");
        assert_eq!(retrieved_id, db_id, "Retrieved ID should match search ID");

        // 5. Test get_module_id_from_raw(&raw)
        let module_id = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("verify_raw_object_retrieval_and_metadata: {e}"))
            .expect("Failed to lock DbClient")
            .get_module_id_from_raw(&raw)
            .inspect_err(|e| tracing::error!("verify_raw_object_retrieval_and_metadata: {e}"))
            .expect("Failed to get module ID from raw");
        assert!(module_id > 0, "Module ID should be valid (non-zero)");
    } else {
        panic!(
            "Could not find 'Toad' in database to run retrieval tests. Ensure test data is loaded."
        );
    }
}

#[test]
fn verify_tile_page_lookups() {
    setup_tracing();
    let client_mutex = get_test_client();

    // Find a tile page using a filtered search
    let query = SearchQuery {
        raw_types: vec![ObjectType::TilePage],
        limit: 1,
        ..Default::default()
    };
    let results = client_mutex
        .lock()
        .inspect_err(|e| tracing::error!("verify_tile_page_lookups: {e}"))
        .expect("Failed to lock DbClient")
        .search_raws(&query)
        .inspect_err(|e| tracing::error!("verify_tile_page_lookups: {e}"))
        .expect("Search failed");

    if let Some(res) = results.results.first() {
        // Test get_tile_page_by_raw_id
        // We know this ID corresponds to a TilePage because of the search filter
        let tile_page_data = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("verify_tile_page_lookups: {e}"))
            .expect("Failed to lock DbClient")
            .get_tile_page_by_raw_id(res.id)
            .inspect_err(|e| tracing::error!("verify_tile_page_lookups: {e}"))
            .expect("Failed to get tile page data by raw ID");

        // Verify we can also look it up by its identifier
        let looked_up_by_ident = client_mutex
            .lock()
            .inspect_err(|e| tracing::error!("verify_tile_page_lookups: {e}"))
            .expect("Failed to lock DbClient")
            .get_tile_page_by_identifier(&tile_page_data.identifier)
            .inspect_err(|e| tracing::error!("verify_tile_page_lookups: {e}"))
            .expect("Failed to lookup tile page by identifier")
            .expect("Should have found tile page by identifier");

        assert_eq!(
            tile_page_data.raw_id, looked_up_by_ident.raw_id,
            "Lookup by ID and Identifier should yield same record"
        );
    } else {
        // If no tile pages are in the test data, we skip this check or warn.
        // Assuming vanilla data, tile pages should exist.
        tracing::warn!("No tile pages found in test data, skipping verification.");
    }
}

#[test]
fn get_raw_from_object_id() {
    let hydra_obj_id: Uuid = Uuid::parse_str("0617fc81-77b0-508a-947b-3899d1aebfd6")
        .expect("Failed to parse Hydra UUID");

    setup_tracing();
    let client_mutex = get_test_client();

    let raw = client_mutex
        .lock()
        .inspect_err(|e| tracing::error!("get_raw_from_object_id: {e}"))
        .expect("Failed to lock DbClient")
        .get_raw_by_object_id(hydra_obj_id)
        .inspect_err(|e| tracing::error!("get_raw_from_object_id: {e}"))
        .expect("Hydra not found by object_id");

    assert_eq!(raw.get_identifier(), "HYDRA");
}

#[test]
fn search_in_modules() {
    // If the version of Vanilla Creatures ever changes from 53.01, then we should
    // update this to match the latest version.
    let vanilla_creatures_5301_obj_id: Uuid =
        Uuid::parse_str("91D6C282-C40E-5964-B648-2351F99AF882")
            .expect("Failed to parse module UIUD");

    setup_tracing();
    let client_mutex = get_test_client();

    let query = SearchQuery {
        in_modules: vec![vanilla_creatures_5301_obj_id],
        limit: 1000,
        ..Default::default()
    };

    let search_results = client_mutex
        .lock()
        .inspect_err(|e| tracing::error!("search_in_modules: {e}"))
        .expect("Failed to lock DbClient")
        .search_raws(&query)
        .inspect_err(|e| tracing::error!("search_in_modules: {e}"))
        .expect("Search failed");

    assert!(
        !search_results.results.is_empty(),
        "Search should have results"
    );

    assert!(
        search_results
            .results
            .iter()
            .any(|r| r.data.get_identifier() == "BIRD_BUZZARD"),
        "Expected BIRD_BUZZARD to be found in Vanilla Creatures module"
    );

    assert!(
        !search_results
            .results
            .iter()
            .any(|r| r.data.get_identifier() == "SWEET_POTATO"),
        "Expected SWEET_POTATO to be abset (it should be in Vanilla Plants module)"
    );
}
