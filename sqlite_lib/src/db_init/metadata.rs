//! Metadata table DDL and documentation.
//!
//! Defines tables that store metadata about parsed raw files, objects and
//! modules.

/// `raw_files`
///
/// Stores information about individual raw input files parsed by the system.
///
/// Columns:
/// - `id` (INTEGER PRIMARY KEY)
/// - `module_id` (INTEGER NOT NULL) — FK to `raw_modules(id)`; the parent module
/// - `path` (TEXT NOT NULL) — file system path or archive path for the raw file
/// - `identifier` (TEXT NOT NULL) — raw file identifier (e.g., `CREATURE`)
///
/// Foreign keys:
/// - `module_id` -> `raw_modules(id)`
pub const RAW_FILES_TABLE: &str = r"
CREATE TABLE raw_files (
    id INTEGER PRIMARY KEY,
    module_id INTEGER NOT NULL,
    path TEXT NOT NULL,
    identifier TEXT NOT NULL,
    FOREIGN KEY (module_id) REFERENCES raw_modules(id)
);";

/// `raw_objects`
///
/// Represents parsed objects extracted from raw files. Each parsed object has a
/// stable `object_id` and an internal numeric `id` used as the primary key.
///
/// Columns / semantics:
/// - `id` (INTEGER PRIMARY KEY) — numeric identity for this parsed object
/// - `object_id` (TEXT NOT NULL UNIQUE) — stable object id from the parsed data
/// - `identifier` (TEXT NOT NULL) — token identifier (e.g., `DWARF`)
/// - `name` (TEXT) — optional human readable name
/// - `type_id` (INTEGER NOT NULL) — FK to `ref_object_types(id)` indicating the object type
/// - `file_id` (INTEGER NOT NULL) — FK to `raw_files(id)` indicating source file
///
/// Foreign keys:
/// - `type_id` -> `ref_object_types(id)`
/// - `file_id` -> `raw_files(id)`
pub const RAW_OBJECTS_TABLE: &str = r"
CREATE TABLE raw_objects (
    id INTEGER PRIMARY KEY,
    object_id TEXT NOT NULL UNIQUE,
    identifier TEXT NOT NULL,
    name TEXT,
    type_id INTEGER NOT NULL,
    file_id INTEGER NOT NULL,
    FOREIGN KEY (type_id) REFERENCES ref_object_types(id),
    FOREIGN KEY (file_id) REFERENCES raw_files(id)
);";

/// `raw_modules`
///
/// Represents modules/packages that provide raw data. Modules are uniquely
/// identified by their `object_id` and constrained so that a particular version
/// from a specific location is unique.
///
/// Columns:
/// - `id` (INTEGER PRIMARY KEY)
/// - `object_id` (TEXT NOT NULL UNIQUE) — module object id (maps to module identity)
/// - `name` (TEXT NOT NULL) — module name
/// - `version` (TEXT NOT NULL) — module version
/// - `location_id` (INTEGER NOT NULL) — FK to `ref_module_locations(id)`
///
/// Foreign keys:
/// - `location_id` -> `ref_module_locations(id)`
///
/// Constraints:
/// - `UNIQUE (name, version, location_id)` — ensures a specific version of a module
///   at a given location only exists once.
pub const RAW_MODULES_TABLE: &str = r"
CREATE TABLE raw_modules (
    id INTEGER PRIMARY KEY,
    object_id TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    location_id INTEGER NOT NULL,
    FOREIGN KEY (location_id) REFERENCES ref_module_locations(id),
    UNIQUE (name, version, location_id)
);";
