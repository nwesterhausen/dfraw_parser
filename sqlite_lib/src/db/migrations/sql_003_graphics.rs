pub const UP: &str = r"
BEGIN;

-- Stores the physical image file locations and metadata
CREATE TABLE tile_pages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    raw_id INTEGER NOT NULL,
    identifier TEXT NOT NULL,
    file_path TEXT NOT NULL,
    tile_width INTEGER NOT NULL,
    tile_height INTEGER NOT NULL,
    FOREIGN KEY(raw_id) REFERENCES raw_definitions(id) ON DELETE CASCADE
);

-- Stores individual sprite entries and links them to tile pages by identifier
CREATE TABLE sprite_graphics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    raw_id INTEGER NOT NULL, -- The GRAPHICS raw object this belongs to
    tile_page_identifier TEXT NOT NULL,
    offset_x INTEGER NOT NULL,
    offset_y INTEGER NOT NULL,
    condition_type TEXT NOT NULL, -- e.g. DEFAULT, CHILD, SHAKING, etc.
    target_identifier TEXT NOT NULL, -- The identifier of the creature/item this represents
    FOREIGN KEY(raw_id) REFERENCES raw_definitions(id) ON DELETE CASCADE
);

-- Indexes for fast lookups when building the UI
CREATE INDEX idx_sprite_target ON sprite_graphics(target_identifier);
CREATE INDEX idx_tile_page_ident ON tile_pages(identifier);

COMMIT;
";

pub const DOWN: &str = r"
BEGIN;
DROP INDEX IF EXISTS idx_tile_page_ident;
DROP INDEX IF EXISTS idx_sprite_target;
DROP TABLE IF EXISTS sprite_graphics;
DROP TABLE IF EXISTS tile_pages;
COMMIT;
";
