//! Insert helper methods for the `[Caste]` struct.
use dfraw_parser::Caste;
use turso::params;

use crate::client::DbClient;

impl DbClient {
    /// Inserts a `[Caste]` into the database.
    ///
    /// # Parameters
    ///
    /// - `caste`: the `[Caste]` to insert
    /// - `creature_id`: the id of the parent `[dfraw_parser::Creature]` in the creatures table
    ///
    /// # Errors
    ///
    /// Will error if a database interaction fails.
    pub async fn insert_caste(
        &self,
        caste: &Caste,
        creature_id: i64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // First, we need a `caste_id` to work with, so basic info insertion
        let conn = self.get_connection()?;

        conn.execute(
            super::INSERT_CASTE_IDENTITY,
            params![creature_id, caste.get_identifier()],
        )
        .await?;
        // Grab the id for what we inserted
        let mut id_rows = conn
            .query(
                super::GET_ID_BY_CREATURE_AND_IDENTIFIER,
                params![creature_id, caste.get_identifier()],
            )
            .await?;

        let caste_id: i64 = id_rows
            .next()
            .await?
            .ok_or("No ID found after caste identity insertion")?
            .get(0)?;

        // Now we will loop through all the tags in the caste and insert them appropriately

        Ok(())
    }
}
