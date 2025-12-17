//! SQL Querys to INSERT from the `colors` table

pub const INSERT_COLOR: &str = r"
INSERT INTO colors (foreground, background, brightness)
VALUES (?1, ?2, ?3);";
