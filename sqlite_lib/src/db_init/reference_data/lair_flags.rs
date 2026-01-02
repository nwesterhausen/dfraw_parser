use crate::util::build_batch_insert;
use turso::Connection;

/// Inserts all the known values for lair tags.
///
/// # Errors
///
/// Will error if there's a database error.
pub async fn insert_ref_lair_tags(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let tokens = [
        "SIMPLE_BURROW",
        "SIMPLE_MOUND",
        "WILDERNESS_LOCATION",
        "SHRINE",
        "LABYRINTH",
    ];
    let batch_sql = build_batch_insert("ref_lair_token_tags", "token", &tokens);

    if !batch_sql.is_empty() {
        conn.execute_batch(&batch_sql).await?;
    }

    let mut count_rows = conn
        .query("SELECT COUNT(*) FROM ref_lair_token_tags;", ())
        .await?;
    let total_lair_tags: u64 = count_rows
        .next()
        .await?
        .ok_or("Unable to verify count of lair_tags")?
        .get(0)?;

    tracing::info!("Inserted {total_lair_tags} tokens into `ref_lair_token_tags` table");

    Ok(())
}
