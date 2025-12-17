pub const CASTES_TABLE: &str = r"
CREATE TABLE castes (
    id INTEGER PRIMARY KEY,
    creature_id INTEGER NOT NULL,
    identifier TEXT NOT NULL,       -- e.g., 'MALE', 'FEMALE', 'DEFAULT'

    -- Text Tokens
    description TEXT,
    baby_name_singular TEXT,
    baby_name_plural TEXT,
    child_name_singular TEXT,
    child_name_plural TEXT,
    caste_name_singular TEXT,
    caste_name_plural TEXT,

    -- Integer Values (Nullable)
    difficulty INTEGER,
    egg_size INTEGER,
    grass_trample INTEGER,
    grazer INTEGER,
    low_light_vision INTEGER,
    pet_value INTEGER,
    pop_ratio INTEGER,
    change_body_size_percent INTEGER,

    -- Flattened Range Arrays ([min, max])
    clutch_size_min INTEGER,
    clutch_size_max INTEGER,
    litter_size_min INTEGER,
    litter_size_max INTEGER,
    max_age_min INTEGER,
    max_age_max INTEGER,

    -- Age Milestones
    baby_age INTEGER,
    child_age INTEGER,

    -- Milkable (1:1 relationship flattened here for simplicity)
    milkable_material TEXT,
    milkable_frequency INTEGER,

    FOREIGN KEY (creature_id) REFERENCES creatures(id)
);";

pub const CASTE_FLAGS_TABLE: &str = r"
CREATE TABLE caste_flags (
    id INTEGER PRIMARY KEY,
    caste_id INTEGER NOT NULL,
    flag_id INTEGER NOT NULL,

    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (flag_id) REFERENCES ref_caste_token_flags(id)
);
CREATE INDEX idx_caste_flags ON caste_flags (caste_id, flag_id)";

pub const CASTE_VALUE_FLAGS_TABLE: &str = r"
CREATE TABLE caste_value_flags (
id INTEGER PRIMARY KEY,
    caste_id INTEGER NOT NULL,
    flag_id INTEGER NOT NULL,

    -- Values are either a string or a number
    value_string TEXT,
    value_int INTEGER,

    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (flag_id) REFERENCES ref_caste_token_flags(id)
);
CREATE INDEX idx_caste_value_flags ON caste_flags (caste_id, flag_id)";

pub const CASTE_ATTACKS_TABLE: &str = r"
CREATE TABLE caste_attacks (
    id INTEGER PRIMARY KEY,
    caste_id INTEGER NOT NULL,
    name TEXT NOT NULL,          -- e.g., 'BITE'
    body_part TEXT,              -- e.g., 'BY_TOKEN:MOUTH'

    FOREIGN KEY (caste_id) REFERENCES castes(id)
);";

pub const CASTE_ATTACK_TRIGGERS_TABLE: &str = r"
CREATE TABLE caste_attack_triggers (
    id INTEGER PRIMARY KEY,
    caste_id INTEGER NOT NULL,
    population INTEGER DEFAULT 0,
    exported_wealth INTEGER DEFAULT 0,
    created_wealth INTEGER DEFAULT 0,

    FOREIGN KEY (caste_id) REFERENCES castes(id)
);";

pub const CASTE_BODY_APPEARANCE_MODIFIERS_TABLE: &str = r"
CREATE TABLE caste_body_appearance_modifiers (
    id INTEGER PRIMARY KEY,
    caste_id INTEGER NOT NULL,
    attribute TEXT,
    value1 INTEGER DEFAULT 0,
    value2 INTEGER DEFAULT 0,
    value3 INTEGER DEFAULT 0,
    value4 INTEGER DEFAULT 0,
    value5 INTEGER DEFAULT 0,
    value6 INTEGER DEFAULT 0,
    value7 INTEGER DEFAULT 0,

    FOREIGN KEY (caste_id) REFERENCES castes(id)
);";

pub const CASTE_BODY_DETAIL_PLANS_TABLE: &str = r"
CREATE TABLE caste_body_detail_plans (
    id INTEGER PRIMARY KEY,
    caste_id INTEGER NOT NULL,
    body_plan TEXT,

    -- Arguments straight from the raw for now
    arguments TEXT,

    FOREIGN KEY (caste_id) REFERENCES castes(id)
);";

pub const CASTE_BODY_SIZES_TABLE: &str = r"
CREATE TABLE caste_body_sizes (
    id INTEGER PRIMARY KEY,
    caste_id INTEGER NOT NULL,
    years INTEGER NOT NULL,
    days INTEGER NOT NULL,
    size_cm3 INTEGER NOT NULL, -- The 'size' value

    FOREIGN KEY (caste_id) REFERENCES castes(id)
);";

pub const CASTE_GAITS: &str = r"
CREATE TABLE caste_gaits (
    id INTEGER PRIMARY KEY,
    caste_id INTEGER NOT NULL,
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

    PRIMARY KEY (caste_id, tile_id),
    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (tile_id) REFERENCES tiles(id)
);";
