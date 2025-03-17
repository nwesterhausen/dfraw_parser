-- This migration creates the table for complex values of raw tags.
CREATE TABLE raw_tag_complex_value (
                                       id INTEGER PRIMARY KEY AUTOINCREMENT,
                                       raw_tag_id INTEGER NOT NULL,
                                       identifier TEXT NOT NULL,
                                       name TEXT NOT NULL,
                                       description TEXT,

                                       FOREIGN KEY (raw_tag_id) REFERENCES raw_tag(id),
                                       UNIQUE(raw_tag_id, identifier)
);

-- Create indexes for efficient lookups
CREATE INDEX idx_raw_tag_complex_value_tag_id ON raw_tag_complex_value(raw_tag_id);
CREATE INDEX idx_raw_tag_complex_value_identifier ON raw_tag_complex_value(identifier);


