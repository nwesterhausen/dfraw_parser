//! Get helper operations for the reference tables

use turso::params;

use crate::client::DbClient;

impl DbClient {
    /// GET the id of a caste flag by its token text
    ///
    /// # Parameters
    ///
    /// - `token`: the string token value (e.g. 'AMPHIBOIOUS', 'BABY' etc.)
    ///
    /// # Errors
    ///
    /// Will error if a database lookup fails
    pub async fn get_caste_flag_id_by_token(
        &self,
        token: &str,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let conn = self.get_connection()?;

        let mut id_rows = conn
            .query(super::GET_CASTE_FLAG_BY_TOKEN, params![token])
            .await?;

        let token_id: i64 = id_rows
            .next()
            .await?
            .ok_or("No ID found for given token on caste_flags table")?
            .get(0)?;

        Ok(token_id)
    }
}
