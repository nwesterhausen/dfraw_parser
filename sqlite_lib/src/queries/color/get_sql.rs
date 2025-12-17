//! SQL Querys to GET from the `colors` table

pub const GET_COLOR_ID_BY_VALUES: &str = r"
SELECT id FROM colors
WHERE foreground = ?1
 AND  background = ?2
 AND  brightness = ?3;";
