-- Your SQL goes here
CREATE TABLE info_file_requires_ids (
  info_file_id integer NOT NULL REFERENCES info_file(id),
  requires_info_file_id integer NOT NULL REFERENCES info_file(id),
  PRIMARY KEY (info_file_id, requires_info_file_id)
);

CREATE TABLE info_file_conflicts_ids (
  info_file_id integer NOT NULL REFERENCES info_file(id),
  conflicts_info_file_id integer NOT NULL REFERENCES info_file(id),
  PRIMARY KEY (info_file_id, conflicts_info_file_id)
);

CREATE TABLE info_file_requires_before (
  info_file_id integer NOT NULL REFERENCES info_file(id),
  requires_info_file_id integer NOT NULL REFERENCES info_file(id),
  PRIMARY KEY (info_file_id, requires_info_file_id)
);

CREATE TABLE info_file_requires_after (
  info_file_id integer NOT NULL REFERENCES info_file(id),
  requires_info_file_id integer NOT NULL REFERENCES info_file(id),
  PRIMARY KEY (info_file_id, requires_info_file_id)
);