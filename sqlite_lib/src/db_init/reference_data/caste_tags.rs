use dfraw_parser::tags::CasteTag;
use strum::IntoEnumIterator;
use turso::Connection;

/// Inserts all the values for `[CasteTag]` into the `caste_tag` table
///
/// # Errors
///
/// Will error if there's a database error.
pub async fn insert_ref_caste_tags(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    // string for holding all the batched sql statments
    let mut batch_sql = String::new();

    for caste_token in CasteTag::iter() {
        let Some(token) = caste_token.get_key() else {
            continue;
        };

        let insert_sql = format!("INSERT INTO ref_caste_token_tags (token) VALUES ('{token}');");
        batch_sql.push_str(&insert_sql);
    }

    conn.execute_batch(&batch_sql).await?;

    let mut count_rows = conn
        .query("SELECT COUNT(*) FROM ref_caste_token_tags;", ())
        .await?;
    let total_caste_tags: u64 = count_rows
        .next()
        .await?
        .ok_or("Unable to verify count of caste tags")?
        .get(0)?;

    tracing::info!("Inserted {total_caste_tags} tokens into `ref_caste_token_tags` table");

    Ok(())
}
