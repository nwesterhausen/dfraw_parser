#![allow(dead_code)]
//! Reference table DDL and documentation.
//!
//! This module defines the SQL DDL for reference tables used across all raws.
//! These tables enumerate known tokens or categories that can be referenced
//! by other tables (for example, object types, biomes, module locations, and
//! token tags).

/// `ref_object_types`
///
/// Purpose:
/// - Enumerates object types parsed from raw input (for example, `CREATURE`,
///   `ITEM_WEAPON`, etc.). The table maps a textual token to a numeric id used
///   by other tables to indicate an object's type.
///
/// Columns:
/// - `id` (INTEGER PRIMARY KEY)
/// - `token` (TEXT NOT NULL UNIQUE) — canonical token for the object type
///
/// Notes:
/// - The `token` is unique so other tables can reference the type via `id`.
pub const REF_OBJECT_TYPE_TABLE: &str = r"
 CREATE TABLE ref_object_types (
     id INTEGER PRIMARY KEY,
     token TEXT NOT NULL UNIQUE
 );";

/// `ref_module_locations`
///
/// Purpose:
/// - Represents the location/source category of a module (for example,
///   official vanilla data vs user mods). Useful for grouping modules by where
///   they came from and for reconstructing module file paths.
///
/// Columns:
/// - `id` (INTEGER PRIMARY KEY)
/// - `token` (TEXT NOT NULL UNIQUE) — short token for the location (e.g., `Vanilla`, `Mods`)
/// - `path_fragment` (TEXT) — optional path fragment used to build file paths
pub const REF_MODULE_LOCATIONS_TABLE: &str = r"
 CREATE TABLE ref_module_locations (
     id INTEGER PRIMARY KEY,
     token TEXT NOT NULL UNIQUE,
     path_fragment TEXT
 );";

/// `ref_biomes`
///
/// Purpose:
/// - Enumerates biome tokens referenced by creature/terrain metadata.
///
/// Columns:
/// - `id` (INTEGER PRIMARY KEY)
/// - `token` (TEXT NOT NULL UNIQUE) — biome token (e.g., `GLACIER`, `ANY_LAND`)
/// - `name` (TEXT NOT NULL) — human-readable biome name
pub const REF_BIOMES_TABLE: &str = r"
 CREATE TABLE ref_biomes (
     id INTEGER PRIMARY KEY,
     token TEXT NOT NULL UNIQUE,
     name TEXT NOT NULL
 );";

/// `ref_caste_token_tags`
///
/// Purpose:
/// - Reference table for caste token tags. These tokens represent boolean or
///   enumerated caste properties (for example, `AMPHIBIOUS`, `FLIER`, `WEBIMMUNE`).
///
/// Columns:
/// - `id` (INTEGER PRIMARY KEY)
/// - `token` (TEXT NOT NULL UNIQUE) — caste tag token
pub const REF_CASTE_TOKEN_TAGS: &str = r"
 CREATE TABLE ref_caste_token_tags (
     id INTEGER PRIMARY KEY,
     token TEXT NOT NULL UNIQUE
 );";

/// `ref_lair_token_tags`
///
/// Purpose:
/// - Reference tokens for lair types used by lair-related caste tags.
///
/// Columns:
/// - `id` (INTEGER PRIMARY KEY)
/// - `token` (TEXT NOT NULL UNIQUE) — lair token
pub const REF_LAIR_TOKEN_TAGS: &str = r"
 CREATE TABLE ref_lair_token_tags (
     id INTEGER PRIMARY KEY,
     token TEXT NOT NULL UNIQUE
 );";

/// `ref_secretion_triggers`
///
/// Purpose:
/// - Tokens representing secretion trigger types referenced by secretion
///   definitions (used by caste secretion rows).
///
/// Columns:
/// - `id` (INTEGER PRIMARY KEY)
/// - `token` (TEXT NOT NULL UNIQUE) — secretion trigger token
pub const REF_SECRETION_TRIGGERS: &str = r"
 CREATE TABLE ref_secretion_triggers (
     id INTEGER PRIMARY KEY,
     token TEXT NOT NULL UNIQUE
 );";
