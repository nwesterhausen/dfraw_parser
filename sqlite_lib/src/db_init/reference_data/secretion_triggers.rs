use turso::Connection;

/// Inserts all the known values for secretion triggers.
///
/// # Errors
///
/// Will error if there's a database error.
pub async fn insert_ref_secretion_triggers(
    conn: &Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    // string for holding all the batched sql statments
    let mut batch_sql = String::new();

    for token in ["CONTINUOUS", "EXERTION", "EXTREME_EMOTION"] {
        let insert_sql = format!("INSERT INTO ref_secretion_triggers (token) VALUES ('{token}');");
        batch_sql.push_str(&insert_sql);
    }

    conn.execute_batch(&batch_sql).await?;

    let mut count_rows = conn
        .query("SELECT COUNT(*) FROM ref_secretion_triggers;", ())
        .await?;
    let total_secretion_triggers: u64 = count_rows
        .next()
        .await?
        .ok_or("Unable to verify count of secretion_triggers")?
        .get(0)?;

    tracing::info!(
        "Inserted {total_secretion_triggers} tokens into `ref_secretion_triggers` table"
    );

    Ok(())
}
