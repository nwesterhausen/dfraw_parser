use crate::util::build_batch_insert;
use dfraw_parser::tags::CreatureTag;
use strum::IntoEnumIterator;
use turso::Connection;

/// Inserts all the values for `[CreatureTag]` into the `ref_creature_token_tags` table
///
/// # Errors
///
/// Will error if there's a database error.
pub async fn insert_ref_creature_tags(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    // collect token strings then build a batched INSERT using the util helper
    let mut values: Vec<&str> = Vec::new();

    for creature_token in CreatureTag::iter() {
        let Some(token) = creature_token.get_key() else {
            continue;
        };
        values.push(token);
    }

    let batch_sql = build_batch_insert("ref_creature_token_tags", "token", &values);

    if !batch_sql.is_empty() {
        conn.execute_batch(&batch_sql).await?;
    }

    let mut count_rows = conn
        .query("SELECT COUNT(*) FROM ref_creature_token_tags;", ())
        .await?;
    let total_creature_tags: u64 = count_rows
        .next()
        .await?
        .ok_or("Unable to verify count of creature tags")?
        .get(0)?;

    tracing::info!("Inserted {total_creature_tags} tokens into `ref_creature_token_tags` table");

    Ok(())
}
