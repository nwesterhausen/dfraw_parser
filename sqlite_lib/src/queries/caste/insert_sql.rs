//! Insert SQL queries for the caste tables.
//!
//! These constants contain parameterized `INSERT` statements used by the
//! sqlite backend when persisting parsed caste data. Each constant documents
//! the ordered parameters (by placeholder position) expected when executing
//! the statement.

/// Identity insertion SQL for the `castes` table.
///
/// # Parameters (ordered)
///
/// 1. `creature_id` - FK to `creatures.id`: the parent creature this caste belongs to.
/// 2. `identifier` - the caste's identifier string (e.g., "MALE", "FEMALE", "DEFAULT").
pub const INSERT_IDENTITY: &str = r"
 INSERT INTO castes (creature_id, identifier)
 VALUES (?1, ?2)";

/// Insert SQL for the `caste_tags` table.
///
/// # Parameters (ordered)
///
/// 1. `caste_id` - FK to `castes.id`.
/// 2. `tag_id` - FK to `ref_caste_token_tags.id`.
/// 3. `tag_position` - index/position of this tag within the caste.
pub const INSERT_TAG: &str = r"
 INSERT INTO caste_tags (caste_id, tag_id, tag_position)
 VALUES (?1, ?2, ?3)";

/// Insert SQL for the `caste_value_flags` table.
///
/// This table stores flag values that can be represented as a mix of a boolean,
/// up to seven strings, and up to seven integers.
///
/// # Parameters (ordered)
///
/// 1. `caste_id` - FK to `castes.id`.
/// 2. `tag_id` - FK to `ref_caste_token_tags.id`.
/// 3. `tag_position` - index/position of this tag within the caste.
/// 4. `value_bit` - boolean / bit value (stored as integer).
/// 5. `value_string1`
/// 6. `value_string2`
/// 7. `value_string3`
/// 8. `value_string4`
/// 9. `value_string5`
/// 10. `value_string6`
/// 11. `value_string7`
/// 12. `value_int1`
/// 13. `value_int2`
/// 14. `value_int3`
/// 15. `value_int4`
/// 16. `value_int5`
/// 17. `value_int6`
/// 18. `value_int7`
pub const INSERT_VALUE_TAG: &str = r"
 INSERT INTO caste_value_flags (
     caste_id, tag_id, tag_position,
     value_bit,
     value_string1, value_string2, value_string3, value_string4, value_string5, value_string6, value_string7,
     value_int1, value_int2, value_int3, value_int4, value_int5, value_int6, value_int7
 )
 VALUES (
     ?1, ?2, ?3,
     ?4,
     ?5, ?6, ?7, ?8, ?9, ?10, ?11,
     ?12, ?13, ?14, ?15, ?16, ?17, ?18
 )";

/// Insert SQL for the `caste_attacks` table.
///
/// # Parameters (ordered)
///
/// 1. `caste_id` - FK to `castes.id`.
/// 2. `tag_position` - index/position of this attack tag.
/// 3. `name` - attack name (e.g., "BITE").
/// 4. `body_part` - body part token (e.g., "`BY_TOKEN:MOUTH`").
pub const INSERT_ATTACK_TAG: &str = r"
 INSERT INTO caste_attacks (caste_id, tag_position, name, body_part)
 VALUES (?1, ?2, ?3, ?4)";

/// Insert SQL for the `caste_attack_triggers` table.
///
/// # Parameters (ordered)
///
/// 1. `caste_id` - FK to `castes.id`.
/// 2. `tag_position` - index/position of this trigger tag.
/// 3. `population` - integer value (default 0).
/// 4. `exported_wealth` - integer value (default 0).
/// 5. `created_wealth` - integer value (default 0).
pub const INSERT_ATTACK_TRIGGER_TAG: &str = r"
 INSERT INTO caste_attack_triggers (caste_id, tag_position, population, exported_wealth, created_wealth)
 VALUES (?1, ?2, ?3, ?4, ?5)";

/// Insert SQL for the `caste_blood` table.
///
/// # Parameters (ordered)
///
/// 1. `caste_id` - FK to `castes.id`.
/// 2. `tag_position` - index/position of this blood tag.
/// 3. `material` - dynamic material identifier.
/// 4. `state` - material state string (e.g., "liquid").
pub const INSERT_BLOOD_TAG: &str = r"
 INSERT INTO caste_blood (caste_id, tag_position, material, state)
 VALUES (?1, ?2, ?3, ?4)";

/// Insert SQL for the `caste_body_detail_plans` table (identity row).
///
/// # Parameters (ordered)
///
/// 1. `caste_id` - FK to `castes.id`.
/// 2. `tag_position` - index/position of this body detail plan tag.
/// 3. `name` - plan name.
pub const INSERT_BODY_DETAIL_PLAN_IDENTITY_TAG: &str = r"
 INSERT INTO caste_body_detail_plans (caste_id, tag_position, name)
 VALUES (?1, ?2, ?3)";

/// Insert SQL for the `caste_body_detail_plan_args` table (plan arguments).
///
/// # Parameters (ordered)
///
/// 1. `body_detail_plan_id` - FK to `caste_body_detail_plans.id`.
/// 2. `argument_index` - position of the argument within the plan.
/// 3. `argument` - argument text.
pub const INSERT_BODY_DETAIL_PLAN_ARGUMENT_TAG: &str = r"
 INSERT INTO caste_body_detail_plan_args (body_detail_plan_id, argument_index, argument)
 VALUES (?1, ?2, ?3)";

/// Insert SQL for the `caste_color_tags` table.
///
/// # Parameters (ordered)
///
/// 1. `caste_id` - FK to `castes.id`.
/// 2. `tag_id` - FK to `ref_caste_token_tags.id`.
/// 3. `color_id` - FK to `colors.id`.
/// 4. `tag_position` - index/position of this tag.
pub const INSERT_COLOR_TAG: &str = r"
 INSERT INTO caste_color_tags (caste_id, tag_id, color_id, tag_position)
 VALUES (?1, ?2, ?3, ?4)";

/// Insert SQL for the `caste_item_tags` table.
///
/// # Parameters (ordered)
///
/// 1. `caste_id` - FK to `castes.id`.
/// 2. `tag_id` - FK to `ref_caste_token_tags.id`.
/// 3. `tag_position` - index/position of this tag.
/// 4. `dyn_item_id` - FK to `dyn_items_of_material.id`.
pub const INSERT_ITEM_TAG: &str = r"
 INSERT INTO caste_item_tags (caste_id, tag_id, tag_position, dyn_item_id)
 VALUES (?1, ?2, ?3, ?4)";

/// Insert SQL for the `caste_material_tags` table.
///
/// # Parameters (ordered)
///
/// 1. `caste_id` - FK to `castes.id`.
/// 2. `tag_id` - FK to `ref_caste_token_tags.id`.
/// 3. `tag_position` - index/position of this tag.
/// 4. `dyn_material_id` - FK to `dyn_materials_in_state.id`.
pub const INSERT_MATERIAL_TAG: &str = r"
 INSERT INTO caste_material_tags (caste_id, tag_id, tag_position, dyn_material_id)
 VALUES (?1, ?2, ?3, ?4)";

/// Insert SQL for the `caste_creature_caste_tags` table.
///
/// # Parameters (ordered)
///
/// 1. `caste_id` - FK to `castes.id`.
/// 2. `tag_id` - FK to `ref_caste_token_tags.id`.
/// 3. `dyn_creature_caste_id` - FK to `dyn_creature_caste_tags.id`.
/// 4. `tag_position` - index/position of this tag.
pub const INSERT_CREATURE_CASTE_TAG: &str = r"
 INSERT INTO caste_creature_caste_tags (caste_id, tag_id, dyn_creature_caste_id, tag_position)
 VALUES (?1, ?2, ?3, ?4)";

/// Insert SQL for the `caste_lairs` table (lair references).
///
/// # Parameters (ordered)
///
/// 1. `caste_id` - FK to `castes.id`.
/// 2. `tag_id` - FK to `ref_caste_token_tags.id`.
/// 3. `tag_position` - index/position of this tag.
/// 4. `lair_id` - FK to `ref_lair_token_flags.id` (the lair token).
/// 5. `probability` - integer probability value.
pub const INSERT_LAIR_REF_TAG: &str = r"
 INSERT INTO caste_lair_tags (caste_id, tag_id, tag_position, lair_id, probability)
 VALUES (?1, ?2, ?3, ?4, ?5)";

/// Insert SQL for the `caste_names` table.
///
/// # Parameters (ordered)
///
/// 1. `caste_id` - FK to `castes.id`.
/// 2. `tag_id` - FK to `ref_caste_token_tags.id`.
/// 3. `tag_position` - index/position of this tag.
/// 4. `name_id` - FK to `dyn_names.id`.
pub const INSERT_NAME_TAG: &str = r"
 INSERT INTO caste_names (caste_id, tag_id, tag_position, name_id)
 VALUES (?1, ?2, ?3, ?4)";

/// Insert SQL for the `caste_profession_names` table.
///
/// # Parameters (ordered)
///
/// 1. `caste_id` - FK to `castes.id`.
/// 2. `tag_id` - FK to `ref_caste_token_tags.id`.
/// 3. `tag_position` - index/position of this tag.
/// 4. `caste_name_id` - FK to `caste_names.id`.
/// 5. `profession_identifier` - profession string identifier.
pub const INSERT_PROFESSION_NAME_TAG: &str = r"
 INSERT INTO caste_profession_names (caste_id, tag_id, tag_position, caste_name_id, profession_identifier)
 VALUES (?1, ?2, ?3, ?4, ?5)";

/// Insert SQL for the `caste_secretions` table.
///
/// # Parameters (ordered)
///
/// 1. `caste_id` - FK to `castes.id`.
/// 2. `tag_id` - FK to `ref_caste_token_tags.id`.
/// 3. `tag_position` - index/position of this tag.
/// 4. `dyn_material_id` - FK to `dyn_materials_in_state.id`.
/// 5. `dyn_body_part_group_id` - FK to `dyn_body_part_groups.id`.
/// 6. `tissue_layer` - string identifying the tissue layer.
/// 7. `secretion_trigger_id` - FK to `ref_secretion_triggers.id`.
pub const INSERT_SECRETION_TAG: &str = r"
 INSERT INTO caste_secretions (caste_id, tag_id, tag_position, dyn_material_id,
     dyn_body_part_group_id, tissue_layer, secretion_trigger_id)
 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)";

/// Insert SQL for the `caste_specific_foods` table.
///
/// # Parameters (ordered)
///
/// 1. `caste_id` - FK to `castes.id`.
/// 2. `tag_id` - FK to `ref_caste_token_tags.id`.
/// 3. `tag_position` - index/position of this tag.
/// 4. `ref_object_type_id` - FK to `ref_object_types.id` (object type of the food).
/// 5. `object_identifier` - identifier string of the object.
pub const INSERT_SPECIFIC_FOOD_TAG: &str = r"
 INSERT INTO caste_specific_foods (caste_id, tag_id, tag_position,
     ref_object_type_id, object_identifier)
 VALUES (?1, ?2, ?3, ?4, ?5)";
