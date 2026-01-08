use std::collections::HashMap;

use dfraw_parser::ParseResult;
use rusqlite::{Connection, Result};
use tracing::info;

use crate::ClientOptions;

use super::insert_module_data;

/// Helper that will insert the `ParseResult` directly.
///
/// It handles breaking up the results into each module, then calling `insert_module_data` on each.
///
/// # Errors
///
/// - on database errors
pub fn insert_parse_results(
    conn: &mut Connection,
    options: &ClientOptions,
    parse_results: ParseResult,
) -> Result<()> {
    // group Raws by Module Identity
    // We use a composite key of (name, version, location_id) to match Raws to their InfoFiles.
    // This allows us to handle multi-module parsing (Vanilla + Mods) correctly.
    let mut module_map = HashMap::new();
    for raw in parse_results.raws {
        let meta = raw.get_metadata();
        let key = (
            String::from(meta.get_module_name()),
            String::from(meta.get_module_version()),
            i32::from(meta.get_location()),
        );
        module_map.entry(key).or_insert_with(Vec::new).push(raw);
    }

    // We iterate through the parsed info files and grab the raws associated with each.
    for info in &parse_results.info_files {
        let key = (
            info.get_name(),
            info.get_version(),
            i32::from(info.get_location()),
        );
        info!("Inserting raws for {key:?}");
        if let Some(module_raws) = module_map.get(&key) {
            insert_module_data(conn, options, info, module_raws)?;
        }
    }
    Ok(())
}
