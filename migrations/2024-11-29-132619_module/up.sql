-- Your SQL goes here
CREATE TABLE module (
  id integer NOT NULL PRIMARY KEY autoincrement,
  name text NOT NULL,
  description text NOT NULL,
  loction_id integer NOT NULL REFERENCES location(id),
  directory_id integer NOT NULL REFERENCES directory(id),
  info_file_id integer REFERENCES info_file(id)
);

-- Modify the metadata table to include the module_id and drop the module_location_id column
ALTER TABLE
  metadata
ADD
  COLUMN module_id integer NOT NULL REFERENCES module(id);

ALTER TABLE
  metadata DROP COLUMN module_location_id;