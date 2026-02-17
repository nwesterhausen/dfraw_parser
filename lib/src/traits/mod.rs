//! Shared traits amongst various parsed objects.
//!
//! This module defines core traits implemented by parsed raw file objects:
//! - [`RawObject`] - Base trait for all parsed raw objects
//! - [`TokenParser`] - Helper trait for parsing raw file tokens
//! - [`TagOperations`] - Operations for manipulating tags
//! - [`CreatureVariationRequirements`] - Handling creature variation requirements
//!

mod cleanable;
mod creature_variation_requirements;
mod is_empty;
mod numeric_tokens;
mod raw_object;
mod raw_token;
mod searchable;
mod tag_operations;
mod to_raw_string;
mod token_parser;

pub use cleanable::Cleanable;
pub use creature_variation_requirements::CreatureVariationRequirements;
pub use is_empty::IsEmpty;
pub use numeric_tokens::NumericTokenTransform;
pub use raw_object::RawObject;
pub use raw_token::RawToken;
pub use searchable::Searchable;
pub use tag_operations::TagOperations;
pub use to_raw_string::ToRawFileString;
pub use token_parser::TokenParser;
