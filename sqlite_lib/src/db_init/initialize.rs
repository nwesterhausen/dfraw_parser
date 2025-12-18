//! The initialization function

use turso::Connection;

/// Initialize the database schema.
///
/// There are no data integrity checks, so it's possible to end up in an inconsistent state.
///
/// # Errors
/// Returns an error if any of the SQL statements fail.
pub async fn initialize_database(
    db: &turso::Database,
) -> Result<(), std::boxed::Box<dyn std::error::Error>> {
    let conn = db.connect()?;
    // Enable foreign key constraints
    conn.execute("PRAGMA foreign_keys = ON;", ()).await?;

    // Apply the table structure
    // Reference tables
    conn.execute(super::reference::REF_BIOMES_TABLE, ()).await?;
    conn.execute(super::reference::REF_CASTE_TOKEN_TAGS, ())
        .await?;
    conn.execute(super::reference::REF_CONDITION_TOKEN_TAGS, ())
        .await?;
    conn.execute(
        super::reference::REF_CREATURE_EFFECT_PROPERTY_TOKEN_TAGS,
        (),
    )
    .await?;
    conn.execute(super::reference::REF_CREATURE_EFFECT_TOKEN_TAGS, ())
        .await?;
    conn.execute(super::reference::REF_CREATURE_TOKEN_TAGS, ())
        .await?;
    conn.execute(super::reference::REF_CREATURE_VARIATION_TOKEN_TAGS, ())
        .await?;
    conn.execute(super::reference::REF_ENTITY_TOKEN_TAGS, ())
        .await?;
    conn.execute(super::reference::REF_LAIR_TOKEN_TAGS, ())
        .await?;
    conn.execute(super::reference::REF_MODULE_LOCATIONS_TABLE, ())
        .await?;
    conn.execute(super::reference::REF_OBJECT_TYPE_TABLE, ())
        .await?;
    conn.execute(super::reference::REF_SECRETION_TRIGGERS, ())
        .await?;

    // Reference data
    super::insert_ref_biome_tags(&conn).await?;
    super::insert_ref_caste_tags(&conn).await?;
    super::insert_ref_condition_tags(&conn).await?;
    super::insert_ref_creature_effect_property_tags(&conn).await?;
    super::insert_ref_creature_effect_tags(&conn).await?;
    super::insert_ref_creature_tags(&conn).await?;
    super::insert_ref_creature_variation_tags(&conn).await?;
    super::insert_ref_entity_tags(&conn).await?;
    super::insert_ref_lair_tags(&conn).await?;
    super::insert_ref_secretion_triggers(&conn).await?;
    super::insert_ref_object_types(&conn).await?;

    tracing::info!("reference tables created");

    // Metadata tables
    conn.execute(super::metadata::RAW_MODULES_TABLE, ()).await?;
    conn.execute(super::metadata::RAW_FILES_TABLE, ()).await?;
    conn.execute(super::metadata::RAW_OBJECTS_TABLE, ()).await?;

    tracing::info!("metadata tables created");

    // Dynamic flags/tags tables
    conn.execute(super::misc::MATERIALS_IN_STATE_TABLE, ())
        .await?;
    conn.execute(super::misc::ITEMS_OF_MATERIAL_TABLE, ())
        .await?;
    conn.execute(super::misc::CREATURE_CASTE_TABLE, ()).await?;
    conn.execute(super::misc::NAMES_TABLE, ()).await?;
    conn.execute(super::misc::BODY_PART_GROUPS_TABLE, ())
        .await?;

    tracing::info!("dynamic flags/tags tables created");
    // Parsed object tables
    conn.execute(super::tile::TILES_TABLE, ()).await?;
    conn.execute(super::color::COLORS_TABLE, ()).await?;
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

    tracing::info!("other object tables created");

    conn.execute(super::creature::CREATURES_TABLE, ()).await?;
    conn.execute(super::creature::CREATURE_BIOMES_TABLE, ())
        .await?;

    tracing::info!("creature tables created");

    create_caste_tables(&conn).await?;

    tracing::info!("caste tables created");

    // Set the schema version to the latest version
    let update_user_version = format!("PRAGMA user_version = {}", super::LATEST_SCHEMA_VERSION);

    tracing::info!("user_version updated");
    conn.execute(&update_user_version, ()).await?;

    Ok(())
}

/// Run the SQL to create the various tables for storing `[dfraw_parser::Caste]` data
///
/// # Error
///
/// Passes database errors along
async fn create_caste_tables(
    conn: &Connection,
) -> Result<(), std::boxed::Box<dyn std::error::Error>> {
    conn.execute(super::caste::CASTES_TABLE, ()).await?;
    conn.execute(super::caste::CASTE_TAGS_TABLE, ()).await?;
    conn.execute(super::caste::CASTE_VALUE_TAGS_TABLE, ())
        .await?;
    conn.execute(super::caste::CASTE_ATTACKS_TABLE, ()).await?;
    conn.execute(super::caste::CASTE_ATTACK_TRIGGERS_TABLE, ())
        .await?;
    conn.execute(super::caste::CASTE_BODY_DETAIL_PLANS_TABLE, ())
        .await?;
    conn.execute(super::caste::CASTE_BODY_DETAIL_PLAN_ARGS_TABLE, ())
        .await?;
    conn.execute(super::caste::CASTE_MATERIAL_TAGS_TABLE, ())
        .await?;
    conn.execute(super::caste::CASTE_ITEM_TAGS_TABLE, ())
        .await?;
    conn.execute(super::caste::CASTE_GAITS, ()).await?;
    conn.execute(super::caste::CASTE_TILES_TABLE, ()).await?;
    conn.execute(super::caste::CASTE_CREATURE_CASTE_TAGS_TABLE, ())
        .await?;
    conn.execute(super::caste::CASTE_LAIRS_TABLE, ()).await?;
    conn.execute(super::caste::CASTE_NAMES, ()).await?;
    conn.execute(super::caste::CASTE_PROFESSION_NAMES, ())
        .await?;
    conn.execute(super::caste::CASTE_SECRETIONS_TABLE, ())
        .await?;
    conn.execute(super::caste::CASTE_SPECIFIC_FOODS_TABLE, ())
        .await?;
    conn.execute(super::caste::CASTE_COLOR_TAGS_TABLE, ())
        .await?;

    Ok(())
}
