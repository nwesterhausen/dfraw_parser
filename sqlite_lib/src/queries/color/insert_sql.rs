//! SQL Querys to INSERT from the `colors` table

/// Insert SQL for the `colors` table.
///
/// Expects parameters:
///
/// 1. `foreground` - foreground color identifier/value (bound to ?1)
/// 2. `background` - background color identifier/value (bound to ?2)
/// 3. `brightness` - brightness value (bound to ?3)
pub const INSERT_COLOR: &str = r"
INSERT INTO colors (foreground, background, brightness)
VALUES (?1, ?2, ?3);";
