use dfraw_parser::tags::ConditionTag;
use strum::IntoEnumIterator;
use turso::Connection;

/// Inserts all the values for `[ConditionTag]` into the `ref_condition_token_tags` table
///
/// # Errors
///
/// Will error if there's a database error.
pub async fn insert_ref_condition_tags(
    conn: &Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    // string for holding all the batched sql statements
    let mut batch_sql = String::new();

    for cond_tag in ConditionTag::iter() {
        let Some(token) = cond_tag.get_key() else {
            continue;
        };

        // escape single quotes in token just in case
        let escaped = token.replace('\'', "''");
        let insert_sql =
            format!("INSERT INTO ref_condition_token_tags (token) VALUES ('{escaped}');");
        batch_sql.push_str(&insert_sql);
    }

    if !batch_sql.is_empty() {
        conn.execute_batch(&batch_sql).await?;
    }

    let mut count_rows = conn
        .query("SELECT COUNT(*) FROM ref_condition_token_tags;", ())
        .await?;
    let total_condition_tags: u64 = count_rows
        .next()
        .await?
        .ok_or("Unable to verify count of condition tags")?
        .get(0)?;

    tracing::info!("Inserted {total_condition_tags} tokens into `ref_condition_token_tags` table");

    Ok(())
}
