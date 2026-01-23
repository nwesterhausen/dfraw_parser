CREATE TABLE module_locations (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);
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
CREATE TABLE sqlite_sequence(name,seq);
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
    FOREIGN KEY(raw_id) REFERENCES raw_definitions(id) ON DELETE CASCADE
);
CREATE TABLE common_raw_flags_with_numeric_value (
    raw_id INTEGER NOT NULL,
    token_name TEXT NOT NULL,
    value INTEGER NOT NULL,
    FOREIGN KEY(raw_id) REFERENCES raw_definitions(id) ON DELETE CASCADE
);
CREATE INDEX idx_module_versions ON modules(identifier, version);
CREATE INDEX idx_raw_lookup ON raw_definitions(raw_type_id, identifier, module_id);
CREATE INDEX idx_flags_token_search ON common_raw_flags(token_name, raw_id);
CREATE INDEX idx_numeric_flags_search ON common_raw_flags_with_numeric_value(token_name, value, raw_id);
CREATE INDEX idx_module_location ON modules(module_location_id);
CREATE TABLE raw_names (
    raw_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    FOREIGN KEY(raw_id) REFERENCES raw_definitions(id) ON DELETE CASCADE
);
CREATE INDEX idx_raw_names_lookup ON raw_names(name);
CREATE VIRTUAL TABLE raw_search_index USING fts5(
    raw_id UNINDEXED,
    names,
    description,
    tokenize='trigram'
)
/* raw_search_index(raw_id,names,description) */;
CREATE TABLE IF NOT EXISTS 'raw_search_index_data'(id INTEGER PRIMARY KEY, block BLOB);
CREATE TABLE IF NOT EXISTS 'raw_search_index_idx'(segid, term, pgno, PRIMARY KEY(segid, term)) WITHOUT ROWID;
CREATE TABLE IF NOT EXISTS 'raw_search_index_content'(id INTEGER PRIMARY KEY, c0, c1, c2);
CREATE TABLE IF NOT EXISTS 'raw_search_index_docsize'(id INTEGER PRIMARY KEY, sz BLOB);
CREATE TABLE IF NOT EXISTS 'raw_search_index_config'(k PRIMARY KEY, v) WITHOUT ROWID;
CREATE TABLE tile_pages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    raw_id INTEGER NOT NULL,
    identifier TEXT NOT NULL,
    file_path TEXT NOT NULL,
    tile_width INTEGER NOT NULL,
    tile_height INTEGER NOT NULL,
    page_width INTEGER NOT NULL,
    page_height INTEGER NOT NULL,
    FOREIGN KEY(raw_id) REFERENCES raw_definitions(id) ON DELETE CASCADE
);
CREATE TABLE sprite_graphics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    raw_id INTEGER NOT NULL, -- The GRAPHICS raw object this belongs to
    tile_page_identifier TEXT NOT NULL,
    offset_x INTEGER NOT NULL,
    offset_y INTEGER NOT NULL,
    offset_x_2 INTEGER,
    offset_y_2 INTEGER,
    primary_condition TEXT NOT NULL, -- e.g. DEFAULT, CHILD, SHAKING, etc.
    secondary_condition TEXT,        -- (optional) e.g. DEFAULT, CHILD, SHAKING, etc.
    target_identifier TEXT NOT NULL, -- The identifier of the creature/item this represents
    FOREIGN KEY(raw_id) REFERENCES raw_definitions(id) ON DELETE CASCADE
);
CREATE INDEX idx_sprite_target ON sprite_graphics(target_identifier);
CREATE INDEX idx_tile_page_ident ON tile_pages(identifier);
CREATE TABLE app_metadata (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
CREATE UNIQUE INDEX idx_raw_definitions_unique_identifier
ON raw_definitions (module_id, identifier);
