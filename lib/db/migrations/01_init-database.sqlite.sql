CREATE TABLE object_type (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  identifier TEXT,
  description TEXT
);

CREATE TABLE biome (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  identifier TEXT,
  description TEXT
);

CREATE TABLE author (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT
);

CREATE TABLE location (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  identifier TEXT,
  path TEXT,
  relative_path TEXT
);

CREATE TABLE module (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  identifier TEXT,
  name TEXT,
  description TEXT,
  version TEXT,
  numerical_version INTEGER,
  earliest_compat_version TEXT,
  earliest_compat_numerical_version INTEGER,
  steam_data INTEGER,
  FOREIGN KEY (steam_data) REFERENCES steam_data(id)
);

CREATE TABLE module_required_module (
  module_id INTEGER,
  required_module_id INTEGER,
  PRIMARY KEY (module_id, required_module_id),
  FOREIGN KEY (module_id) REFERENCES module(id),
  FOREIGN KEY (required_module_id) REFERENCES module(id)
);

CREATE TABLE module_conflicts_with_module (
  module_id INTEGER,
  conflicting_module_id INTEGER,
  PRIMARY KEY (module_id, conflicting_module_id),
  FOREIGN KEY (module_id) REFERENCES module(id),
  FOREIGN KEY (conflicting_module_id) REFERENCES module(id)
);

CREATE TABLE module_required_before_module (
  module_id INTEGER,
  required_before_module_id INTEGER,
  PRIMARY KEY (module_id, required_before_module_id),
  FOREIGN KEY (module_id) REFERENCES module(id),
  FOREIGN KEY (required_before_module_id) REFERENCES module(id)
);

CREATE TABLE module_required_after_module (
  module_id INTEGER,
  required_after_module_id INTEGER,
  PRIMARY KEY (module_id, required_after_module_id),
  FOREIGN KEY (module_id) REFERENCES module(id),
  FOREIGN KEY (required_after_module_id) REFERENCES module(id)
);

CREATE TABLE raw_metadata (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  identifier TEXT,
  module INTEGER,
  module_location INTEGER,
  object_type INTEGER,
  raw_file_path TEXT,
  FOREIGN KEY (module) REFERENCES module(id),
  FOREIGN KEY (module_location) REFERENCES location(id),
  FOREIGN KEY (object_type) REFERENCES object_type(id)
);

CREATE TABLE steam_data (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  file_id TEXT,
  title TEXT,
  description TEXT,
  changelog TEXT
);

CREATE TABLE steam_tag (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT,
  has_value INTEGER
);

CREATE TABLE steam_data_2_tag (
  steam_data_id INTEGER,
  steam_tag_id INTEGER,
  PRIMARY KEY (steam_data_id, steam_tag_id),
  FOREIGN KEY (steam_data_id) REFERENCES steam_data(id),
  FOREIGN KEY (steam_tag_id) REFERENCES steam_tag(id)
);

CREATE TABLE steam_data_2_value_tag (
  steam_data_id INTEGER,
  steam_tag_id INTEGER,
  value TEXT,
  PRIMARY KEY (steam_data_id, steam_tag_id),
  FOREIGN KEY (steam_data_id) REFERENCES steam_data(id),
  FOREIGN KEY (steam_tag_id) REFERENCES steam_tag(id)
);

CREATE TABLE creature (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  metadata INTEGER,
  identifier TEXT,
  tile INTEGER,
  frequency INTEGER,
  cluster_number INTEGER,
  population_number INTEGER,
  underground_depth INTEGER,
  general_baby_name INTEGER,
  name INTEGER,
  copy_tags_from_identifier TEXT,
  FOREIGN KEY (metadata) REFERENCES raw_metadata(id),
  FOREIGN KEY (tile) REFERENCES tile(id),
  FOREIGN KEY (cluster_number) REFERENCES min_max_data(id),
  FOREIGN KEY (population_number) REFERENCES min_max_data(id),
  FOREIGN KEY (underground_depth) REFERENCES min_max_data(id),
  FOREIGN KEY (general_baby_name) REFERENCES name_data(id),
  FOREIGN KEY (name) REFERENCES name_data(id)
);

CREATE INDEX idx_creature_identifier ON creature(identifier);

CREATE TABLE apply_creature_variation_step (
  creature_id INTEGER,
  raw_step_text TEXT,
  FOREIGN KEY (creature_id) REFERENCES creature(id)
);

CREATE TABLE select_creature_variation_step (
  creature_id INTEGER,
  raw_step_text TEXT,
  FOREIGN KEY (creature_id) REFERENCES creature(id)
);

CREATE TABLE creature_pref_strings (
  creature_id INTEGER,
  pref_string TEXT,
  PRIMARY KEY (creature_id, pref_string),
  FOREIGN KEY (creature_id) REFERENCES creature(id)
);

CREATE TABLE creature_2_biome (
  creature_id INTEGER,
  biome_id INTEGER,
  PRIMARY KEY (creature_id, biome_id),
  FOREIGN KEY (creature_id) REFERENCES creature(id),
  FOREIGN KEY (biome_id) REFERENCES biome(id)
);

CREATE TABLE creature_2_tags (
  creature_id INTEGER,
  raw_tag_id INTEGER,
  PRIMARY KEY (creature_id, raw_tag_id),
  FOREIGN KEY (creature_id) REFERENCES creature(id),
  FOREIGN KEY (raw_tag_id) REFERENCES raw_tag(id)
);

CREATE TABLE creature_2_caste (
  creature_id INTEGER,
  caste_id INTEGER,
  PRIMARY KEY (creature_id, caste_id),
  FOREIGN KEY (creature_id) REFERENCES creature(id),
  FOREIGN KEY (caste_id) REFERENCES caste(id)
);

CREATE TABLE caste (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  description TEXT,
  baby_name INTEGER,
  caste_name INTEGER,
  child_name INTEGER,
  clutch_size INTEGER,
  litter_size INTEGER,
  max_age INTEGER,
  baby INTEGER,
  child INTEGER,
  difficult INTEGER,
  egg_size INTEGER,
  grass_trample INTEGER,
  grazer INTEGER,
  low_light_vision INTEGER,
  pet_value INTEGER,
  pop_ratio INTEGER,
  change_body_size_percentage INTEGER,
  body_size INTEGER,
  milkable INTEGER,
  tile INTEGER,
  FOREIGN KEY (baby_name) REFERENCES name_data(id),
  FOREIGN KEY (caste_name) REFERENCES name_data(id),
  FOREIGN KEY (child_name) REFERENCES name_data(id),
  FOREIGN KEY (clutch_size) REFERENCES min_max_data(id),
  FOREIGN KEY (litter_size) REFERENCES min_max_data(id),
  FOREIGN KEY (max_age) REFERENCES min_max_data(id),
  FOREIGN KEY (body_size) REFERENCES body_size(id),
  FOREIGN KEY (milkable) REFERENCES milkable(id),
  FOREIGN KEY (tile) REFERENCES tile(id)
);

CREATE TABLE tile (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  character TEXT,
  alt_character TEXT,
  color INTEGER,
  glow_character TEXT,
  glow_color INTEGER,
  FOREIGN KEY (color) REFERENCES color(id),
  FOREIGN KEY (glow_color) REFERENCES color(id)
);

CREATE TABLE milkable (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  material INTEGER,
  frequency INTEGER,
  FOREIGN KEY (material) REFERENCES material_data(id)
);

CREATE INDEX idx_milkable_material_frequency ON milkable(material, frequency);

CREATE TABLE body_size (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  size_cm3 INTEGER,
  years INTEGER,
  days INTEGER
);

CREATE TABLE gait_type (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  identifier TEXT
);

CREATE TABLE gait (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  gait_type INTEGER,
  identifier TEXT,
  max_speed INTEGER,
  energy_use INTEGER,
  FOREIGN KEY (gait_type) REFERENCES gait_type(id)
);

CREATE TABLE caste_2_gait (
  caste_id INTEGER,
  gait_id INTEGER,
  PRIMARY KEY (caste_id, gait_id),
  FOREIGN KEY (caste_id) REFERENCES caste(id),
  FOREIGN KEY (gait_id) REFERENCES gait(id)
);

CREATE TABLE creature_class (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  identifier TEXT,
  description TEXT
);

CREATE TABLE caste_2_creature_class (
  caste_id INTEGER,
  creature_class_id INTEGER,
  PRIMARY KEY (caste_id, creature_class_id),
  FOREIGN KEY (caste_id) REFERENCES caste(id),
  FOREIGN KEY (creature_class_id) REFERENCES creature_class(id)
);

CREATE TABLE caste_2_tags (
  caste_id INTEGER,
  raw_tag_id INTEGER,
  PRIMARY KEY (caste_id, raw_tag_id),
  FOREIGN KEY (caste_id) REFERENCES caste(id),
  FOREIGN KEY (raw_tag_id) REFERENCES raw_tag(id)
);

CREATE TABLE raw_tag (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  identifier TEXT,
  name TEXT,
  description TEXT,
  has_value INTEGER,
  has_complex_value INTEGER
);

CREATE TABLE tag_2_value (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  raw_tag_id INTEGER,
  value TEXT,
  raw_value TEXT,
  FOREIGN KEY (raw_tag_id) REFERENCES raw_tag(id)
);

CREATE INDEX idx_raw_tag_id_2_value ON tag_2_value (raw_tag_id, value);

CREATE TABLE min_max_data (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  minimum INTEGER NOT NULL,
  maximum INTEGER NOT NULL
);

CREATE TABLE name_data (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  singular TEXT,
  plural TEXT,
  adjective TEXT
);

CREATE TABLE material_data (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  identifier TEXT,
  state INTEGER,
  FOREIGN KEY (state) REFERENCES state_of_matter(id)
);

CREATE TABLE state_of_matter (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  state TEXT
);

CREATE TABLE color (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  identifier TEXT,
  foreground INTEGER,
  background INTEGER,
  brightness INTEGER
);

CREATE TABLE temperature_data (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  specific_heat INTEGER,
  ignition_point INTEGER,
  melting_point INTEGER,
  boiling_point INTEGER,
  heat_damage_point INTEGER,
  cold_damage_point INTEGER,
  material_fixed_temperature INTEGER
);

CREATE TABLE color_modification (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  modification TEXT
);

CREATE TABLE creature_effect (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  severity INTEGER,
  probability INTEGER,
  start INTEGER,
  peak INTEGER,
  end INTEGER,
  dwf_stretch INTEGER
);

CREATE TABLE creature_effect_2_body_part_category (
  creature_effect_id INTEGER,
  body_part_category TEXT,
  FOREIGN KEY (creature_effect_id) REFERENCES creature_effect(id)
);

CREATE TABLE creature_effect_2_body_part_type (
  creature_effect_id INTEGER,
  body_part_type TEXT,
  FOREIGN KEY (creature_effect_id) REFERENCES creature_effect(id)
);

CREATE TABLE creature_effect_2_body_part_token (
  creature_effect_id INTEGER,
  body_part_token TEXT,
  FOREIGN KEY (creature_effect_id) REFERENCES creature_effect(id)
);

CREATE TABLE creature_effect_2_tags (
  creature_effect_id INTEGER,
  raw_tag_id INTEGER,
  PRIMARY KEY (creature_effect_id, raw_tag_id),
  FOREIGN KEY (creature_effect_id) REFERENCES creature_effect(id),
  FOREIGN KEY (raw_tag_id) REFERENCES raw_tag(id)
);

CREATE TABLE dimension_data (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  x INTEGER,
  y INTEGER
);

CREATE TABLE entity (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  metadata INTEGER,
  identifier TEXT,
  creature INTEGER,
  translation TEXT,
  exclusive_start_biome INTEGER,
  max_pop_number INTEGER DEFAULT 0,
  max_site_pop_number INTEGER DEFAULT 0,
  max_starting_civ_number INTEGER DEFAULT 0,
  friendly_color INTEGER,
  religion TEXT,
  land_holder_trigger TEXT,
  active_season TEXT,
  banditry REAL,
  progress_trigger_population INTEGER DEFAULT 0,
  progress_trigger_production INTEGER DEFAULT 0,
  progress_trigger_trade INTEGER DEFAULT 0,
  progress_trigger_population_siege INTEGER DEFAULT 0,
  progress_trigger_production_siege INTEGER DEFAULT 0,
  progress_trigger_trade_siege INTEGER DEFAULT 0,
  FOREIGN KEY (metadata) REFERENCES raw_metadata(id),
  FOREIGN KEY (creature) REFERENCES creature(id),
  FOREIGN KEY (exclusive_start_biome) REFERENCES biome(id),
  FOREIGN KEY (friendly_color) REFERENCES color(id)
);

CREATE TABLE entity_2_ethic (
  entity_id INTEGER,
  identifier TEXT,
  value TEXT,
  FOREIGN KEY (entity_id) REFERENCES entity(id)
);

CREATE TABLE entity_2_value (
  entity_id INTEGER,
  identifier TEXT,
  strength INTEGER,
  FOREIGN KEY (entity_id) REFERENCES entity(id)
);

CREATE TABLE entity_2_variable_value (
  entity_id INTEGER,
  identifier TEXT,
  strength INTEGER,
  FOREIGN KEY (entity_id) REFERENCES entity(id),
  FOREIGN KEY (strength) REFERENCES min_max_data(id)
);

CREATE TABLE entity_2_site_variable_position (
  entity_id INTEGER,
  position_id INTEGER,
  PRIMARY KEY (entity_id, position_id),
  FOREIGN KEY (entity_id) REFERENCES entity(id),
  FOREIGN KEY (position_id) REFERENCES position(id)
);

CREATE TABLE entity_2_variable_position (
  entity_id INTEGER,
  position_id INTEGER,
  PRIMARY KEY (entity_id, position_id),
  FOREIGN KEY (entity_id) REFERENCES entity(id),
  FOREIGN KEY (position_id) REFERENCES position(id)
);

CREATE TABLE entity_2_position (
  entity_id INTEGER,
  position_id INTEGER,
  PRIMARY KEY (entity_id, position_id),
  FOREIGN KEY (entity_id) REFERENCES entity(id),
  FOREIGN KEY (position_id) REFERENCES position(id)
);

CREATE TABLE position (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  identifier TEXT,
  appointed_by TEXT,
  color INTEGER,
  commander TEXT,
  demand_max INTEGER DEFAULT 0,
  execution_skill TEXT,
  gender TEXT,
  land_holder INTEGER DEFAULT 0,
  land_name TEXT,
  mandate_max INTEGER DEFAULT 0,
  name INTEGER,
  name_male INTEGER,
  name_female INTEGER,
  number INTEGER DEFAULT -1,
  precedence INTEGER DEFAULT -1,
  replaced_by TEXT,
  required_bedroom INTEGER DEFAULT 0,
  required_boxes INTEGER DEFAULT 0,
  required_cabinets INTEGER DEFAULT 0,
  required_dining INTEGER DEFAULT 0,
  required_office INTEGER DEFAULT 0,
  required_racks INTEGER DEFAULT 0,
  required_stands INTEGER DEFAULT 0,
  required_tomb INTEGER DEFAULT 0,
  required_population INTEGER DEFAULT 0,
  spouse_name INTEGER,
  spouse_name_male INTEGER,
  spouse_name_female INTEGER,
  squad TEXT,
  succession TEXT,
  FOREIGN KEY (color) REFERENCES color(id),
  FOREIGN KEY (name) REFERENCES name_data(id),
  FOREIGN KEY (name_male) REFERENCES name_data(id),
  FOREIGN KEY (name_female) REFERENCES name_data(id),
  FOREIGN KEY (spouse_name) REFERENCES name_data(id),
  FOREIGN KEY (spouse_name_male) REFERENCES name_data(id),
  FOREIGN KEY (spouse_name_female) REFERENCES name_data(id)
);

CREATE TABLE position_2_tag (
  position_id INTEGER,
  raw_tag_id INTEGER,
  PRIMARY KEY (position_id, raw_tag_id),
  FOREIGN KEY (position_id) REFERENCES position(id),
  FOREIGN KEY (raw_tag_id) REFERENCES raw_tag(id)
);

CREATE TABLE position_responsibility (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  identifier TEXT,
  description TEXT
);

CREATE TABLE position_2_responsibility (
  position_id INTEGER,
  responsibility_id INTEGER,
  PRIMARY KEY (position_id, responsibility_id),
  FOREIGN KEY (position_id) REFERENCES position(id),
  FOREIGN KEY (responsibility_id) REFERENCES position_responsibility(id)
);

CREATE TABLE position_2_rejecteded_creature (
  position_id INTEGER,
  creature_id INTEGER,
  PRIMARY KEY (position_id, creature_id),
  FOREIGN KEY (position_id) REFERENCES position(id),
  FOREIGN KEY (creature_id) REFERENCES creature(id)
);

CREATE TABLE position_2_rejected_creature_class (
  position_id INTEGER,
  creature_class_id INTEGER,
  PRIMARY KEY (position_id, creature_class_id),
  FOREIGN KEY (position_id) REFERENCES position(id),
  FOREIGN KEY (creature_class_id) REFERENCES creature_class(id)
);

CREATE TABLE position_2_allowed_creature (
  position_id INTEGER,
  creature_id INTEGER,
  PRIMARY KEY (position_id, creature_id),
  FOREIGN KEY (position_id) REFERENCES position(id),
  FOREIGN KEY (creature_id) REFERENCES creature(id)
);

CREATE TABLE position_2_allowed_creature_class (
  position_id INTEGER,
  creature_class_id INTEGER,
  PRIMARY KEY (position_id, creature_class_id),
  FOREIGN KEY (position_id) REFERENCES position(id),
  FOREIGN KEY (creature_class_id) REFERENCES creature_class(id)
);

CREATE TABLE entity_2_religious_sphere_alignment (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  entity_id INTEGER,
  alignment TEXT,
  FOREIGN KEY (entity_id) REFERENCES entity(id)
);

CREATE TABLE entity_2_religious_sphere (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  entity_id INTEGER,
  sphere TEXT,
  FOREIGN KEY (entity_id) REFERENCES entity(id)
);

CREATE TABLE entity_item_cull_symbol (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  entity_id INTEGER,
  symbol TEXT,
  value TEXT,
  FOREIGN KEY (entity_id) REFERENCES entity(id)
);

CREATE TABLE entity_item_subselect_symbol (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  entity_id INTEGER,
  symbol TEXT,
  value TEXT,
  FOREIGN KEY (entity_id) REFERENCES entity(id)
);

CREATE TABLE entity_item_select_symbol (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  entity_id INTEGER,
  symbol TEXT,
  value TEXT,
  FOREIGN KEY (entity_id) REFERENCES entity(id)
);

CREATE TABLE entity_item_improvement_modifier (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  entity_id INTEGER,
  improvement TEXT,
  modifier INTEGER,
  FOREIGN KEY (entity_id) REFERENCES entity(id)
);

CREATE TABLE entity_art_image_element_modifier (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  entity_id INTEGER,
  image_element TEXT,
  modifier INTEGER,
  FOREIGN KEY (entity_id) REFERENCES entity(id)
);

CREATE TABLE entity_art_facet_modifier (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  entity_id INTEGER,
  art_facet TEXT,
  modifier INTEGER,
  FOREIGN KEY (entity_id) REFERENCES entity(id)
);

CREATE TABLE entity_currency (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  entity_id INTEGER,
  currency TEXT,
  value INTEGER,
  FOREIGN KEY (entity_id) REFERENCES entity(id)
);

CREATE TABLE entity_2_permitted_reaction (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  entity_id INTEGER,
  reaction TEXT,
  FOREIGN KEY (entity_id) REFERENCES entity(id)
);

CREATE TABLE entity_2_permitted_job (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  entity_id INTEGER,
  job TEXT,
  FOREIGN KEY (entity_id) REFERENCES entity(id)
);

CREATE TABLE entity_2_permitted_building (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  entity_id INTEGER,
  building TEXT,
  FOREIGN KEY (entity_id) REFERENCES entity(id)
);

CREATE TABLE entity_2_world_construction (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  entity_id INTEGER,
  construction TEXT,
  FOREIGN KEY (entity_id) REFERENCES entity(id)
);

CREATE TABLE entity_2_tolerated_site (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  entity_id INTEGER,
  site TEXT,
  FOREIGN KEY (entity_id) REFERENCES entity(id)
);

CREATE TABLE entity_2_liked_site (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  entity_id INTEGER,
  site TEXT,
  FOREIGN KEY (entity_id) REFERENCES entity(id)
);

CREATE INDEX idx_entity_2_liked_site_entity_id_site ON entity_2_liked_site (entity_id, site);

CREATE TABLE entity_2_start_biome (
  entity_id INTEGER,
  biome_id INTEGER,
  PRIMARY KEY (entity_id, biome_id),
  FOREIGN KEY (entity_id) REFERENCES entity(id),
  FOREIGN KEY (biome_id) REFERENCES biome(id)
);

CREATE TABLE entity_2_settlement_biome (
  entity_id INTEGER,
  biome_id INTEGER,
  PRIMARY KEY (entity_id, biome_id),
  FOREIGN KEY (entity_id) REFERENCES entity(id),
  FOREIGN KEY (biome_id) REFERENCES biome(id)
);

CREATE TABLE entity_2_biome_support (
  entity_id INTEGER,
  biome_support_id INTEGER,
  PRIMARY KEY (entity_id, biome_support_id),
  FOREIGN KEY (entity_id) REFERENCES entity(id),
  FOREIGN KEY (biome_support_id) REFERENCES biome_support(id)
);

CREATE TABLE biome_support (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  biome INTEGER,
  support INTEGER,
  FOREIGN KEY (biome) REFERENCES biome(id)
);

CREATE INDEX idx_biome_support_biome_support ON biome_support (biome, support);

CREATE TABLE entity_2_tag (
  entity_id INTEGER,
  raw_tag_id INTEGER,
  PRIMARY KEY (entity_id, raw_tag_id),
  FOREIGN KEY (entity_id) REFERENCES entity(id),
  FOREIGN KEY (raw_tag_id) REFERENCES raw_tag(id)
);