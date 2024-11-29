-- Your SQLite goes here
CREATE TABLE gait_type (
  id integer NOT NULL PRIMARY KEY autoincrement,
  name text NOT NULL
);

INSERT INTO
  gait_type (name)
VALUES
  ('WALK'),
  ('CRAWL'),
  ('SWIM'),
  ('FLY'),
  ('CLIMB');

CREATE TABLE gait (
  id integer NOT NULL PRIMARY KEY autoincrement,
  type_id integer NOT NULL REFERENCES gait_type(id),
  name text NOT NULL,
  max_speed integer NOT NULL,
  energy_use integer NOT NULL
);

CREATE TABLE gait_modifier (
  id integer NOT NULL PRIMARY KEY autoincrement,
  name text NOT NULL
);

INSERT INTO
  gait_modifier (name)
VALUES
  ('LayersSlow'),
  ('Strength'),
  ('Agiility'),
  ('StealSlows'),
  ('NoBuildUp'),
  ('BuildUp');

CREATE TABLE gait_modifier_value (
  gait_modifier_id integer NOT NULL REFERENCES gait_modifier(id),
  name text NOT NULL,
  value integer NOT NULL,
  PRIMARY KEY (gait_modifier_id, name)
);