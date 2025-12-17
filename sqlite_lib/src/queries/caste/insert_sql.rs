//! Insert SQL queries for the caste tables

/// Identity insertion SQL for `castes`
///
/// Expects parameters:
///
/// 1. `creature_id`: the id of the parent `[dfraw_parser::Creature]` in the creatures table
/// 2. `identifier`: the `[Caste]`'s identifier string
pub const INSERT_CASTE_IDENTITY: &str = r"
INSERT INTO castes (creature_id, identitifer)
VALUES (?1, ?2)";

/// Insert SQL for the `caste_flags` table.
///
/// Expects parameters:
///
/// 1. `caste_id`
/// 2. `flag_id`
/// 3. `position` index of the flag in the caste
pub const INSERT_CASTE_FLAG: &str = r"
INSERT INTO caste_flags (caste_id, flag_id, position)
VALUES (?1, ?2, ?3)";

/// Insert SQL for the `caste_value_flags` table when the value is a string.
///
/// Expects parameters:
///
/// 1. `caste_id`
/// 2. `flag_id`
/// 3. `position` index of the flag in the caste
/// 4. `value_string1`
pub const INSERT_CASTE_BOOLEAN_FLAG: &str = r"
INSERT INTO caste_value_flags (caste_id, flag_id, position, value_bit)
VALUES (?1, ?2, ?3, ?4)";

/// Insert SQL for the `caste_value_flags` table when the value is a string.
///
/// Expects parameters:
///
/// 1. `caste_id`
/// 2. `flag_id`
/// 3. `position` index of the flag in the caste
/// 4. `value_string1`
pub const INSERT_CASTE_STRING_FLAG: &str = r"
INSERT INTO caste_value_flags (caste_id, flag_id, position, value_string1)
VALUES (?1, ?2, ?3, ?4)";

/// Insert SQL for the `caste_value_flags` table when the value is an integer.
///
/// Expects parameters:
///
/// 1. `caste_id`
/// 2. `flag_id`
/// 3. `position` index of the flag in the caste
/// 4. `value_int1`
pub const INSERT_CASTE_INTEGER_FLAG: &str = r"
INSERT INTO caste_value_flags (caste_id, flag_id, position, value_int1)
VALUES (?1, ?2, ?3, ?4)";

/// Insert SQL for the `caste_value_flags` table when the value is an integer.
///
/// Expects parameters:
///
/// 1. `caste_id`
/// 2. `flag_id`
/// 3. `position` index of the flag in the caste
/// 4. `value_int1`
/// 5. `value_int2`
pub const INSERT_CASTE_MIN_MAX_FLAG: &str = r"
INSERT INTO caste_value_flags (caste_id, flag_id, position, value_int1, value_int2)
VALUES (?1, ?2, ?3, ?4, ?5)";

/// Insert SQL for the `caste_value_flags` table when the value is an integer.
///
/// Expects parameters:
///
/// 1. `caste_id`
/// 2. `flag_id`
/// 3. `position` index of the flag in the caste
/// 4. `value_string1` supplied string value with range (if applicable)
/// 5. `value_int1` first number in spread range
/// 6. `value_int2` first number in spread range
/// 7. `value_int3` first number in spread range
/// 8. `value_int4` first number in spread range
/// 9. `value_int5` first number in spread range
/// 10. `value_int6` first number in spread range
/// 11. `value_int7` first number in spread range
pub const INSERT_CASTE_7SPREAD_RANGE_FLAG: &str = r"
INSERT INTO caste_value_flags (caste_id, flag_id, position, value_string1,
    value_int1, value_int2, value_int3, value_int4, value_int5,
    value_int6, value_int7)
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11);";

pub const INSERT_CASTE_STRING_INTEGER_FLAG: &str = r"
INSERT INTO caste_value_flags (caste_id, flag_id, position, value_string1, value_int1)
VALUES (?1, ?2, ?3, ?4, ?5);";
pub const INSERT_CASTE_STRING_INTEGER_INTEGER_FLAG: &str = r"
INSERT INTO caste_value_flags (caste_id, flag_id, position, value_string1, value_int1, value_int2)
VALUES (?1, ?2, ?3, ?4, ?5, ?6);";
pub const INSERT_CASTE_STRING_INTEGER_INTEGER_INTEGER_FLAG: &str = r"
INSERT INTO caste_value_flags (caste_id, flag_id, position, value_string1, value_int1, value_int2, value_int3)
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);";
pub const INSERT_CASTE_STRING_INTEGER_INTEGER_INTEGER_INTEGER_FLAG: &str = r"
INSERT INTO caste_value_flags (caste_id, flag_id, position, value_string1, value_int1, value_int2, value_int3, value_int4)
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);";

pub const INSERT_CASTE_STRING_STRING_INTEGER_FLAG: &str = r"
INSERT INTO caste_value_flags (caste_id, flag_id, position, value_string1, value_string2, value_int1)
VALUES (?1, ?2, ?3, ?4, ?5, ?6);";
pub const INSERT_CASTE_STRING_STRING_INTEGER_INTEGER_FLAG: &str = r"
INSERT INTO caste_value_flags (caste_id, flag_id, position, value_string1, value_string2, value_int1, value_int2)
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);";
pub const INSERT_CASTE_STRING_STRING_INTEGER_INTEGER_INTEGER_FLAG: &str = r"
INSERT INTO caste_value_flags (caste_id, flag_id, position, value_string1, value_string2, value_int1, value_int2, value_int3)
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);";

pub const INSERT_CASTE_STRING_STRING_FLAG: &str = r"
INSERT INTO caste_value_flags (caste_id, flag_id, position, value_string1, value_string2)
VALUES (?1, ?2, ?3, ?4, ?5);";
pub const INSERT_CASTE_STRING_STRING_STRING_FLAG: &str = r"
INSERT INTO caste_value_flags (caste_id, flag_id, position, value_string1, value_string2, value_string3)
VALUES (?1, ?2, ?3, ?4, ?5, ?6);";
pub const INSERT_CASTE_STRING_STRING_STRING_STRING_FLAG: &str = r"
INSERT INTO caste_value_flags (caste_id, flag_id, position, value_string1, value_string2, value_string3, value_string4)
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);";
pub const INSERT_CASTE_STRING_STRING_STRING_STRING_STRING_FLAG: &str = r"
INSERT INTO caste_value_flags (caste_id, flag_id, position, value_string1, value_string2, value_string3, value_string4, value_string5)
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);";
pub const INSERT_CASTE_STRING_STRING_STRING_STRING_STRING_STRING_FLAG: &str = r"
INSERT INTO caste_value_flags (caste_id, flag_id, position, value_string1, value_string2, value_string3, value_string4, value_string5, value_string6)
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9);";

pub const INSERT_CASTE_INTEGER_INTEGER_FLAG: &str = r"
INSERT INTO caste_value_flags (caste_id, flag_id, position, value_int1, value_int2)
VALUES (?1, ?2, ?3, ?4, ?5)";
pub const INSERT_CASTE_INTEGER_INTEGER_INTEGER_FLAG: &str = r"
INSERT INTO caste_value_flags (caste_id, flag_id, position, value_int1, value_int2, value_int3)
VALUES (?1, ?2, ?3, ?4, ?5, ?6)";
pub const INSERT_CASTE_ATTACK: &str = r"
INSERT INTO caste_attacks (caste_id, tag_position, name, body_part)
VALUES (?1, ?2, ?3, ?4);";

pub const INSERT_CASTE_ATTACK_TRIGGER: &str = r"
INSERT INTO caste_attack_triggers (caste_id, tag_position, population, exported_wealth, created_wealth)
VALUES (?1, ?2, ?3, ?4, ?5);";

pub const INSERT_CASTE_BLOOD: &str = r"
INSERT INTO caste_blood (caste_id, tag_position, material, state)
VALUES (?1, ?2, ?3, ?4);";

pub const INSERT_BODY_DETAIL_PLAN_IDENTITY: &str = r"
INSERT INTO caste_body_detail_plans (caste_id, tag_position, name)
VALUES (?1, ?2, ?3);";

pub const INSERT_BODY_DETAIL_PLAN_ARGUMENT: &str = r"
INSERT INTO caste_body_detail_plan_args (body_detail_plan_id, argument_index, argument)
VALUES (?1, ?2, ?3);";

pub const INSERT_CASTE_COLOR_TAG: &str = r"
INSERT INTO caste_color_tags (caste_id, flag_id, color_id, tag_position)
VALUES (?1, ?2, ?3, ?4);";

pub const INSERT_CASTE_ITEM_TAG: &str = r"
INSERT INTO caste_ebo_items (caste_id, flag_id, dyn_item_id, tag_position)
VALUES (?1, ?2, ?3, ?4);";

pub const INSERT_CASTE_MATERIAL_TAG: &str = r"
INSERT INTO caste_egg_materials (caste_id, flag_id, dyn_material_id, tag_position)
VALUES (?1, ?2, ?3, ?4);";

pub const INSERT_CASTE_CREATURE_CASTE_REF_TAG: &str = r"
INSERT INTO caste_creature_caste_references (caste_id, flag_id, dyn_creature_caste_id, tag_position)
VALUES (?1, ?2, ?3, ?4);";

pub const INSERT_CASTE_LAIR_REF_TAG: &str = r"
INSERT INTO caste_lairs (caste_id, flag_id, lair_id, tag_position, probability)
VALUES (?1, ?2, ?3, ?4, ?5);";
pub const INSERT_CASTE_NAME_TAG: &str = r"
INSERT INTO caste_names (caste_id, flag_id, name_id, tag_position)
VALUES (?1, ?2, ?3, ?4);";
pub const INSERT_CASTE_PROFESSION_NAME_TAG: &str = r"
INSERT INTO caste_profession_names (caste_name_id, profession_identifier, flag_id, tag_position)
VALUES (?1, ?2, ?3, ?4);";
pub const INSERT_CASTE_SECRETION: &str = r"
INSERT INTO caste_secretions (caste_id, flag_id, tag_position, dyn_material_id,
    dyn_body_part_group_id, tissue_layer, secretion_trigger_id)
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);";
pub const INSERT_SPECIFIC_FOOD: &str = r"
INSERT INTO caste_specific_foods (caste_id, flag_id, tag_position,
    ref_object_type_id, object_identifier)
VALUES (?1, ?2, ?3, ?4, ?5);";
