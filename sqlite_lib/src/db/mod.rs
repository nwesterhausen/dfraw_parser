//! Provides database functionality.

mod init;
mod migrations;

pub use init::init_db;

pub const LATEST_SCHEMA_VERSION: i32 = 1;
