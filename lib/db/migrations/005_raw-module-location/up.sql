-- Populate the location table with RawModuleLocation enum values
INSERT INTO location (id, identifier, path, relative_path)
VALUES (1, 'InstalledMods', 'data/installed_mods', 'data/installed_mods'),
       (2, 'Mods', 'mods', 'mods'),
       (3, 'Vanilla', 'data/vanilla', 'data/vanilla'),
       (4, 'Unknown', 'unknown', 'unknown'),
       (5, 'LegendsExport', '.', '.');