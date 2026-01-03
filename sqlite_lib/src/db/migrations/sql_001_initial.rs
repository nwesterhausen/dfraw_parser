pub const UP: &str = r"
BEGIN;
CREATE TABLE module_locations (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);
INSERT INTO module_locations VALUES
    (1, 'Vanilla'),
    (2, 'Workshop Mods'),
    (3, 'Installed Mods'),
    -- For any module loaded from some arbitrary location instead of one of the 3 recognized ones
    (4, 'Unknown');

CREATE TABLE modules (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    identifier TEXT NOT NULL,
    version INTEGER NOT NULL,
    display_version TEXT,
    earliest_compatible_version INTEGER,
    earliest_compatible_display_version TEXT,
    author TEXT DEFAULT 'unknown',
    description TEXT,
    module_directory_path TEXT NOT NULL,
    module_location_id INTEGER NOT NULL,
    steam_file_id INTEGER,
    steam_title TEXT,
    steam_description TEXT,
    steam_changelog TEXT,
    FOREIGN KEY(module_location_id) REFERENCES module_locations(id)
);

CREATE TABLE load_order (
    position INTEGER PRIMARY KEY,
    module_id INTEGER NOT NULL UNIQUE,
    -- Ensure we don't accidentally link to a non-existent module
    FOREIGN KEY(module_id) REFERENCES modules(id) ON DELETE CASCADE
);

CREATE TABLE steam_metadata (
    module_id INTEGER NOT NULL,
    metadata TEXT
);

CREATE TABLE steam_tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);

CREATE TABLE module_steam_tags (
    module_id INTEGER NOT NULL,
    steam_tag_id INTEGER NOT NULL,
    PRIMARY KEY(module_id, steam_tag_id),
    FOREIGN KEY(module_id) REFERENCES modules(id),
    FOREIGN KEY(steam_tag_id) REFERENCES steam_tags(id)
);

CREATE TABLE steam_key_value_keys (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);

CREATE TABLE module_steam_key_value_pairs (
    module_id INTEGER NOT NULL,
    steam_key_value_id INTEGER NOT NULL,
    PRIMARY KEY(module_id, steam_key_value_id),
    FOREIGN KEY(module_id) REFERENCES modules(id),
    FOREIGN KEY(steam_key_value_id) REFERENCES steam_key_value_keys(id)
);

CREATE TABLE module_restriction_rules (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);
INSERT INTO module_restriction_rules (id, name) VALUES
    (1, 'REQUIRES'),
    (2, 'CONFLICTS'),
    (3, 'BEFORE'),
    (4, 'AFTER');

CREATE TABLE module_dependencies (
    module_id INTEGER NOT NULL,            -- The mod that has the rule
    target_identifier TEXT NOT NULL,       -- The identifier it points to
    restriction_type_id INTEGER NOT NULL,  -- Link to module_restriction_rules
    PRIMARY KEY(module_id, target_identifier, restriction_type_id),
    FOREIGN KEY(module_id) REFERENCES modules(id),
    FOREIGN KEY(restriction_type_id) REFERENCES module_restriction_rules(id)
);

CREATE TABLE raw_types (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);
INSERT INTO raw_types (id, name) VALUES
    (1, 'CREATURE'),
    (2, 'INORGANIC'),
    (3, 'PLANT'),
    (4, 'ITEM'),
    (5, 'ITEM_AMMO'),
    (6, 'ITEM_ARMOR'),
    (7, 'ITEM_FOOD'),
    (8, 'ITEM_GLOVES'),
    (9, 'ITEM_HELM'),
    (10, 'ITEM_INSTRUMENT'),
    (11, 'ITEM_PANTS'),
    (12, 'ITEM_SHIELD'),
    (13, 'ITEM_SHOES'),
    (14, 'ITEM_SIEGEAMMO'),
    (15, 'ITEM_TOOL'),
    (16, 'ITEM_TOY'),
    (17, 'ITEM_TRAPCOMP'),
    (18, 'ITEM_WEAPON'),
    (19, 'BUILDING'),
    (20, 'BUILDING_WORKSHOP'),
    (21, 'BUILDING_FURNACE'),
    (22, 'REACTION'),
    (23, 'GRAPHICS'),
    (24, 'MATERIAL_TEMPLATE'),
    (25, 'BODY_DETAIL_PLAN'),
    (26, 'BODY'),
    (27, 'ENTITY'),
    (28, 'LANGUAGE'),
    (29, 'TRANSLATION'),
    (30, 'TISSUE_TEMPLATE'),
    (31, 'CREATURE_VARIATION'),
    (32, 'TEXT_SET'),
    (33, 'TILE_PAGE'),
    (34, 'DESCRIPTOR_COLOR'),
    (35, 'DESCRIPTOR_PATTERN'),
    (36, 'DESCRIPTOR_SHAPE'),
    (37, 'PALETTE'),
    (38, 'MUSIC'),
    (39, 'SOUND'),
    (40, 'INTERACTION');

CREATE TABLE raw_definitions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    raw_type_id INTEGER NOT NULL,
    identifier TEXT NOT NULL,
    module_id INTEGER NOT NULL,
    data_blob BLOB NOT NULL,
    FOREIGN KEY(module_id) REFERENCES modules(id),
    FOREIGN KEY(raw_type_id) REFERENCES raw_types(id)
);

CREATE TABLE common_raw_flags (
    raw_id INTEGER NOT NULL,
    token_name TEXT NOT NULL,
    PRIMARY KEY(raw_id, token_name),
    FOREIGN KEY(raw_id) REFERENCES raw_definitions(id)
);

CREATE TABLE common_raw_flags_with_numeric_value (
    raw_id INTEGER NOT NULL,
    token_name TEXT NOT NULL,
    value INTEGER NOT NULL,
    FOREIGN KEY(raw_id) REFERENCES raw_definitions(id)
);

CREATE INDEX idx_module_versions ON modules(identifier, version);
CREATE INDEX idx_raw_lookup ON raw_definitions(raw_type_id, identifier, module_id);
CREATE INDEX idx_dep_lookup ON module_dependencies(target_identifier);
CREATE INDEX idx_flags_token_search ON common_raw_flags(token_name, raw_id);
CREATE INDEX idx_numeric_flags_search ON common_raw_flags_with_numeric_value(token_name, value, raw_id);
CREATE INDEX idx_module_location ON modules(module_location_id);

COMMIT;
";

pub const DOWN: &str = r"
PRAGMA foreign_keys = OFF;
DROP TABLE IF EXISTS common_raw_flags_with_numeric_value;
DROP TABLE IF EXISTS common_raw_flags;
DROP TABLE IF EXISTS raw_definitions;
DROP TABLE IF EXISTS raw_types;
DROP TABLE IF EXISTS module_dependencies;
DROP TABLE IF EXISTS module_restriction_rules;
DROP TABLE IF EXISTS module_steam_key_value_pairs;
DROP TABLE IF EXISTS steam_key_value_keys;
DROP TABLE IF EXISTS module_steam_tags;
DROP TABLE IF EXISTS steam_tags;
DROP TABLE IF EXISTS steam_metadata;
DROP TABLE IF EXISTS load_order;
DROP TABLE IF EXISTS modules;
DROP TABLE IF EXISTS module_locations;
PRAGMA foreign_keys = ON;
VACUUM;
";
