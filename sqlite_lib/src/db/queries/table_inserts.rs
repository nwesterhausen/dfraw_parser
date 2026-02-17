//! Insert queries available as const for readability and moved out of the
//! functions to share common usage amongst a copule places

/// Requires 2 params:
///
/// * `raw_id`
/// * `token_name`
pub(super) const INSERT_COMMON_FLAG: &str =
    "INSERT INTO common_raw_flags (raw_id, token_name) VALUES (?1, ?2);";

/// Requires 2 params:
///
/// * `raw_id`
/// * `token_name`
/// * `value`
pub(super) const INSERT_NUMERIC_FLAG: &str = r"
INSERT INTO common_raw_flags_with_numeric_value
    (raw_id, token_name, value)
VALUES
    (?1, ?2, ?3);
";

/// Requires 7 params:
///
/// * `raw_id`
/// * `identifier`
/// * `file_path`
/// * `tile_width`
/// * `tile_height`
/// * `page_width`
/// * `page_height`
pub(super) const INSERT_TILE_PAGE: &str = r"
INSERT INTO tile_pages
    (raw_id, identifier, file_path, tile_width, tile_height, page_width, page_height)
VALUES
    (?1, ?2, ?3, ?4, ?5, ?6, ?7);
";

/// Requires 7 params:
///
/// * `raw_id`
/// * `tile_page_identifier`
/// * `offset_x`
/// * `offset_y`
/// * `primary_condition`
/// * `secondary_condition`
/// * `target_identifier`
pub(super) const INSERT_SPRITE_GRAPHIC: &str = r"
INSERT INTO sprite_graphics
    (raw_id, tile_page_identifier, offset_x, offset_y,
        primary_condition, secondary_condition, target_identifier)
VALUES
    (?1, ?2, ?3, ?4,
        ?5, ?6, ?7);
";

/// Requires 9 params:
///
/// * `raw_id`
/// * `tile_page_identifier`
/// * `offset_x`
/// * `offset_y`
/// * `offset_x_2`
/// * `offset_y_2`
/// * `primary_condition`
/// * `secondary_condition`
/// * `target_identifier`
pub(super) const INSERT_LARGE_SPRITE_GRAPHIC: &str = r"
INSERT INTO sprite_graphics
    (raw_id, tile_page_identifier, offset_x, offset_y, offset_x_2, offset_y_2,
        primary_condition, secondary_condition, target_identifier)
VALUES
    (?1, ?2, ?3, ?4, ?5, ?6,
        ?7, ?8, ?9);
";

/// Requires 2 params:
///
/// * `raw_id`
/// * `name`
pub(super) const INSERT_LOOKUP_NAME: &str = "INSERT INTO raw_names (raw_id, name) VALUES (?1, ?2);";

/// Requires 3 params:
///
/// * `raw_id`
/// * `names`
/// * `description`
pub(super) const INSERT_SEARCH_INDEX: &str =
    "INSERT OR REPLACE INTO raw_search_index (raw_id, names, description) VALUES (?1, ?2, ?3);";

/// Will insert or overwrite existing, returning the `raw_id`
///
/// Requires 5 params:
///
/// * `raw_type` by name
/// * `identifier`
/// * `module_id`
/// * `data_blob` as raw JSON string
/// * `object_id` as bytes
pub(super) const INSERT_RAW_DEFINITION_WITH_UPDATE_RETURN_ID: &str = r"
INSERT INTO raw_definitions
    (raw_type_id,
        identifier, module_id, data_blob, object_id)
VALUES
    ((SELECT id FROM raw_types WHERE name = ?1),
        ?2, ?3, ?4, ?5)
ON CONFLICT(module_id, identifier) DO UPDATE SET
    data_blob = excluded.data_blob,
    object_id = excluded.object_id
RETURNING id;
";

/// Will insert but not overwrite, returning the `raw_id`
///
/// Requires 5 params:
///
/// * `raw_type` by name
/// * `identifier`
/// * `module_id`
/// * `data_blob` as raw JSON string
/// * `object_id` as bytes
pub(super) const INSERT_RAW_DEFINITION_NO_UPDATE_RETURN_ID: &str = r"
INSERT INTO raw_definitions
    (raw_type_id,
        identifier, module_id, data_blob, object_id)
VALUES
    ((SELECT id FROM raw_types WHERE name = ?1),
        ?2, ?3, ?4, ?5)
ON CONFLICT(module_id, identifier) DO NOTHING
RETURNING id;
";
