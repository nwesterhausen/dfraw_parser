pub const RAW_FILES_TABLE: &str = r"
CREATE TABLE raw_files (
    id INTEGER PRIMARY KEY,
    module_id INTEGER NOT NULL,    -- Parent Module
    path TEXT NOT NULL,            -- Maps to Metadata.raw_file_path
    identifier TEXT NOT NULL,      -- Maps to Metadata.raw_identifier (e.g., 'CREATURE')

    FOREIGN KEY (module_id) REFERENCES raw_modules(id)
);";

pub const RAW_OBJECTS_TABLE: &str = r"
CREATE TABLE raw_objects (
    id INTEGER PRIMARY KEY,

    -- Identity
    object_id TEXT NOT NULL UNIQUE, -- Maps to parsed struct's `object_id`
    identifier TEXT NOT NULL,       -- Maps to `identifier` (e.g., 'DWARF')
    name TEXT,                      -- Maps to generic name accessor

    -- Metadata Links
    type_id INTEGER NOT NULL,                   -- Maps to Metadata.object_type
    file_id INTEGER NOT NULL,                   -- Links to the specific file source

    FOREIGN KEY (type_id) REFERENCES ref_object_types(id),
    FOREIGN KEY (file_id) REFERENCES raw_files(id)
);";

pub const RAW_MODULES_TABLE: &str = r"
CREATE TABLE raw_modules (
    id INTEGER PRIMARY KEY,
    object_id TEXT NOT NULL UNIQUE,  -- Maps to Metadata.module_object_id
    name TEXT NOT NULL,              -- Maps to Metadata.module_name
    version TEXT NOT NULL,           -- Maps to Metadata.module_version
    location_id INTEGER NOT NULL,    -- Maps to Metadata.raw_module_location

    FOREIGN KEY (location_id) REFERENCES ref_module_locations(id),
    -- Ensure a specific version of a module is unique
    UNIQUE (name, version, location_id)
);";
