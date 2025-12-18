//! Tile table DDL and documentation.

/// `tiles`
///
///  Columns and semantics:
/// - `id` (INTEGER PRIMARY KEY) — numeric identifier for the tile record.
/// - `character` (TEXT) — primary character/token for the tile (e.g., `[CASTE_TILE]`).
/// - `alt_character` (TEXT) — alternate character/token (e.g., `[CASTE_ALTTILE]`).
/// - `color` (TEXT) — color token or descriptor (e.g., `[CASTE_COLOR]`).
/// - `glow_character` (TEXT) — glyph used when the tile is glowing (e.g., `[CASTE_GLOWTILE]`).
/// - `glow_color` (TEXT) — color applied when the tile is glowing (e.g., `[CASTE_GLOWCOLOR]`).
pub const TILES_TABLE: &str = r"
 CREATE TABLE tiles (
     id INTEGER PRIMARY KEY,
     character TEXT,
     alt_character TEXT,
     color TEXT,
     glow_character TEXT,
     glow_color TEXT
 );";
