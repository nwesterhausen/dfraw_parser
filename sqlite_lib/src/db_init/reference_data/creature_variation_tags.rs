use crate::util::build_batch_insert;
use dfraw_parser::tags::CreatureVariationTag;
use strum::IntoEnumIterator;
use turso::Connection;

/// Inserts all the values for `[CreatureVariationTag]` into the `ref_creature_variation_token_tags` table
///
/// # Errors
///
/// Will error if there's a database error.
pub async fn insert_ref_creature_variation_tags(
    conn: &Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    // collect token strings then build a batched INSERT using the util helper
    let mut values: Vec<&str> = Vec::new();

    for tag in CreatureVariationTag::iter() {
        let Some(token) = tag.get_key() else {
            continue;
        };
        values.push(token);
    }

    let batch_sql = build_batch_insert("ref_creature_variation_token_tags", "token", &values);

    if !batch_sql.is_empty() {
        conn.execute_batch(&batch_sql).await?;
    }

    let mut count_rows = conn
        .query(
            "SELECT COUNT(*) FROM ref_creature_variation_token_tags;",
            (),
        )
        .await?;
    let total_creature_variation_tags: u64 = count_rows
        .next()
        .await?
        .ok_or("Unable to verify count of creature variation tags")?
        .get(0)?;

    tracing::info!(
        "Inserted {total_creature_variation_tags} tokens into `ref_creature_variation_token_tags` table"
    );

    Ok(())
}
