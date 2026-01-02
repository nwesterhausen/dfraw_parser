//! Caste table DDL and documentation.
//!
//! This module defines the SQL DDL for all caste-related tables used by the
//! sqlite backend. Each `*_TABLE` constant contains the `CREATE TABLE` (and
//! optional `CREATE INDEX`) statements used when initializing the database.
//!
//! The doc comment for each table documents:
//! - the purpose of the table,
//! - example values where useful,
//! - foreign keys (FK) referencing other tables, and
//! - indices (primary key and other indices, including composite indices).
//!
//! Any explanatory comments that were previously embedded in the SQL strings
//! have been moved into these doc comments and removed from the SQL itself.

/// `castes`
///
/// Stores castes for a creature (e.g., `MALE`, `FEMALE`, `DEFAULT`).
///
/// Foreign keys:
/// - `creature_id` -> `creatures(id)`
///
/// Primary key:
/// - `id` (INTEGER PRIMARY KEY)
pub const CASTES_TABLE: &str = r"
CREATE TABLE castes (
    id INTEGER PRIMARY KEY,
    creature_id INTEGER NOT NULL,
    identifier TEXT NOT NULL,
    FOREIGN KEY (creature_id) REFERENCES creatures(id)
);";

/// `caste_tags`
///
/// Simple tag entries for castes.
///
/// Foreign keys:
/// - `caste_id` -> `castes(id)`
/// - `tag_id` -> `ref_caste_token_tags(id)`
///
/// Primary key:
/// - `id` (INTEGER PRIMARY KEY)
///
/// Indices:
/// - `idx_caste_tags` on `(caste_id, tag_id)` — composite index to speed tag lookups
pub const CASTE_TAGS_TABLE: &str = r"
CREATE TABLE caste_tags (
    id INTEGER PRIMARY KEY,
    caste_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,

    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (tag_id) REFERENCES ref_caste_token_tags(id)
);
CREATE INDEX IF NOT EXISTS idx_caste_tags ON caste_tags (caste_id, tag_id);";

/// `caste_value_tags`
///
/// Stores tag values that may be represented as a boolean, strings and/or
/// integers. This table supports up to 1 boolean (`value_bit`), 7 strings
/// (`value_string1..7`) and 7 integers (`value_int1..7`) per tag row.
///
/// Foreign keys:
/// - `caste_id` -> `castes(id)`
/// - `tag_id` -> `ref_caste_token_tags(id)`
///
/// Primary key:
/// - `id` (INTEGER PRIMARY KEY)
///
/// Indices:
/// - `idx_caste_value_tags` on `(caste_id, tag_id)` — composite index for value lookups
pub const CASTE_VALUE_TAGS_TABLE: &str = r"
CREATE TABLE caste_value_tags (
    id INTEGER PRIMARY KEY,
    caste_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,
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
    FOREIGN KEY (tag_id) REFERENCES ref_caste_token_tags(id)
);
CREATE INDEX IF NOT EXISTS idx_caste_value_tags ON caste_value_tags (caste_id, tag_id);";

/// `caste_attacks`
///
/// Attack definitions associated with a caste (e.g., `BITE` on `BY_TOKEN:MOUTH`).
///
/// Foreign keys:
/// - `caste_id` -> `castes(id)`
/// - `tag_id` -> `ref_caste_token_tags(id)`
///
/// Primary key:
/// - `id` (INTEGER PRIMARY KEY)
///
/// Indices:
/// - `idx_caste_attacks` on `(caste_id, tag_position)` — composite index for attack ordering
pub const CASTE_ATTACKS_TABLE: &str = r"
CREATE TABLE caste_attacks (
    id INTEGER PRIMARY KEY,
    caste_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,
    name TEXT NOT NULL,
    body_part TEXT,
    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (tag_id) REFERENCES ref_caste_token_tags(id)
);
CREATE INDEX IF NOT EXISTS idx_caste_attacks ON caste_attacks (caste_id, tag_position);";

/// `caste_attack_triggers`
///
/// Attack-trigger related numeric properties (population, wealth, etc.).
///
/// Foreign keys:
/// - `caste_id` -> `castes(id)`
/// - `tag_id` -> `ref_caste_token_tags(id)`
///
/// Primary key:
/// - `id` (INTEGER PRIMARY KEY)
///
/// Indices:
/// - `idx_caste_attack_triggers` on `(caste_id, tag_position)`
pub const CASTE_ATTACK_TRIGGERS_TABLE: &str = r"
CREATE TABLE caste_attack_triggers (
    id INTEGER PRIMARY KEY,
    caste_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,
    population INTEGER DEFAULT 0,
    exported_wealth INTEGER DEFAULT 0,
    created_wealth INTEGER DEFAULT 0,
    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (tag_id) REFERENCES ref_caste_token_tags(id)
);
CREATE INDEX IF NOT EXISTS idx_caste_attack_triggers ON caste_attack_triggers (caste_id, tag_position);";

/// `caste_body_detail_plans`
///
/// Named body-detail plans associated with a caste.
///
/// Example: a plan name describing a body-detail layout.
///
/// Foreign keys:
/// - `caste_id` -> `castes(id)`
/// - `tag_id` -> `ref_caste_token_tags(id)`
///
/// Primary key:
/// - `id` (INTEGER PRIMARY KEY)
///
/// Indices:
/// - `idx_caste_body_detail_plans` on `(caste_id, tag_position)`
pub const CASTE_BODY_DETAIL_PLANS_TABLE: &str = r"
CREATE TABLE caste_body_detail_plans (
    id INTEGER PRIMARY KEY,
    caste_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,
    name TEXT NOT NULL,
    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (tag_id) REFERENCES ref_caste_token_tags(id)
);
CREATE INDEX IF NOT EXISTS idx_caste_body_detail_plans ON caste_body_detail_plans (caste_id, tag_position);";

/// `caste_body_detail_plan_args`
///
/// Arguments for a `caste_body_detail_plans` entry. Arguments are ordered by
/// `argument_index`.
///
/// Foreign keys:
/// - `body_detail_plan_id` -> `caste_body_detail_plans(id)`
/// - `caste_id` -> `castes(id)`
/// - `tag_id` -> `ref_caste_token_tags(id)`
///
/// Primary key:
/// - composite `(body_detail_plan_id, argument_index)`
///
/// Notes:
/// - This table references its parent plan by `body_detail_plan_id`.
pub const CASTE_BODY_DETAIL_PLAN_ARGS_TABLE: &str = r"
CREATE TABLE caste_body_detail_plan_args (
    caste_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,
    body_detail_plan_id INTEGER NOT NULL,
    argument_index INTEGER,
    argument TEXT,
    PRIMARY KEY (body_detail_plan_id, argument_index),
    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (tag_id) REFERENCES ref_caste_token_tags(id),
    FOREIGN KEY (body_detail_plan_id) REFERENCES caste_body_detail_plans(id)
);";

/// `caste_color_tags`
///
/// Color tag entries linking a caste tag to a color.
///
/// Foreign keys:
/// - `caste_id` -> `castes(id)`
/// - `tag_id` -> `ref_caste_token_tags(id)`
/// - `color_id` -> `colors(id)`
///
/// Primary key:
/// - composite `(caste_id, tag_id, tag_position)`
///
/// Notes:
/// - Composite primary key ensures uniqueness per caste/tag/position.
pub const CASTE_COLOR_TAGS_TABLE: &str = r"
CREATE TABLE caste_color_tags (
    color_id INTEGER NOT NULL,
    caste_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,
    PRIMARY KEY (caste_id, tag_id, tag_position),
    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (tag_id) REFERENCES ref_caste_token_tags(id),
    FOREIGN KEY (color_id) REFERENCES colors(id)
);";

/// `caste_item_tags`
///
/// Item tag entries linking a caste tag to a dynamic item-of-material.
///
/// Foreign keys:
/// - `caste_id` -> `castes(id)`
/// - `tag_id` -> `ref_caste_token_tags(id)`
/// - `dyn_item_id` -> `dyn_items_of_material(id)`
///
/// Primary key:
/// - composite `(caste_id, tag_id, tag_position)`
pub const CASTE_ITEM_TAGS_TABLE: &str = r"
CREATE TABLE caste_item_tags (
    dyn_item_id INTEGER NOT NULL,
    caste_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,
    PRIMARY KEY (caste_id, tag_id, tag_position),
    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (tag_id) REFERENCES ref_caste_token_tags(id),
    FOREIGN KEY (dyn_item_id) REFERENCES dyn_items_of_material(id)
);";

/// `caste_material_tags`
///
/// Material tag entries linking a caste tag to a dynamic material-in-state.
///
/// Foreign keys:
/// - `caste_id` -> `castes(id)`
/// - `tag_id` -> `ref_caste_token_tags(id)`
/// - `dyn_material_id` -> `dyn_materials_in_state(id)`
///
/// Primary key:
/// - composite `(caste_id, tag_id, tag_position)`
pub const CASTE_MATERIAL_TAGS_TABLE: &str = r"
CREATE TABLE caste_material_tags (
    dyn_material_id INTEGER NOT NULL,
    caste_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,
    PRIMARY KEY (caste_id, tag_id, tag_position),
    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (tag_id) REFERENCES ref_caste_token_tags(id),
    FOREIGN KEY (dyn_material_id) REFERENCES dyn_materials_in_state(id)
);";

/// `caste_creature_caste_tags`
///
/// Links a caste tag to another creature/caste definition (dynamic reference).
///
/// Foreign keys:
/// - `caste_id` -> `castes(id)`
/// - `tag_id` -> `ref_caste_token_tags(id)`
/// - `creature_caste_tag_id` -> `dyn_creature_caste_tags(id)`
///
/// Primary key:
/// - composite `(caste_id, tag_position)`
pub const CASTE_CREATURE_CASTE_TAGS_TABLE: &str = r"
CREATE TABLE caste_creature_caste_tags (
    creature_caste_tag_id INTEGER NOT NULL,
    caste_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,
    PRIMARY KEY (caste_id, tag_position),
    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (tag_id) REFERENCES ref_caste_token_tags(id),
    FOREIGN KEY (creature_caste_tag_id) REFERENCES dyn_creature_caste_tags(id)
);";

/// `caste_gaits`
///
/// Defines gait types and movement parameters for a caste (e.g., `WALK`, `FLY`).
///
/// Foreign keys:
/// - `caste_id` -> `castes(id)`
/// - `tag_id` -> `ref_caste_token_tags(id)`
///
/// Primary key:
/// - `id` `(INTEGER PRIMARY KEY)`
pub const CASTE_GAITS: &str = r"
CREATE TABLE caste_gaits (
    id INTEGER PRIMARY KEY,
    caste_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,
    gait_type TEXT NOT NULL,
    max_speed INTEGER,
    build_up_time INTEGER,
    turning_max INTEGER,
    start_speed INTEGER,
    energy_usage INTEGER,
    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (tag_id) REFERENCES ref_caste_token_tags(id)
);";

/// `caste_tiles`
///
/// Tile entries associating a caste tag with a tile reference.
///
/// Foreign keys:
/// - `caste_id` -> `castes(id)`
/// - `tag_id` -> `ref_caste_token_tags(id)`
/// - `tile_id` -> `tiles(id)`
///
/// Primary key:
/// - composite `(caste_id, tag_id, tag_position)`
pub const CASTE_TILES_TABLE: &str = r"
CREATE TABLE caste_tiles (
    tile_id INTEGER NOT NULL,
    caste_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,
    PRIMARY KEY (caste_id, tag_id, tag_position),
    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (tag_id) REFERENCES ref_caste_token_tags(id),
    FOREIGN KEY (tile_id) REFERENCES tiles(id)
);";

/// `caste_lairs`
///
/// Lair token references that include a probability value.
///
/// Foreign keys:
/// - `caste_id` -> `castes(id)`
/// - `tag_id` -> `ref_caste_token_tags(id)`
/// - `lair_id` -> `ref_lair_token_tags(id)`
///
/// Primary key:
/// - composite `(caste_id, tag_id, tag_position)`
pub const CASTE_LAIRS_TABLE: &str = r"
CREATE TABLE caste_lairs (
    lair_id INTEGER NOT NULL,
    caste_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,
    probability INTEGER NOT NULL,
    PRIMARY KEY (caste_id, tag_id, tag_position),
    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (tag_id) REFERENCES ref_caste_token_tags(id),
    FOREIGN KEY (lair_id) REFERENCES ref_lair_token_tags(id)
);";

/// `caste_names`
///
/// Name entries for a caste referencing dynamic names.
///
/// Foreign keys:
/// - `name_id` -> `dyn_names(id)`
/// - `caste_id` -> `castes(id)`
/// - `tag_id` -> `ref_caste_token_tags(id)`
///
/// Primary key:
/// - `id` (INTEGER PRIMARY KEY)
///
/// Indices:
/// - `idx_caste_names` on `(caste_id, tag_id, tag_position)`
pub const CASTE_NAMES: &str = r"
CREATE TABLE caste_names (
    id INTEGER PRIMARY KEY,
    name_id INTEGER NOT NULL,
    caste_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,
    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (tag_id) REFERENCES ref_caste_token_tags(id),
    FOREIGN KEY (name_id) REFERENCES dyn_names(id)
);
CREATE INDEX IF NOT EXISTS idx_caste_names ON caste_names (caste_id, tag_id, tag_position);";

/// `caste_profession_names`
///
/// Profession-specific names referencing a caste name entry.
///
/// Foreign keys:
/// - `caste_id` -> `castes(id)`
/// - `tag_id` -> `ref_caste_token_tags(id)`
///
/// Primary key:
/// - composite `(caste_id, tag_id, tag_position)`
pub const CASTE_PROFESSION_NAMES: &str = r"
CREATE TABLE caste_profession_names (
    profession_identifier TEXT NOT NULL,
    caste_name_id INTEGER NOT NULL,
    caste_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,
    PRIMARY KEY (caste_id, tag_id, tag_position),
    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (tag_id) REFERENCES ref_caste_token_tags(id),
    FOREIGN KEY (caste_name_id) REFERENCES caste_names(id)
);";

/// `caste_secretions`
///
/// Secretion definitions for a caste, linking to a dynamic material and a
/// dynamic body-part-group. `tissue_layer` is a textual layer identifier and
/// `secretion_trigger_id` references a trigger token table.
///
/// Foreign keys:
/// - `caste_id` -> `castes(id)`
/// - `tag_id` -> `ref_caste_token_tags(id)`
/// - `dyn_material_id` -> `dyn_materials_in_state(id)`
/// - `dyn_body_part_group_id` -> `dyn_body_part_groups(id)`
/// - `secretion_trigger_id` -> `ref_secretion_triggers(id)`
///
/// Primary key:
/// - composite `(caste_id, tag_id, tag_position)`
pub const CASTE_SECRETIONS_TABLE: &str = r"
CREATE TABLE caste_secretions (
    caste_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,
    dyn_material_id INTEGER NOT NULL,
    dyn_body_part_group_id INTEGER NOT NULL,
    tissue_layer TEXT NOT NULL,
    secretion_trigger_id INTEGER NOT NULL,
    PRIMARY KEY (caste_id, tag_id, tag_position),
    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (tag_id) REFERENCES ref_caste_token_tags(id),
    FOREIGN KEY (dyn_material_id) REFERENCES dyn_materials_in_state(id),
    FOREIGN KEY (dyn_body_part_group_id) REFERENCES dyn_body_part_groups(id),
    FOREIGN KEY (secretion_trigger_id) REFERENCES ref_secretion_triggers(id)
);";

/// `caste_specific_foods`
///
/// Specific food entries for a caste referencing an object type and identifier.
///
/// Foreign keys:
/// - `caste_id` -> `castes(id)`
/// - `tag_id` -> `ref_caste_token_tags(id)`
/// - `ref_object_type_id` -> `ref_object_types(id)`
///
/// Primary key:
/// - composite `(caste_id, tag_id, tag_position)`
pub const CASTE_SPECIFIC_FOODS_TABLE: &str = r"
CREATE TABLE caste_specific_foods (
    caste_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    tag_position INTEGER NOT NULL,
    ref_object_type_id INTEGER NOT NULL,
    object_identifier TEXT NOT NULL,
    PRIMARY KEY (caste_id, tag_id, tag_position),
    FOREIGN KEY (caste_id) REFERENCES castes(id),
    FOREIGN KEY (tag_id) REFERENCES ref_caste_token_tags(id),
    FOREIGN KEY (ref_object_type_id) REFERENCES ref_object_types(id)
);";
