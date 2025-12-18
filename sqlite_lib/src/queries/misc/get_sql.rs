//! Get SQL Commands for misc queries.

/// Selects the `id` from `dyn_items_of_material` for a given item and material.
///
/// Expects parameters:
///
/// 1. `item_identifier` - bound to ?1
/// 2. `material_identifier` - bound to ?2
pub const GET_DYNAMIC_ITEM_OF_MATERIAL_ID: &str = r"
SELECT id FROM dyn_items_of_material
WHERE item_identifier = ?1 AND material_identifier = ?2;";

/// Selects the `id` from `dyn_items_of_material` for a given material in a state.
///
/// Expects parameters:
///
/// 1. `material_identifier` - bound to ?1
/// 2. `state` - bound to ?2
pub const GET_DYNAMIC_MATERIAL_IN_STATE_ID: &str = r"
SELECT id FROM dyn_items_of_material
WHERE material_identifier = ?1 AND state = ?2;";

/// Selects the `id` from `dyn_creature_caste_tags` for a creature/caste pair.
///
/// Expects parameters:
///
/// 1. `creature_identifier` - bound to ?1
/// 2. `caste_identifier` - bound to ?2
pub const GET_CREATURE_CASTE_TAG_ID: &str = r"
SELECT id FROM dyn_creature_caste_tags
WHERE creature_identifier = ?1 AND caste_identifier = ?2;";

/// Selects the `id` from `dyn_names` matching singular and plural.
///
/// Expects parameters:
///
/// 1. `singular` - bound to ?1
/// 2. `plural` - bound to ?2
pub const GET_NAME_ID_BY_SINGULAR_PLURAL: &str = r"
SELECT id FROM dyn_names
WHERE singular = ?1 AND plural = ?2;";

/// Selects the `id` from `dyn_names` matching singular, plural, and adjective.
///
/// Expects parameters:
///
/// 1. `singular` - bound to ?1
/// 2. `plural` - bound to ?2
/// 3. `adjective` - bound to ?3
pub const GET_NAME_ID_BY_SINGULAR_PLURAL_ADJECTIVE: &str = r"
SELECT id FROM dyn_names
WHERE singular = ?1
 AND  plural = ?2
 AND  adjective = ?3;";

/// Selects the `id` from `dyn_body_part_groups` for a selector
/// or `body_part` pair.
///
/// Expects parameters:
///
/// 1. `body_part_selector` - bound to ?1
/// 2. `body_part` - bound to ?2
pub const GET_BODY_PART_GROUP_ID: &str = r"
SELECT id FROM dyn_body_part_groups
WHERE body_part_selector = ?1 AND body_part = ?2;";

/// Selects the `id` from `ref_lair_token_flags` by token.
///
/// Expects parameters:
///
/// 1. `token` - bound to ?1
pub const GET_REF_LAIR_TAG_ID: &str = r"
SELECT id FROM ref_lair_token_flags
WHERE token = ?1;";

/// Selects the `id` from `ref_secretion_triggers` by token.
///
/// Expects parameters:
///
/// 1. `token` - bound to ?1
pub const GET_REF_SECRETION_TAG_ID: &str = r"
SELECT id FROM ref_secretion_triggers
WHERE token = ?1;";

/// Selects the `id` from `ref_object_types` by token.
///
/// Expects parameters:
///
/// 1. `token` - bound to ?1
pub const GET_REF_OBJECT_TYPE: &str = r"
SELECT id FROM ref_object_types
WHERE token = ?1;";
