-- Your SQL goes here
CREATE TABLE steam_data (
  id integer NOT NULL PRIMARY KEY autoincrement,
  -- file_id is a u64 which is too big for sqlite. but we aren't doing math with it
  file_id text NOT NULL,
  title text,
  description text,
  changelog text
);

CREATE TABLE steam_tag (
  id integer NOT NULL PRIMARY KEY autoincrement,
  name text NOT NULL
);

CREATE TABLE steam_data_tag (
  steam_data_id integer NOT NULL REFERENCES steam_data(id),
  steam_tag_id integer NOT NULL REFERENCES steam_tag(id),
  PRIMARY KEY (steam_data_id, steam_tag_id)
);

CREATE TABLE steam_data_key_value_tag (
  id integer NOT NULL PRIMARY KEY autoincrement,
  steam_data_id integer NOT NULL REFERENCES steam_data(id),
  steam_tag_id integer NOT NULL REFERENCES steam_tag(id),
  value text NOT NULL
);

CREATE TABLE steam_data_metadata (
  id integer NOT NULL PRIMARY KEY autoincrement,
  steam_data_id integer NOT NULL REFERENCES steam_data(id),
  metadata text NOT NULL
);