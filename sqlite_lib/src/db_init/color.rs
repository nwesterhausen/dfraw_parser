pub const COLORS_TABLE: &str = r"
CREATE TABLE colors (
    id INTEGER PRIMARY KEY,
    foreground INTEGER DEFAULT 0,
    background INTEGER DEFAULT 0,
    brightness INTEGER DEFAULT 0
);";
