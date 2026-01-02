/// Insert SQL for the `dyn_items_of_material` table mapping an item to a material.
///
/// Expects parameters:
///
/// 1. `item_identifier`: identifier string for the dynamic item
/// 2. `material_identifier`: identifier string for the material
pub const INSERT_DYNAMIC_ITEM_OF_MATERIAL: &str = r"
INSERT INTO dyn_items_of_material (item_identifier, material_identifier)
VALUES (?1, ?2);";

/// Insert SQL for the `dyn_items_of_material` table mapping a material to a state.
///
/// Expects parameters:
///
/// 1. `material_identifier`: identifier string for the material
/// 2. `state`: textual representation of the material state
pub const INSERT_DYNAMIC_MATERIAL_IN_STATE: &str = r"
INSERT INTO dyn_items_of_material (material_identifier, state)
VALUES (?1, ?2);";

/// Insert SQL for the `dyn_creature_caste_tags` table referencing a creature and caste.
///
/// Expects parameters:
///
/// 1. `creature_identifier`: identifier string for the dynamic creature
/// 2. `caste_identifier`: identifier string for the dynamic caste
pub const INSERT_CREATURE_CASTE_TAG_REFERENCE: &str = r"
INSERT INTO dyn_creature_caste_tags (creature_identifier, caste_identifier)
VALUES (?1, ?2);";

/// Insert SQL for the `dyn_names` table (singular/plural).
///
/// Expects parameters:
///
/// 1. `singular`: singular form of the name
/// 2. `plural`: plural form of the name
pub const INSERT_DYNAMIC_NAME: &str = r"
INSERT INTO dyn_names (singular, plural)
VALUES (?1, ?2);";

/// Insert SQL for the `dyn_names` table including an adjective.
///
/// Expects parameters:
///
/// 1. `singular`: singular form of the name
/// 2. `plural`: plural form of the name
/// 3. `adjective`: associated adjective string
pub const INSERT_DYNAMIC_NAME_WITH_ADJECTIVE: &str = r"
INSERT INTO dyn_names (singular, plural, adjective)
VALUES (?1, ?2, ?3);";

/// Insert SQL for the `dyn_body_part_groups` table mapping a selector to a body part.
///
/// Expects parameters:
///
/// 1. `body_part_selector`: selector or identifier for the body part group
/// 2. `body_part`: identifier or name of the body part
pub const INSERT_BODY_PART_GROUP: &str = r"
INSERT INTO dyn_body_part_groups (body_part_selector, body_part)
VALUES (?1, ?2)";
