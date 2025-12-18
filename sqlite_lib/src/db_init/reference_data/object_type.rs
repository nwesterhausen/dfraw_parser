use crate::util::build_batch_insert;
use dfraw_parser::metadata::ObjectType;
use strum::IntoEnumIterator;
use turso::Connection;

/// Inserts all the values for `[ObjectType]` into the `ref_object_types` table
///
/// # Errors
///
/// Will error if there's a database error.
pub async fn insert_ref_object_types(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    // collect token strings then build a batched INSERT using the util helper
    let mut values: Vec<&str> = Vec::new();

    for object_type in ObjectType::iter() {
        let Some(token) = object_type.get_key() else {
            continue;
        };
        values.push(token);
    }

    let batch_sql = build_batch_insert("ref_object_types", "token", &values);

    if !batch_sql.is_empty() {
        conn.execute_batch(&batch_sql).await?;
    }

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
