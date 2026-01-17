pub const UP: &str = r"
-- This unique index allows the ON CONFLICT(module_id, identifier) clause to function
CREATE UNIQUE INDEX idx_raw_definitions_unique_identifier
ON raw_definitions (module_id, identifier);
";

pub const DOWN: &str = r"
DROP INDEX idx_raw_definitions_unique_identifier;
";
