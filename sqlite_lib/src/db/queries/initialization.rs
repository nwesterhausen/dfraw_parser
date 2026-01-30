use dfraw_parser::{metadata::RawModuleLocation, tokens::ObjectType, traits::RawToken};
use rusqlite::{Connection, Result, Transaction, params};
use strum::IntoEnumIterator;

/// Populates the lookup tables (`raw_types`, `module_locations`) using values
/// derived directly from the Rust enums.
///
/// This ensures the database IDs always match the `u32` representation
/// of the Enums in the application code.
///
/// # Errors
///
/// - database errors
pub fn init_constant_tables(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    populate_raw_types(&tx)?;
    populate_module_locations(&tx)?;

    tx.commit()
}

/// Populates the `raw_types` table using [`ObjectType`]
///
/// # Errors
///
/// - database errors
fn populate_raw_types(tx: &Transaction) -> Result<()> {
    let mut stmt = tx.prepare("INSERT OR REPLACE INTO raw_types (id, name) VALUES (?1, ?2)")?;
    for obj_type in ObjectType::iter() {
        // Try to get the raw token (e.g., "CREATURE").
        // If None, fallback to the Rust enum name (e.g., "Unknown").
        let name = obj_type
            .get_key()
            .map_or_else(|| format!("{obj_type:?}"), String::from);

        stmt.execute(params![u32::from(obj_type), name])?;
    }
    Ok(())
}

/// Populates the `module_locations` table using [`RawModuleLocation`]
///
/// # Errors
///
/// - database errors
fn populate_module_locations(tx: &Transaction) -> Result<()> {
    let mut stmt =
        tx.prepare("INSERT OR REPLACE INTO module_locations (id, name) VALUES (?1, ?2)")?;
    for location in RawModuleLocation::iter() {
        // Using Debug formatting (e.g., "WorkshopMods") for the name.
        stmt.execute(params![u32::from(location), location.to_string()])?;
    }
    Ok(())
}
