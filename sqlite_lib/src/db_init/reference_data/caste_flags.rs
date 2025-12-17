use dfraw_parser::tags::CasteTag;
use strum::IntoEnumIterator;
use turso::Connection;

/// Inserts all the values for `[CasteTag]` into the `caste_flag` table
///
/// # Errors
///
/// Will error if there's a database error.
pub async fn insert_ref_caste_flags(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    // string for holding all the batched sql statments
    let mut batch_sql = String::new();

    for caste_token in CasteTag::iter() {
        let Some(token) = caste_token.get_key() else {
            continue;
        };

        let insert_sql = format!("INSERT INTO ref_caste_token_flags (token) VALUES ('{token}');");
        batch_sql.push_str(&insert_sql);
    }

    conn.execute_batch(&batch_sql).await?;

    let mut count_rows = conn
        .query("SELECT COUNT(*) FROM ref_caste_token_flags;", ())
        .await?;
    let total_caste_flags: u64 = count_rows
        .next()
        .await?
        .ok_or("Unable to verify count of caste_flags")?
        .get(0)?;

    tracing::info!("Inserted {total_caste_flags} tokens into `caste_flags` table");

    Ok(())
}
