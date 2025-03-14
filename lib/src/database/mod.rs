//! This module contains the database functionality for the application.
use once_cell::sync::Lazy;
use rusqlite::Connection;
use std::sync::Mutex;
use tracing::info;
use std::time::Instant;

/// The SQLite database module. Has helper functions relevant to our application.
pub(crate) mod sqlite;

/// The name of the SQLite database file.
pub const DB_FILE: &str = "dfraw-db.sqlite";

/// The connection to the SQLite database.
pub(crate) static DB_CONN: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = sqlite::load_database(DB_FILE).expect("Failed to load database");
    Mutex::new(conn)
});

/// Get the connection to the database.
fn get_db_conn() -> std::sync::MutexGuard<'static, Connection> {
    DB_CONN.lock().unwrap()
}

/// Initialize the database.
/// 
/// This function will load the SQLite database, and apply all migrations to it.
/// 
/// # Returns
/// 
/// An empty `Result` if successful.
/// 
/// # Errors
/// 
/// If the database fails to initialize.
pub fn init_db() -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing database at {}", DB_FILE);
    let start_time = Instant::now();
    
    sqlite::apply_migrations()?;

    let duration = start_time.elapsed();
    info!("Database initialized in {:?}", duration);

    Ok(())
}