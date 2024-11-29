//! Library for parsing Dwarf Fortress raw files and module information files into
//! a structured format. Stores in a `SQLite` database for easy querying.
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use std::{env, error::Error};

/// Data model definitions.
pub mod models;
/// Database schema definitions.
pub mod schema;

/// Embed our migrations
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

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

/// Run the migrations to bring database up to current schema.
///
/// # Errors
///
/// Returns an error if the migrations fail or if the connection to the database fails.
pub fn run_migrations(
    connection: &mut impl MigrationHarness<diesel::sqlite::Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}
