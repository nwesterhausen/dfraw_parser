-- This file should undo anything in `up.sql`
DROP TABLE module;

ALTER TABLE
  metadata
ADD
  COLUMN module_location_id integer NOT NULL REFERENCES location(id);

ALTER TABLE
  metadata DROP COLUMN module_id;