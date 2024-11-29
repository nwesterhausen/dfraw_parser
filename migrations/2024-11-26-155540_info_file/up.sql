-- Required directory table reference
CREATE TABLE directory (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  full_path TEXT NOT NULL
);

-- Required location table reference
CREATE TABLE location (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL
);

-- Author table
CREATE TABLE author (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name text NOT NULL
);

-- Info file table finally
CREATE TABLE info_file (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  -- specially formatted text, may remove in the future
  object_id TEXT NOT NULL,
  numeric_version INTEGER NOT NULL,
  display_version TEXT NOT NULL,
  earliest_compat_numeric_version INTEGER NOT NULL,
  earliest_compat_display_version TEXT NOT NULL,
  -- M:1 relationship, foreign key
  author_id INTEGER NOT NULL REFERENCES author(id),
  name TEXT NOT NULL,
  description TEXT NOT NULL
);

-- Insert the location values
INSERT INTO
  location (name)
VALUES
  ("mods"),
  ("installed_mods"),
  ("vanilla"),
  ("unknown/unset");