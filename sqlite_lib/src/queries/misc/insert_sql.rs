pub const INSERT_DYNAMIC_ITEM_OF_MATERIAL: &str = r"
INSERT INTO dyn_items_of_material (item_identifier, material_identifier)
VALUES (?1, ?2);";
pub const INSERT_DYNAMIC_MATERIAL_IN_STATE: &str = r"
INSERT INTO dyn_items_of_material (material_identifier, state)
VALUES (?1, ?2);";
pub const INSERT_CREATURE_CASTE_TAG_REFERENCE: &str = r"
INSERT INTO dyn_creature_caste_tags (creature_identifier, caste_identifier)
VALUES (?1, ?2);";
pub const INSERT_DYNAMIC_NAME: &str = r"
INSERT INTO dyn_names (singular, plural)
VALUES (?1, ?2);";
pub const INSERT_DYNAMIC_NAME_WITH_ADJECTIVE: &str = r"
INSERT INTO dyn_names (singular, plural, adjective)
VALUES (?1, ?2, ?3);";
pub const INSERT_BODY_PART_GROUP: &str = r"
INSERT INTO dyn_body_part_groups (body_part_selector, body_part)
VALUES (?1, ?2)";
