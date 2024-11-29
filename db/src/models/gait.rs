//! The gait model and supporting types
#![allow(clippy::module_name_repetitions)]
use diesel::prelude::*;

/// The `Gait` struct represents a gait in the game
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::gait)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(GaitType, foreign_key = type_id))]
pub struct Gait {
    /// The primary key of the gait
    pub id: i32,
    /// The type of gait
    pub type_id: i32,
    /// The name of the gait
    pub name: String,
    /// The maximum speed of the gait
    pub max_speed: i32,
    /// The energy use of the gait
    pub energy_use: i32,
}

/// The `GaitType` struct represents a type of gait
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::gait_type)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GaitType {
    /// The primary key of the gait type
    pub id: i32,
    /// The name of the gait type
    pub name: String,
}

/// The `GaitModifier` struct represents a modifier for a gait
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::gait_modifier)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GaitModifier {
    /// The primary key of the gait modifier
    pub id: i32,
    /// The name of the gait modifier
    pub name: String,
}

/// The `GaitModifierValue` struct represents a value for a gait modifier
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::gait_modifier_value)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(GaitModifier, foreign_key = gait_modifier_id))]
pub struct GaitModifierValue {
    /// The primary key of the gait modifier value
    pub gait_modifier_id: i32,
    /// The name of the gait modifier value
    pub name: String,
    /// The value of the gait modifier value
    pub value: i32,
}
