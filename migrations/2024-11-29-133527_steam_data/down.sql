-- This file should undo anything in `up.sql`
DROP TABLE steam_data;

ALTER TABLE
  info_file DROP COLUMN steam_data_id;

DROP TABLE steam_tag;

DROP TABLE steam_data_tag;

DROP TABLE steam_data_key_value_tag;

DROP TABLE steam_data_metadata;