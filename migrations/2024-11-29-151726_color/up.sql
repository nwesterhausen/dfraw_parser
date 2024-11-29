-- Your SQL goes here
CREATE TABLE color (
  id integer NOT NULL PRIMARY KEY autoincrement,
  foreground integer NOT NULL,
  background integer NOT NULL,
  brightness integer NOT NULL
);