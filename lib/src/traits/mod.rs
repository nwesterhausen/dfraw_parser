//! Shared traits amongst various parsed objects.

pub mod creature_variation_requirements;
pub mod database_compatability;
pub mod raw_object;
pub mod raw_object_token;
pub mod raw_object_token_to_any;
pub mod searchable;
pub mod tag_operations;

pub use creature_variation_requirements::CreatureVariationRequirements;
pub use database_compatability::{Insertable, Queryable};
pub use raw_object::RawObject;
pub use raw_object_token::RawObjectToken;
pub use raw_object_token_to_any::RawObjectTokenToAny;
pub use searchable::Searchable;
pub use tag_operations::TagOperations;
