CREATE TABLE object_type
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    identifier  TEXT,
    description TEXT
);

CREATE TABLE biome
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    identifier  TEXT,
    description TEXT
);

CREATE TABLE author
(
    id   INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT
);

CREATE TABLE location
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    identifier    TEXT,
    path          TEXT,
    relative_path TEXT
);

CREATE TABLE steam_data
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    file_id     TEXT,
    title       TEXT,
    description TEXT,
    changelog   TEXT
);

CREATE TABLE module
(
    id                                INTEGER PRIMARY KEY AUTOINCREMENT,
    identifier                        TEXT,
    name                              TEXT,
    description                       TEXT,
    version                           TEXT,
    numerical_version                 INTEGER,
    earliest_compat_version           TEXT,
    earliest_compat_numerical_version INTEGER,
    steam_data                        INTEGER,
    FOREIGN KEY (steam_data) REFERENCES steam_data (id)
);

CREATE TABLE module_required_module
(
    module_id          INTEGER,
    required_module_id INTEGER,
    PRIMARY KEY (module_id, required_module_id),
    FOREIGN KEY (module_id) REFERENCES module (id),
    FOREIGN KEY (required_module_id) REFERENCES module (id)
);

CREATE TABLE module_conflicts_with_module
(
    module_id             INTEGER,
    conflicting_module_id INTEGER,
    PRIMARY KEY (module_id, conflicting_module_id),
    FOREIGN KEY (module_id) REFERENCES module (id),
    FOREIGN KEY (conflicting_module_id) REFERENCES module (id)
);

CREATE TABLE module_required_before_module
(
    module_id                 INTEGER,
    required_before_module_id INTEGER,
    PRIMARY KEY (module_id, required_before_module_id),
    FOREIGN KEY (module_id) REFERENCES module (id),
    FOREIGN KEY (required_before_module_id) REFERENCES module (id)
);

CREATE TABLE module_required_after_module
(
    module_id                INTEGER,
    required_after_module_id INTEGER,
    PRIMARY KEY (module_id, required_after_module_id),
    FOREIGN KEY (module_id) REFERENCES module (id),
    FOREIGN KEY (required_after_module_id) REFERENCES module (id)
);

CREATE TABLE raw_metadata
(
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    identifier      TEXT,
    module          INTEGER,
    module_location INTEGER,
    object_type     INTEGER,
    raw_file_path   TEXT,
    FOREIGN KEY (module) REFERENCES module (id),
    FOREIGN KEY (module_location) REFERENCES location (id),
    FOREIGN KEY (object_type) REFERENCES object_type (id)
);

CREATE TABLE steam_tag
(
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    name      TEXT,
    has_value INTEGER
);

CREATE TABLE steam_data_2_tag
(
    steam_data_id INTEGER,
    steam_tag_id  INTEGER,
    PRIMARY KEY (steam_data_id, steam_tag_id),
    FOREIGN KEY (steam_data_id) REFERENCES steam_data (id),
    FOREIGN KEY (steam_tag_id) REFERENCES steam_tag (id)
);

CREATE TABLE steam_data_2_value_tag
(
    steam_data_id INTEGER,
    steam_tag_id  INTEGER,
    value         TEXT,
    PRIMARY KEY (steam_data_id, steam_tag_id),
    FOREIGN KEY (steam_data_id) REFERENCES steam_data (id),
    FOREIGN KEY (steam_tag_id) REFERENCES steam_tag (id)
);

CREATE TABLE min_max_data
(
    id      INTEGER PRIMARY KEY AUTOINCREMENT,
    minimum INTEGER NOT NULL,
    maximum INTEGER NOT NULL
);

CREATE TABLE name_data
(
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    singular  TEXT,
    plural    TEXT,
    adjective TEXT
);

CREATE TABLE state_of_matter
(
    id    INTEGER PRIMARY KEY AUTOINCREMENT,
    state TEXT
);

CREATE TABLE material_data
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    identifier TEXT,
    state      INTEGER,
    FOREIGN KEY (state) REFERENCES state_of_matter (id)
);

CREATE TABLE color
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    identifier TEXT,
    foreground INTEGER,
    background INTEGER,
    brightness INTEGER
);

CREATE TABLE tile
(
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    character      TEXT,
    alt_character  TEXT,
    color          INTEGER,
    glow_character TEXT,
    glow_color     INTEGER,
    FOREIGN KEY (color) REFERENCES color (id),
    FOREIGN KEY (glow_color) REFERENCES color (id)
);

CREATE TABLE milkable
(
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    material  INTEGER,
    frequency INTEGER,
    FOREIGN KEY (material) REFERENCES material_data (id)
);
CREATE INDEX idx_milkable_material_frequency ON milkable (material, frequency);
CREATE INDEX idx_milkable_id_material_frequency ON milkable (id, material, frequency);

CREATE TABLE body_size
(
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    size_cm3 INTEGER,
    years    INTEGER,
    days     INTEGER
);

CREATE TABLE caste
(
    id                          INTEGER PRIMARY KEY AUTOINCREMENT,
    description                 TEXT,
    baby_name                   INTEGER,
    caste_name                  INTEGER,
    child_name                  INTEGER,
    clutch_size                 INTEGER,
    litter_size                 INTEGER,
    max_age                     INTEGER,
    baby                        INTEGER,
    child                       INTEGER,
    difficult                   INTEGER,
    egg_size                    INTEGER,
    grass_trample               INTEGER,
    grazer                      INTEGER,
    low_light_vision            INTEGER,
    pet_value                   INTEGER,
    pop_ratio                   INTEGER,
    change_body_size_percentage INTEGER,
    body_size                   INTEGER,
    milkable                    INTEGER,
    tile                        INTEGER,
    FOREIGN KEY (baby_name) REFERENCES name_data (id),
    FOREIGN KEY (caste_name) REFERENCES name_data (id),
    FOREIGN KEY (child_name) REFERENCES name_data (id),
    FOREIGN KEY (clutch_size) REFERENCES min_max_data (id),
    FOREIGN KEY (litter_size) REFERENCES min_max_data (id),
    FOREIGN KEY (max_age) REFERENCES min_max_data (id),
    FOREIGN KEY (body_size) REFERENCES body_size (id),
    FOREIGN KEY (milkable) REFERENCES milkable (id),
    FOREIGN KEY (tile) REFERENCES tile (id)
);

CREATE TABLE creature
(
    id                        INTEGER PRIMARY KEY AUTOINCREMENT,
    metadata                  INTEGER,
    identifier                TEXT,
    tile                      INTEGER,
    frequency                 INTEGER,
    cluster_number            INTEGER,
    population_number         INTEGER,
    underground_depth         INTEGER,
    general_baby_name         INTEGER,
    name                      INTEGER,
    copy_tags_from_identifier TEXT,
    FOREIGN KEY (metadata) REFERENCES raw_metadata (id),
    FOREIGN KEY (tile) REFERENCES tile (id),
    FOREIGN KEY (cluster_number) REFERENCES min_max_data (id),
    FOREIGN KEY (population_number) REFERENCES min_max_data (id),
    FOREIGN KEY (underground_depth) REFERENCES min_max_data (id),
    FOREIGN KEY (general_baby_name) REFERENCES name_data (id),
    FOREIGN KEY (name) REFERENCES name_data (id)
);
CREATE INDEX idx_creature_identifier ON creature (identifier);
CREATE INDEX idx_creature_id_identifier ON creature (id, identifier);

CREATE TABLE apply_creature_variation_step
(
    creature_id   INTEGER,
    raw_step_text TEXT,
    FOREIGN KEY (creature_id) REFERENCES creature (id)
);

CREATE TABLE select_creature_variation_step
(
    creature_id   INTEGER,
    raw_step_text TEXT,
    FOREIGN KEY (creature_id) REFERENCES creature (id)
);

CREATE TABLE creature_pref_strings
(
    creature_id INTEGER,
    pref_string TEXT,
    PRIMARY KEY (creature_id, pref_string),
    FOREIGN KEY (creature_id) REFERENCES creature (id)
);

CREATE TABLE creature_2_biome
(
    creature_id INTEGER,
    biome_id    INTEGER,
    PRIMARY KEY (creature_id, biome_id),
    FOREIGN KEY (creature_id) REFERENCES creature (id),
    FOREIGN KEY (biome_id) REFERENCES biome (id)
);

CREATE TABLE raw_tag
(
    id                INTEGER PRIMARY KEY AUTOINCREMENT,
    identifier        TEXT    NOT NULL,
    name              TEXT,
    description       TEXT,
    has_value         INTEGER NOT NULL DEFAULT 0,
    has_complex_value INTEGER NOT NULL DEFAULT 0,
    object_type INTEGER NOT NULL,
    FOREIGN KEY (object_type) REFERENCES object_type (id)
);

CREATE TABLE creature_2_tags
(
    creature_id INTEGER,
    raw_tag_id  INTEGER,
    PRIMARY KEY (creature_id, raw_tag_id),
    FOREIGN KEY (creature_id) REFERENCES creature (id),
    FOREIGN KEY (raw_tag_id) REFERENCES raw_tag (id)
);

CREATE TABLE creature_2_caste
(
    creature_id INTEGER,
    caste_id    INTEGER,
    PRIMARY KEY (creature_id, caste_id),
    FOREIGN KEY (creature_id) REFERENCES creature (id),
    FOREIGN KEY (caste_id) REFERENCES caste (id)
);

CREATE TABLE gait_type
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    identifier TEXT
);

CREATE TABLE gait
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    gait_type  INTEGER,
    identifier TEXT,
    max_speed  INTEGER,
    energy_use INTEGER,
    FOREIGN KEY (gait_type) REFERENCES gait_type (id)
);

CREATE TABLE caste_2_gait
(
    caste_id INTEGER,
    gait_id  INTEGER,
    PRIMARY KEY (caste_id, gait_id),
    FOREIGN KEY (caste_id) REFERENCES caste (id),
    FOREIGN KEY (gait_id) REFERENCES gait (id)
);

CREATE TABLE creature_class
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    identifier  TEXT,
    description TEXT
);

CREATE TABLE caste_2_creature_class
(
    caste_id          INTEGER,
    creature_class_id INTEGER,
    PRIMARY KEY (caste_id, creature_class_id),
    FOREIGN KEY (caste_id) REFERENCES caste (id),
    FOREIGN KEY (creature_class_id) REFERENCES creature_class (id)
);

CREATE TABLE caste_2_tags
(
    caste_id   INTEGER,
    raw_tag_id INTEGER,
    PRIMARY KEY (caste_id, raw_tag_id),
    FOREIGN KEY (caste_id) REFERENCES caste (id),
    FOREIGN KEY (raw_tag_id) REFERENCES raw_tag (id)
);

CREATE TABLE tag_2_value
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    raw_tag_id INTEGER,
    value      TEXT,
    raw_value  TEXT,
    FOREIGN KEY (raw_tag_id) REFERENCES raw_tag (id)
);
CREATE INDEX idx_tag_2_value_tag_value ON tag_2_value (raw_tag_id, value);

CREATE TABLE raw_tag_complex_value
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    raw_tag_id  INTEGER NOT NULL,
    identifier  TEXT    NOT NULL,
    name        TEXT    NOT NULL,
    description TEXT,
    FOREIGN KEY (raw_tag_id) REFERENCES raw_tag (id)
);
CREATE INDEX idx_complex_value_raw_tag_id ON raw_tag_complex_value (raw_tag_id);
CREATE INDEX idx_complex_value_identifier ON raw_tag_complex_value (identifier);

CREATE TABLE complex_tag_2_value
(
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    complex_tag_id INTEGER NOT NULL,
    value          TEXT    NOT NULL,
    FOREIGN KEY (complex_tag_id) REFERENCES raw_tag_complex_value (id)
);
CREATE INDEX idx_complex_tag_2_value_complex_tag_id ON complex_tag_2_value (complex_tag_id);
CREATE INDEX idx_complex_tag_2_value_tag_value ON complex_tag_2_value (complex_tag_id, value);

CREATE TABLE temperature_data
(
    id                         INTEGER PRIMARY KEY AUTOINCREMENT,
    specific_heat              INTEGER,
    ignition_point             INTEGER,
    melting_point              INTEGER,
    boiling_point              INTEGER,
    heat_damage_point          INTEGER,
    cold_damage_point          INTEGER,
    material_fixed_temperature INTEGER
);

CREATE TABLE color_modification
(
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    modification TEXT
);

CREATE TABLE creature_effect
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    severity    INTEGER,
    probability INTEGER,
    start       INTEGER,
    peak        INTEGER,
    end         INTEGER,
    dwf_stretch INTEGER
);

CREATE TABLE creature_effect_2_body_part_category
(
    creature_effect_id INTEGER,
    body_part_category TEXT,
    FOREIGN KEY (creature_effect_id) REFERENCES creature_effect (id)
);

CREATE TABLE creature_effect_2_body_part_type
(
    creature_effect_id INTEGER,
    body_part_type     TEXT,
    FOREIGN KEY (creature_effect_id) REFERENCES creature_effect (id)
);

CREATE TABLE creature_effect_2_body_part_token
(
    creature_effect_id INTEGER,
    body_part_token    TEXT,
    FOREIGN KEY (creature_effect_id) REFERENCES creature_effect (id)
);

CREATE TABLE creature_effect_2_tags
(
    creature_effect_id INTEGER,
    raw_tag_id         INTEGER,
    PRIMARY KEY (creature_effect_id, raw_tag_id),
    FOREIGN KEY (creature_effect_id) REFERENCES creature_effect (id),
    FOREIGN KEY (raw_tag_id) REFERENCES raw_tag (id)
);

CREATE TABLE dimension_data
(
    x INTEGER,
    y INTEGER,
    PRIMARY KEY (x, y)
);