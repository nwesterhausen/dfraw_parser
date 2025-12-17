pub const CASTES_TABLE: &str = r"
CREATE TABLE castes (
    id INTEGER PRIMARY KEY,
    creature_id INTEGER NOT NULL,
    identifier TEXT NOT NULL,       -- e.g., 'MALE', 'FEMALE', 'DEFAULT'

    FOREIGN KEY (creature_id) REFERENCES creatures(id)
);";

pub const CASTE_FLAGS_TABLE: &str = r"
CREATE TABLE caste_flags (
    id INTEGER PRIMARY KEY,
    caste_id INTEGER NOT NULL,
    flag_id INTEGER NOT NULL,
    position INTEGER NOT NULL,

    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (flag_id) REFERENCES ref_caste_token_flags(id)
);
CREATE INDEX idx_caste_flags ON caste_flags (caste_id, flag_id)";

pub const CASTE_VALUE_FLAGS_TABLE: &str = r"
CREATE TABLE caste_value_flags (
id INTEGER PRIMARY KEY,
    caste_id INTEGER NOT NULL,
    flag_id INTEGER NOT NULL,
    position INTEGER NOT NULL,

    -- Values may be:
    --      a boolean
    --      a string
    --      a number
    --      any combination of up to 7 string, 7 numbers and 1 boolean
    value_bit INTEGER,
    value_string1 TEXT,
    value_string2 TEXT,
    value_string3 TEXT,
    value_string4 TEXT,
    value_string5 TEXT,
    value_string6 TEXT,
    value_string7 TEXT,
    value_int1 INTEGER,
    value_int2 INTEGER,
    value_int3 INTEGER,
    value_int4 INTEGER,
    value_int5 INTEGER,
    value_int6 INTEGER,
    value_int7 INTEGER,

    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (flag_id) REFERENCES ref_caste_token_flags(id)
);
CREATE INDEX idx_caste_value_flags ON caste_flags (caste_id, flag_id);";

pub const CASTE_ATTACKS_TABLE: &str = r"
CREATE TABLE caste_attacks (
    id INTEGER PRIMARY KEY,
    caste_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,
    name TEXT NOT NULL,          -- e.g., 'BITE'
    body_part TEXT,              -- e.g., 'BY_TOKEN:MOUTH'

    FOREIGN KEY (caste_id) REFERENCES castes(id)
);
CREATE INDEX idx_caste_attacks ON caste_attacks (caste_id, tag_position);";

pub const CASTE_ATTACK_TRIGGERS_TABLE: &str = r"
CREATE TABLE caste_attack_triggers (
    id INTEGER PRIMARY KEY,
    caste_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,
    population INTEGER DEFAULT 0,
    exported_wealth INTEGER DEFAULT 0,
    created_wealth INTEGER DEFAULT 0,

    FOREIGN KEY (caste_id) REFERENCES castes(id)
);
CREATE INDEX idx_caste_attack_triggers ON caste_attack_triggers (caste_id, tag_position);";

pub const CASTE_BODY_DETAIL_PLANS_TABLE: &str = r"
CREATE TABLE caste_body_detail_plans (
    id INTEGER PRIMARY KEY,
    caste_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,
    name TEXT NOT NULL,
    FOREIGN KEY (caste_id) REFERENCES castes(id)
);
CREATE INDEX idx_caste_body_detail_plans ON caste_body_detail_plans (caste_id, tag_position);";

pub const CASTE_BODY_DETAIL_PLAN_ARGS_TABLE: &str = r"
CREATE TABLE caste_body_detail_plan_args (
    body_detail_plan_id INTEGER NOT NULL,
    argument_index INTEGER,
    argument TEXT,

    PRIMARY KEY (body_detail_plan_id, argument_index),
    FOREIGN KEY (body_detail_plan_id) REFERENCES caste_body_detail_plans(id)
);";

pub const CASTE_BODY_SIZES_TABLE: &str = r"
CREATE TABLE caste_body_sizes (
    id INTEGER PRIMARY KEY,
    caste_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,
    years INTEGER NOT NULL,
    days INTEGER NOT NULL,
    size_cm3 INTEGER NOT NULL, -- The 'size' value

    FOREIGN KEY (caste_id) REFERENCES castes(id)
);";

pub const CASTE_COLOR_TAGS_TABLE: &str = r"
CREATE TABLE caste_color_tags (
    caste_id INTEGER NOT NULL,
    color_id INTEGER NOT NULL,
    flag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,

    PRIMARY KEY (caste_id, flag_id, tag_position),
    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (color_id) REFERENCES colors(id),
    FOREIGN KEY (flag_id) REFERENCES ref_caste_token_flags(id)
);";

pub const CASTE_ITEM_TAGS_TABLE: &str = r"
CREATE TABLE caste_item_tags (
    caste_id INTEGER NOT NULL,
    dyn_item_id INTEGER NOT NULL,
    flag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,

    PRIMARY KEY (caste_id, flag_id, tag_position),
    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (flag_id) REFERENCES ref_caste_token_flags(id),
    FOREIGN KEY (dyn_item_id) REFERENCES dyn_items_of_material(id)
);";

pub const CASTE_MATERIAL_TAGS_TABLE: &str = r"
CREATE TABLE caste_material_tags (
    caste_id INTEGER NOT NULL,
    dyn_material_id INTEGER NOT NULL,
    flag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,

    PRIMARY KEY (caste_id, flag_id, tag_position),
    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (flag_id) REFERENCES ref_caste_token_flags(id),
    FOREIGN KEY (dyn_material_id) REFERENCES dyn_materials_in_state(id)
);";

pub const CASTE_CREATURE_CASTE_TAGS_TABLE: &str = r"
CREATE TABLE caste_creature_caste_tags (
    caste_id INTEGER NOT NULL,
    creature_caste_tag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,

    PRIMARY KEY (caste_id, tag_position),
    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (creature_caste_tag_id) REFERENCES dyn_creature_caste_tags(id)
);";

pub const CASTE_GAITS: &str = r"
CREATE TABLE caste_gaits (
    id INTEGER PRIMARY KEY,
    caste_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,
    gait_type TEXT NOT NULL,     -- e.g., 'WALK', 'FLY'
    max_speed INTEGER,
    build_up_time INTEGER,
    turning_max INTEGER,
    start_speed INTEGER,
    energy_usage INTEGER,

    FOREIGN KEY (caste_id) REFERENCES castes(id)
);";

pub const CASTE_TILES_TABLE: &str = r"
CREATE TABLE caste_tiles (
    caste_id INTEGER NOT NULL,
    tile_id INTEGER NOT NULL,
    flag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,

    PRIMARY KEY (caste_id, flag_id,tag_position),
    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (flag_id) REFERENCES ref_caste_token_flags(id),
    FOREIGN KEY (tile_id) REFERENCES tiles(id)
);";

pub const CASTE_LAIRS_TABLE: &str = r"
CREATE TABLE caste_lairs (
    caste_id INTEGER NOT NULL,
    lair_id INTEGER NOT NULL,
    flag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,
    probability INTEGER NOT NULL,

    PRIMARY KEY (caste_id, flag_id,tag_position),
    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (flag_id) REFERENCES ref_caste_token_flags(id),
    FOREIGN KEY (lair_id) REFERENCES ref_lair_token_flags(id)
);";

pub const CASTE_NAMES: &str = r"
CREATE TABLE caste_names (
    id INTEGER PRIMARY KEY,
    caste_id INTEGER NOT NULL,
    name_id INTEGER NOT NULL,
    flag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,

    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (flag_id) REFERENCES ref_caste_token_flags(id),
    FOREIGN KEY (name_id) REFERENCES dyn_names(id)
);
CREATE INDEX idx_caste_names ON caste_names (caste_id, flag_id, tag_position);";

pub const CASTE_PROFESSION_NAMES: &str = r"
CREATE TABLE caste_profession_names (
    profession_identifier TEXT NOT NULL,
    caste_name_id INTEGER NOT NULL,
    caste_id INTEGER NOT NULL,
    flag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,


    PRIMARY KEY (caste_id, flag_id, tag_position),
    FOREIGN KEY (caste_name_id) REFERENCES caste_names(id),
    FOREIGN KEY (flag_id) REFERENCES ref_caste_token_flags(id)
);";

pub const CASTE_SECRETIONS_TABLE: &str = r"
CREATE TABLE caste_secretions (
    caste_id INTEGER NOT NULL,
    flag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,
    dyn_material_id INTEGER NOT NULL,
    dyn_body_part_group_id INTEGER NOT NULL,
    tissue_layer TEXT NOT NULL,
    secretion_trigger_id INTEGER NOT NULL,

    PRIMARY KEY (caste_id, flag_id, tag_position),
    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (flag_id) REFERENCES ref_caste_token_flags(id),
    FOREIGN KEY (dyn_material_id) REFERENCES dyn_materials_in_state(id),
    FOREIGN KEY (dyn_body_part_group_id) REFERENCES dyn_body_part_groups(id)
);";

pub const CASTE_SPECIFIC_FOODS_TABLE: &str = r"
CREATE TABLE caste_specific_foods (
    caste_id INTEGER NOT NULL,
    flag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,
    ref_object_type_id INTEGER NOT NULL,
    object_identifier TEXT NOT NULL,

    PRIMARY KEY (caste_id, flag_id, tag_position),
    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (flag_id) REFERENCES ref_caste_token_flags(id),
    FOREIGN KEY (ref_object_type_id) REFERENCES ref_object_types(id)
);";
