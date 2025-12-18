//! Creature table DDL and documentation.

/// `creatures` table
///
/// Purpose:
/// - Store creature-specific scalar attributes parsed from raw definitions.
///
/// Primary key:
/// - `id` (INTEGER PRIMARY KEY)
///
/// Columns / mappings:
/// - `id` : shared PK and FK to `raw_objects(id)`
/// - `frequency` : INTEGER DEFAULT 50 — maps to `Creature.frequency`
/// - `population_min` / `population_max` : map to `Creature.population_number[0/1]`
/// - `cluster_min` / `cluster_max` : map to `Creature.cluster_number[0/1]`
/// - `underground_depth_min` / `underground_depth_max` : map to `Creature.underground_depth[0/1]`
///
/// Foreign keys:
/// - `id` -> `raw_objects(id)`
pub const CREATURES_TABLE: &str = r"
 CREATE TABLE creatures (
     id INTEGER PRIMARY KEY,
     frequency INTEGER DEFAULT 50,
     population_min INTEGER,
     population_max INTEGER,
     cluster_min INTEGER,
     cluster_max INTEGER,
     underground_depth_min INTEGER,
     underground_depth_max INTEGER,
     FOREIGN KEY (id) REFERENCES raw_objects(id)
 );";

/// `creature_biomes` table
///
/// Purpose:
/// - Many-to-many relationship between creatures and biome tokens.
///
/// Primary key:
/// - composite `(creature_id, biome_tag_id)`
///
/// Columns:
/// - `creature_id` (INTEGER NOT NULL) — FK to `creatures(id)`
/// - `biome_tag_id` (INTEGER NOT NULL) — FK to `ref_biomes(id)`
///
/// Foreign keys:
/// - `creature_id` -> `creatures(id)`
/// - `biome_tag_id` -> `ref_biomes(id)`
pub const CREATURE_BIOMES_TABLE: &str = r"
 CREATE TABLE creature_biomes (
     creature_id INTEGER NOT NULL,
     biome_tag_id INTEGER NOT NULL,
     PRIMARY KEY (creature_id, biome_tag_id),
     FOREIGN KEY (creature_id) REFERENCES creatures(id),
     FOREIGN KEY (biome_tag_id) REFERENCES ref_biomes(id)
 );";
