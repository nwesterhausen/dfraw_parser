pub const TILES_TABLE: &str = r"
CREATE TABLE tiles (
    id INTEGER PRIMARY KEY,
    character TEXT,               -- [CASTE_TILE]
    alt_character TEXT,           -- [CASTE_ALTTILE]
    color TEXT,                   -- [CASTE_COLOR]
    glow_character TEXT,          -- [CASTE_GLOWTILE]
    glow_color TEXT               -- [CASTE_GLOWCOLOR]
);";
