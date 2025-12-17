/// SQL statement to create the `ref_object_types` table.
///
/// This table represents `[dfraw_parser::raw_definitions::tokens::ObjectType]`
pub const REF_OBJECT_TYPE_TABLE: &str = r"
CREATE TABLE ref_object_types (
    id INTEGER PRIMARY KEY,
    token TEXT NOT NULL UNIQUE  -- e.g., 'CREATURE', 'ITEM_WEAPON'
    --,name TEXT NOT NULL           -- e.g., 'Creature', 'Weapon (Item)'
);";

/// SQL statement to create the `ref_module_locations` table.
///
/// This table represents the locations of modules in the game.
pub const REF_MODULE_LOCATIONS_TABLE: &str = r"
CREATE TABLE ref_module_locations (
    id INTEGER PRIMARY KEY,
    token TEXT NOT NULL UNIQUE,  -- e.g., 'Vanilla', 'Mods'
    path_fragment TEXT           -- e.g., 'data/vanilla', 'mods'
);";

/// SQL statement to create the `ref_biomes` table.
///
/// This table represents the `[dfraw_parser::raw_definitions::tokens::Biome]`
pub const REF_BIOMES_TABLE: &str = r"
CREATE TABLE ref_biomes (
    id INTEGER PRIMARY KEY,
    token TEXT NOT NULL UNIQUE,  -- e.g., 'GLACIER', 'ANY_LAND'
    name TEXT NOT NULL           -- e.g., 'Glacier', 'Any Land Biome'
);";

/// SQL statement to create the `ref_caste_token_flags` table.
///
/// This table represents
pub const REF_CASTE_TOKEN_FLAGS: &str = r"
CREATE TABLE ref_caste_token_flags (
    id INTEGER PRIMARY KEY,
    token TEXT NOT NULL UNIQUE  -- e.g., 'AMPHIBIOUS', 'FLIER', 'WEBIMMUNE'
);";

pub const REF_LAIR_TOKEN_FLAGS: &str = r"
CREATE TABLE ref_lair_token_flags (
    id INTEGER PRIMARY KEY,
    token TEXT NOT NULL UNIQUE
);";
pub const REF_SECRETION_TRIGGERS: &str = r"
CREATE TABLE ref_secretion_triggers (
    id INTEGER PRIMARY KEY,
    token TEXT NOT NULL UNIQUE
);";
