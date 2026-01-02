use crate::util::build_batch_insert;
use dfraw_parser::tags::CasteTag;
use strum::IntoEnumIterator;
use turso::Connection;

/// Inserts all the values for `[CasteTag]` into the `ref_caste_token_tags` table
///
/// # Errors
///
/// Will error if there's a database error.
pub async fn insert_ref_caste_tags(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    // collect token strings then build a batched INSERT using the util helper
    let mut values: Vec<&str> = Vec::new();

    for caste_token in CasteTag::iter() {
        let Some(token) = caste_token.get_key() else {
            continue;
        };
        values.push(token);
    }

    let batch_sql = build_batch_insert("ref_caste_token_tags", "token", &values);

    if !batch_sql.is_empty() {
        conn.execute_batch(&batch_sql).await?;
    }

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
