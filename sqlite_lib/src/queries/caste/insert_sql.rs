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
pub const INSERT_CASTE_FLAG: &str = r"
INSERT INTO caste_flags (caste_id, flag_id)
VALUES (?1, ?2)";

/// Insert SQL for the `caste_value_flags` table when the value is a string.
///
/// Expects parameters:
///
/// 1. `caste_id`
/// 2. `flag_id`
/// 3. `value_string`
pub const INSERT_CASTE_STRING_FLAG: &str = r"
INSERT INTO caste_value_flags (caste_id, flag_id, value_string)
VALUES (?1, ?2, ?3)";

/// Insert SQL for the `caste_value_flags` table when the value is an integer.
///
/// Expects parameters:
///
/// 1. `caste_id`
/// 2. `flag_id`
/// 3. `value_int`
pub const INSERT_CASTE_INTEGER_FLAG: &str = r"
INSERT INTO caste_value_flags (caste_id, flag_id, value_int)
VALUES (?1, ?2, ?3)";
