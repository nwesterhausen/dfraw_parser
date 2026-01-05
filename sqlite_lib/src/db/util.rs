use std::collections::HashSet;

use rusqlite::{Connection, Error};

pub(super) fn get_current_schema_version(conn: &Connection) -> Result<i32, Error> {
    let user_version: i32 = conn.pragma_query_value(None, "user_version", |row| row.get(0))?;
    Ok(user_version)
}

/// Removes duplicate strings or substrings in a `Vec<&str>`
pub(super) fn remove_dup_strings(strv: Vec<&str>, remove_singular_when_plural: bool) -> Vec<&str> {
    let mut deduped = HashSet::new();
    for str in strv {
        str.split_whitespace().for_each(|s| {
            if !s.eq("STP") {
                deduped.insert(s);
            }
        });
    }

    if !remove_singular_when_plural {
        return deduped.into_iter().collect();
    }

    deduped
        .iter()
        .cloned()
        .filter(|&word| {
            // Check if it's a singular word (doesn't end in 's')
            if !word.ends_with('s') {
                let plural = format!("{}s", word);
                // Only keep it if the plural doesn't exist in our set
                !deduped.contains(plural.as_str())
            } else {
                // Always keep words that already end in 's'
                true
            }
        })
        .collect()
}
