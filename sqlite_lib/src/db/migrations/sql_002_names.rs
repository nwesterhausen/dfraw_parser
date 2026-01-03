pub const UP: &str = r"
BEGIN;

-- Table for all names associated with a raw (Castes, Growths, etc.)
-- Use this for exact lookups or populating UI lists.
CREATE TABLE raw_names (
    raw_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    FOREIGN KEY(raw_id) REFERENCES raw_definitions(id) ON DELETE CASCADE
);
CREATE INDEX idx_raw_names_lookup ON raw_names(name);

-- FTS5 Virtual Table for high-performance name and description searching.
-- Populating 'names' with your space-separated string is ideal here.
-- 'trigram' is used to allow substring matching (e.g., 'oad' matches 'toad').
CREATE VIRTUAL TABLE raw_search_index USING fts5(
    raw_id UNINDEXED,
    names,
    description,
    tokenize='trigram'
);

COMMIT;
";
pub const DOWN: &str = r"
BEGIN;
DROP TABLE raw_names;
DROP TABLE raw_search_index;
COMMIT;
";
