pub const GET_DYNAMIC_ITEM_OF_MATERIAL_ID: &str = r"
SELECT id FROM dyn_items_of_material
WHERE item_identifier = ?1 AND material_identifier = ?2;";
pub const GET_DYNAMIC_MATERIAL_IN_STATE_ID: &str = r"
SELECT id FROM dyn_items_of_material
WHERE material_identifier = ?1 AND state = ?2;";
pub const GET_CREATURE_CASTE_TAG_ID: &str = r"
SELECT id FROM dyn_creature_caste_tags
WHERE creature_identifier = ?1 AND caste_identifier = ?2;";
pub const GET_NAME_ID_BY_SINGULAR_PLURAL: &str = r"
SELECT id FROM dyn_names
WHERE singular = ?1 AND plural = ?2;";
pub const GET_NAME_ID_BY_SINGULAR_PLURAL_ADJECTIVE: &str = r"
SELECT id FROM dyn_names
WHERE singular = ?1
 AND  plural = ?2
 AND  adjective = ?3;";
pub const GET_BODY_PART_GROUP_ID: &str = r"
SELECT id FROM dyn_body_part_groups
WHERE body_part_selector = ?1 AND body_part = ?2;";

pub const GET_REF_LAIR_TAG_ID: &str = r"
SELECT id FROM ref_lair_token_flags
WHERE token = ?1;";
pub const GET_REF_SECRETION_TAG_ID: &str = r"
SELECT id FROM ref_secretion_triggers
WHERE token = ?1;";
pub const GET_REF_OBJECT_TYPE: &str = r"
SELECT id FROM ref_object_types
WHERE token = ?1;";
