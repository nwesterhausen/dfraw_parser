//! Color table DDL and documentation.

/// `colors` table DDL.
///
/// Columns:
/// - `id` (INTEGER PRIMARY KEY)
/// - `foreground` (INTEGER, default 0) — integer color value for the foreground
/// - `background` (INTEGER, default 0) — integer color value for the background
/// - `brightness` (INTEGER, default 0) — brightness or intensity as an integer
pub const COLORS_TABLE: &str = r"
 CREATE TABLE colors (
     id INTEGER PRIMARY KEY,
     foreground INTEGER DEFAULT 0,
     background INTEGER DEFAULT 0,
     brightness INTEGER DEFAULT 0
 );";
