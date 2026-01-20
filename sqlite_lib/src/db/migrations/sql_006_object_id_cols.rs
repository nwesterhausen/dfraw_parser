pub const UP: &str = r"
ALTER TABLE modules ADD COLUMN object_id BLOB;
ALTER TABLE raw_definitions ADD COLUMN object_id BLOB;

-- Create indices for the new columns to ensure fast lookups
CREATE INDEX idx_modules_object_id ON modules(object_id);
CREATE INDEX idx_raw_definitions_object_id ON raw_definitions(object_id);
";

pub const DOWN: &str = r"
DROP INDEX IF EXISTS idx_modules_object_id;
DROP INDEX IF EXISTS idx_raw_definitions_object_id;

ALTER TABLE modules DROP COLUMN object_id;
ALTER TABLE raw_definitions DROP COLUMN object_id;
";
