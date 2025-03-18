-- Remove all biome records that were seeded
DELETE
FROM biome;
-- Reset the auto-increment counter
DELETE
FROM sqlite_sequence
WHERE name = 'biome';