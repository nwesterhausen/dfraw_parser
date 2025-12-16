//! The initialization function

/// Initialize the database schema.
///
/// There are no data integrity checks, so it's possible to end up in an inconsistent state.
///
/// # Errors
/// Returns an error if any of the SQL statements fail.
pub async fn initialize_database(db: &turso::Database) -> Result<(), turso::Error> {
    let conn = db.connect()?;
    // Enable foreign key constraints
    conn.execute("PRAGMA foreign_keys = ON;", ()).await?;

    // Apply the table structure
    // Reference tables
    conn.execute(super::reference::REF_OBJECT_TYPE_TABLE, ())
        .await?;
    conn.execute(super::reference::REF_MODULE_LOCATIONS_TABLE, ())
        .await?;
    conn.execute(super::reference::REF_BIOMES_TABLE, ()).await?;
    conn.execute(super::reference::REF_CASTE_TOKEN_FLAGS, ())
        .await?;

    // Metadata tables
    conn.execute(super::metadata::RAW_MODULES_TABLE, ()).await?;
    conn.execute(super::metadata::RAW_FILES_TABLE, ()).await?;
    conn.execute(super::metadata::RAW_OBJECTS_TABLE, ()).await?;

    // Parsed object tables
    conn.execute(super::tile::TILES_TABLE, ()).await?;

    conn.execute(super::creature::CREATURES_TABLE, ()).await?;
    conn.execute(super::creature::CREATURE_BIOMES_TABLE, ())
        .await?;

    conn.execute(super::caste::CASTES_TABLE, ()).await?;
    conn.execute(super::caste::CASTE_ATTACKS_TABLE, ()).await?;
    conn.execute(super::caste::CASTE_ATTACK_TRIGGERS_TABLE, ())
        .await?;
    conn.execute(super::caste::CASTE_BEACH_FREQUENCIES_TABLE, ())
        .await?;
    conn.execute(super::caste::CASTE_BODY_APPEARANCE_MODIFIERS_TABLE, ())
        .await?;
    conn.execute(super::caste::CASTE_BODY_DETAIL_PLANS_TABLE, ())
        .await?;
    conn.execute(super::caste::CASTE_BODY_GLOSSES_TABLE, ())
        .await?;
    conn.execute(super::caste::CASTE_BODY_SIZES_TABLE, ())
        .await?;
    conn.execute(super::caste::CASTE_CREATURE_CLASSES_TABLE, ())
        .await?;
    conn.execute(super::caste::CASTE_BODY_DETAIL_PLANS_TABLE, ())
        .await?;
    conn.execute(super::caste::CASTE_FLAGS_TABLE, ()).await?;
    conn.execute(super::caste::CASTE_GAITS, ()).await?;
    conn.execute(super::caste::CASTE_TILES_TABLE, ()).await?;

    conn.execute(
        super::creature_variation::APPLIED_CREATURE_VARIATIONS_TABLE,
        (),
    )
    .await?;
    conn.execute(
        super::creature_variation::APPLIED_CREATURE_VARIATION_ARGUMENTS_TABLE,
        (),
    )
    .await?;

    // Set the schema version to the latest version
    let update_user_version = format!("PRAGMA user_version = {}", super::LATEST_SCHEMA_VERSION);
    conn.execute(&update_user_version, ()).await?;

    Ok(())
}
