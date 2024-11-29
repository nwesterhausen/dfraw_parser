-- Your SQLite goes here
CREATE TABLE object_type (
  id integer NOT NULL PRIMARY KEY autoincrement,
  name text NOT NULL
);

CREATE TABLE metadata (
  id integer NOT NULL PRIMARY KEY autoincrement,
  -- the special object_id string
  object_id text NOT NULL,
  name text NOT NULL,
  version text NOT NULL,
  raw_identifier text NOT NULL,
  object_type_id integer NOT NULL REFERENCES object_type(id),
  raw_file_path text NOT NULL,
  module_location_id integer NOT NULL REFERENCES location(id)
);

INSERT INTO
  object_type (name)
VALUES
  ("CREATURE"),
  ("INORGANIC"),
  ("PLANT"),
  ("ITEM"),
  ("ITEM_AMMO"),
  ("ITEM_ARMOR"),
  ("ITEM_FOOD"),
  ("ITEM_GLOVES"),
  ("ITEM_HELM"),
  ("ITEM_INSTRUMENT"),
  ("ITEM_PANTS"),
  ("ITEM_SHIELD"),
  ("ITEM_SHOES"),
  ("ITEM_SIEGEAMMO"),
  ("ITEM_TOOL"),
  ("ITEM_TOY"),
  ("ITEM_TRAPCOMP"),
  ("ITEM_WEAPON"),
  ("BUILDING"),
  ("BUILDING_WORKSHOP"),
  ("BUILDING_FURNACE"),
  ("REACTION"),
  ("GRAPHICS"),
  ("MATERIAL_TEMPLATE"),
  ("BODY_DETAIL_PLAN"),
  ("BODY"),
  ("ENTITY"),
  ("LANGUAGE"),
  ("TRANSLATION"),
  ("TISSUE_TEMPLATE"),
  ("CREATURE_VARIATION"),
  ("TEXT_SET"),
  ("TILE_PAGE"),
  ("DESCRIPTOR_COLOR"),
  ("DESCRIPTOR_PATTERN"),
  ("DESCRIPTOR_SHAPE"),
  ("PALETTE"),
  ("MUSIC"),
  ("SOUND"),
  ("INTERACTION");