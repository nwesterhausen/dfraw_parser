//! Helpers used in regards to compiling data used for filling search indices
use std::collections::HashSet;

use dfraw_parser::{
    Creature, Plant,
    tokens::ObjectType,
    traits::{RawObject, Searchable as _},
};

/// Given a raw object (via `&Box<dyn RawObject>`) will extract names and descriptions to use in
/// the search indices.
#[allow(clippy::borrowed_box)]
pub fn extract_names_and_descriptions(raw: &dyn RawObject) -> (Vec<String>, Vec<String>) {
    // Metadata extraction for search index
    let mut search_names = Vec::<String>::new();
    let mut search_descriptions = Vec::<String>::new();

    match raw.get_type() {
        ObjectType::Creature => {
            if let Some(creature) = raw.as_any().downcast_ref::<Creature>() {
                search_names.clone_from(&creature.get_all_names());
                search_descriptions.clone_from(&creature.get_all_descriptions());
            }
        }
        ObjectType::Plant => {
            if let Some(plant) = raw.as_any().downcast_ref::<Plant>() {
                search_names.clone_from(&plant.get_all_names());
                search_descriptions.clone_from(&plant.get_all_descriptions());
            }
        }
        _ => {}
    }

    (search_names, search_descriptions)
}

/// Removes duplicate strings or substrings in a `Vec<&str>`
///
/// This is used when condensing the names and descriptions to remove duplicates
/// to more efficiently store them for lookup.
///
/// * `strv` the [`Vec<&str>`] to clean
/// * `remove_singular_when_plural` whether to remove the singular version if the
///   list contains a plural version.
///
/// Returns a cleaned vector of `&str`
pub fn remove_dup_strings(strv: &[String], remove_singular_when_plural: bool) -> Vec<&str> {
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
        .copied()
        .filter(|&word| {
            // Check if it's a singular word (doesn't end in 's')
            if word.ends_with('s') {
                // Always keep words that already end in 's'
                true
            } else {
                let plural = format!("{word}s");
                // Only keep it if the plural doesn't exist in our set
                !deduped.contains(plural.as_str())
            }
        })
        .collect()
}
