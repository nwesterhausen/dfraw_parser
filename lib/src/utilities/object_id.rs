use uuid::Uuid;

use crate::{
    constants::DFRAW_PARSER_NAMESPACE,
    metadata::{RawMetadata, RawModuleLocation},
    tokens::ObjectType,
};

/// Generate a UUID to use as an `object_id` for a given raw object or info file.
///
/// Requires a location, object type, version and identifier.
pub fn generate_object_id(
    location: RawModuleLocation,
    object_type: ObjectType,
    identifier: &str,
    module_numeric_version: u32,
) -> Uuid {
    // Convert the enums to stable integers.
    let location_id = u32::from(location);
    let type_id = u32::from(object_type);

    // Create a seed string with separates to avoid accidental collisions
    let seed_data = format!(
        "{location_id}-{type_id}-{}-{module_numeric_version}",
        identifier.to_lowercase()
    );

    Uuid::new_v5(&DFRAW_PARSER_NAMESPACE, seed_data.as_bytes())
}
/// Generate a UUID to use as an `object_id` for a given raw object or info file.
///
/// This requires the identifier and object type to use, and will use the metadata to
/// get the rest of the required information.
pub fn generate_object_id_using_raw_metadata(
    identifier: &str,
    object_type: ObjectType,
    raw_metadata: &RawMetadata,
) -> Uuid {
    generate_object_id(
        raw_metadata.get_location(),
        object_type,
        identifier,
        raw_metadata.get_module_numerical_version(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::ObjectType;

    #[test]
    fn test_generate_object_id_determinism() {
        let loc = RawModuleLocation::Vanilla;
        let obj_type = ObjectType::Creature;
        let ident = "TOAD";
        let version = 1;

        let id1 = generate_object_id(loc, obj_type, ident, version);
        let id2 = generate_object_id(loc, obj_type, ident, version);

        assert_eq!(
            id1, id2,
            "UUID generation should be deterministic given the same inputs"
        );
    }

    #[test]
    fn test_generate_object_id_uniqueness() {
        let loc = RawModuleLocation::Vanilla;
        let obj_type = ObjectType::Creature;
        let version = 1;

        let id1 = generate_object_id(loc, obj_type, "TOAD", version);
        let id2 = generate_object_id(loc, obj_type, "FROG", version);

        assert_ne!(id1, id2, "Different identifiers should yield different IDs");
    }
}
