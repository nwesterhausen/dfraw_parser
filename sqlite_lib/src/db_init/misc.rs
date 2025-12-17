pub const MATERIALS_IN_STATE_TABLE: &str = r"
CREATE TABLE dyn_materials_in_state (
    id INTEGER PRIMARY KEY,
    material_identifier TEXT NOT NULL,
    state TEXT DEFAULT 'solid'
);";

pub const ITEMS_OF_MATERIAL_TABLE: &str = r"
CREATE TABLE dyn_items_of_material (
    id INTEGER PRIMARY KEY,
    item_identifier TEXT NOT NULL,
    material_identifier TEXT NOT NULL
);";

pub const CREATURE_CASTE_TABLE: &str = r"
CREATE TABLE dyn_creature_caste_tags (
    id INTEGER PRIMARY KEY,
    creature_identifier TEXT NOT NULL,
    caste_identifier TEXT NOT NULL
);";

pub const NAMES_TABLE: &str = r"
CREATE TABLE dyn_names (
    id INTEGER PRIMARY KEY,
    singular TEXT NOT NULL,
    plural TEXT,
    adjective TEXT
);";

pub const BODY_PART_GROUPS_TABLE: &str = r"
CREATE TABLE dyn_body_part_groups (
    id INTEGER PRIMARY KEY,
    body_part_selector TEXT NOT NULL,
    body_part TEXT NOT NULL
);";
