-- Remove all object types
DELETE
FROM object_type;
-- Reset the auto-increment counter
DELETE
FROM sqlite_sequence
WHERE name = 'object_type';