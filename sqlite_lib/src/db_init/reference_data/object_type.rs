use dfraw_parser::metadata::ObjectType;
use strum::IntoEnumIterator;
use turso::Connection;

/// Inserts all the values for `[ObjectType]` into the `ref_object_types` table
///
/// # Errors
///
/// Will error if there's a database error.
pub async fn insert_ref_object_types(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    // string for holding all the batched sql statments
    let mut batch_sql = String::new();

    for caste_token in ObjectType::iter() {
        let Some(token) = caste_token.get_key() else {
            continue;
        };

        let insert_sql = format!("INSERT INTO ref_object_types (token) VALUES ('{token}');");
        batch_sql.push_str(&insert_sql);
    }

    conn.execute_batch(&batch_sql).await?;

    let mut count_rows = conn
        .query("SELECT COUNT(*) FROM ref_object_types;", ())
        .await?;
    let total_object_type_tags: u64 = count_rows
        .next()
        .await?
        .ok_or("Unable to verify count of object_type_tags")?
        .get(0)?;

    tracing::info!("Inserted {total_object_type_tags} tokens into `ref_object_types` table");

    Ok(())
}
