pub const UP: &str = r"
BEGIN;

CREATE TABLE app_metadata (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

COMMIT;
";

pub const DOWN: &str = r"
BEGIN;
DROP TABLE IF EXISTS app_metadata;
COMMIT;
";
