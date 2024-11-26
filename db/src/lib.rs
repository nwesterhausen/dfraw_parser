//! Library for parsing Dwarf Fortress raw files and module information files into
//! a structured format. Stores in a `SQLite` database for easy querying.
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

/// Data model definitions.
pub mod models;
/// Database schema definitions.
pub mod schema;
/// Create the database connection.
///
/// # Panics
///
/// Panics if the `DATABASE_URL` environment variable is not set.
#[must_use]
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}
