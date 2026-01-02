//! SQL for GET queries on the various reference tables.

/// SQL to GET the `flag_id` of a caste token by the `token` (name)
///
/// Expects parameters:
///
/// 1. `token`: the name of the token (e.g. 'AMPHIBIOUS' or 'BABY')
pub const GET_CASTE_FLAG_BY_TOKEN: &str = r"
SELECT id FROM ref_caste_token_flags
WHERE token = ?1";
