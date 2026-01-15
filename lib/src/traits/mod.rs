//! Shared traits amongst various parsed objects.
//!
//! This module defines core traits implemented by parsed raw file objects:
//! - [`RawObject`] - Base trait for all parsed raw objects
//! - [`Searchable`] - Provides search string functionality
//! - [`TokenParser`] - Helper trait for parsing raw file tokens
//! - [`TagOperations`] - Operations for manipulating tags
//! - [`CreatureVariationRequirements`] - Handling creature variation requirements
//!
//! # Examples
//!
//! Using the Searchable trait:
//! ```
//! use dfraw_parser::traits::Searchable;
//! # use dfraw_parser::Creature;
//!
//! # let creature = Creature::default();
//! let search_string = creature.get_search_vec();
//! ```

pub mod creature_variation_requirements;
pub mod raw_object;
pub mod raw_object_token;
pub mod searchable;
pub mod tag_operations;
pub mod token_parser;

pub use creature_variation_requirements::CreatureVariationRequirements;
pub use raw_object::RawObject;
pub use raw_object_token::RawObjectToken;
pub use searchable::Searchable;
pub use tag_operations::TagOperations;
pub use token_parser::TokenParser;
