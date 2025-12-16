pub const CREATURES_TABLE: &str = r"
CREATE TABLE creatures (
    -- Shared Primary Key / Foreign Key pattern
    -- The ID here matches the ID in `raw_objects`
    id INTEGER PRIMARY KEY,

    -- Creature specific scalar fields
    frequency INTEGER DEFAULT 50,               -- Maps to Creature.frequency
    population_min INTEGER,                     -- Maps to Creature.population_number[0]
    population_max INTEGER,                     -- Maps to Creature.population_number[1]
    cluster_min INTEGER,                        -- Maps to Creature.cluster_number[0]
    cluster_max INTEGER,                        -- Maps to Creature.cluster_number[1]
    underground_depth_min INTEGER,              -- Maps to Creature.underground_depth[0]
    underground_depth_max INTEGER,              -- Maps to Creature.underground_depth[1]

    -- Foreign Key to the master table
    FOREIGN KEY (id) REFERENCES raw_objects(id)
);";

pub const CREATURE_BIOMES_TABLE: &str = r"
-- You would need a ref_biomes table first (similar to ref_object_types)
CREATE TABLE creature_biomes (
    creature_id INTEGER NOT NULL,
    biome_token_id INTEGER NOT NULL,       -- FK to ref_biomes table

    PRIMARY KEY (creature_id, biome_token_id),
    FOREIGN KEY (creature_id) REFERENCES creatures(id),
    FOREIGN KEY (biome_token_id) REFERENCES ref_biomes(id)
);";
