//! Queries for interacting with `[CASTE]` objects

pub mod get;
mod get_sql;
pub mod insert;
mod insert_sql;

pub use get_sql::*;
pub use insert_sql::*;
