//! Helpers used in regards to compiling data used for filling search indices

use dfraw_parser::{Creature, Plant, tags::ObjectType, traits::RawObject};

/// Given a raw object (via `&Box<dyn RawObject>`) will extract names and descriptions to use in
/// the search indices.
#[allow(clippy::borrowed_box)]
pub fn extract_names_and_descriptions(raw: &Box<dyn RawObject>) -> (Vec<&str>, Vec<&str>) {
    // Metadata extraction for search index
    let mut search_names = Vec::<&str>::new();
    let mut search_descriptions = Vec::<&str>::new();

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
                search_descriptions.clone_from(plant.get_pref_strings().as_ref());
            }
        }
        _ => {}
    }

    (search_names, search_descriptions)
}
