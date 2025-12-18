//! Misc dynamic-table DDL and documentation.

/// `dyn_materials_in_state`
///
/// Represents dynamic materials in a specific state (e.g., a material token
/// and whether it is `solid`, `liquid`, etc.).
///
/// Columns:
/// - `id` (INTEGER PRIMARY KEY)
/// - `material_identifier` (TEXT NOT NULL) — material token/identifier
/// - `state` (TEXT DEFAULT 'solid') — material state string
pub const MATERIALS_IN_STATE_TABLE: &str = r"
CREATE TABLE dyn_materials_in_state (
    id INTEGER PRIMARY KEY,
    material_identifier TEXT NOT NULL,
    state TEXT DEFAULT 'solid'
);";

/// `dyn_items_of_material`
///
/// Represents dynamic item definitions that are associated with a specific
/// material. Useful for modelling item variants created from a material.
///
/// Columns:
/// - `id` (INTEGER PRIMARY KEY)
/// - `item_identifier` (TEXT NOT NULL) — token identifying the item
/// - `material_identifier` (TEXT NOT NULL) — token identifying the material
pub const ITEMS_OF_MATERIAL_TABLE: &str = r"
CREATE TABLE dyn_items_of_material (
    id INTEGER PRIMARY KEY,
    item_identifier TEXT NOT NULL,
    material_identifier TEXT NOT NULL
);";

/// `dyn_creature_caste_tags`
///
/// Dynamic creature/caste tag registry used by caste-related tag rows that
/// refer to a creature/caste by a dynamic identifier pair.
///
/// Columns:
/// - `id` (INTEGER PRIMARY KEY)
/// - `creature_identifier` (TEXT NOT NULL) — creature token / identifier
/// - `caste_identifier` (TEXT NOT NULL) — caste token / identifier
pub const CREATURE_CASTE_TABLE: &str = r"
CREATE TABLE dyn_creature_caste_tags (
    id INTEGER PRIMARY KEY,
    creature_identifier TEXT NOT NULL,
    caste_identifier TEXT NOT NULL
);";

/// `dyn_names`
///
/// Stores dynamic name records used by caste and other tables that reference
/// names (singular, plural, adjective).
///
/// Columns:
/// - `id` (INTEGER PRIMARY KEY)
/// - `singular` (TEXT NOT NULL) — singular form of the name
/// - `plural` (TEXT) — optional plural form
/// - `adjective` (TEXT) — optional adjective form
pub const NAMES_TABLE: &str = r"
CREATE TABLE dyn_names (
    id INTEGER PRIMARY KEY,
    singular TEXT NOT NULL,
    plural TEXT,
    adjective TEXT
);";

/// `dyn_body_part_groups`
///
/// Represents dynamic body-part group selectors and their associated token(s).
/// Each row maps a selector string to a specific body part token.
///
/// Columns:
/// - `id` (INTEGER PRIMARY KEY)
/// - `body_part_selector` (TEXT NOT NULL) — selector or group token
/// - `body_part` (TEXT NOT NULL) — resolved body part token/key
pub const BODY_PART_GROUPS_TABLE: &str = r"
CREATE TABLE dyn_body_part_groups (
    id INTEGER PRIMARY KEY,
    body_part_selector TEXT NOT NULL,
    body_part TEXT NOT NULL
);";
