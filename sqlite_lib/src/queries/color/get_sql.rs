//! SQL Querys to GET from the `colors` table

/// Get a `colors` table id by its value triplet.
///
/// Expects parameters:
///
/// 1. `foreground` - foreground color identifier/value (bound to ?1)
/// 2. `background` - background color identifier/value (bound to ?2)
/// 3. `brightness` - brightness value (bound to ?3)
pub const GET_COLOR_ID_BY_VALUES: &str = r"
SELECT id FROM colors
WHERE foreground = ?1
 AND  background = ?2
 AND  brightness = ?3;";
