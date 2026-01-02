//! Provides a client for interacting with the database. This holds the database "object" and provides methods for interacting with it.
use crate::{db_init, util};

/// A client for interacting with the database.
#[derive(Clone)]
pub struct DbClient {
    db: turso::Database,
}

impl DbClient {
    /// Creates a new client for interacting with the database.
    ///
    /// # Errors
    /// Returns an error if the database cannot be opened.
    pub async fn new(path: &str) -> Result<Self, turso::Error> {
        let db = turso::Builder::new_local(path).build().await?;

        Ok(Self { db })
    }

    /// Initializes the database.
    ///
    /// This returns early if the database is already initialized to the latest version.
    ///
    /// This could end up corrupting the database by forcing a schema migration, there are no data
    /// integrity checks in place to prevent this.
    ///
    /// # Errors
    /// Returns an error if the database cannot be initialized.
    pub async fn init(&self) -> Result<(), std::boxed::Box<dyn std::error::Error>> {
        let conn = self.get_connection()?;

        if util::get_schema_version(&conn).await? == db_init::LATEST_SCHEMA_VERSION {
            return Ok(());
        }

        db_init::initialize_database(&self.db).await?;

        Ok(())
    }

    /// Connects to the database.
    ///
    /// # Errors
    /// Returns an error if the connection cannot be established.
    pub fn get_connection(&self) -> Result<turso::Connection, turso::Error> {
        self.db.connect()
    }
}
