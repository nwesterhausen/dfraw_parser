//! Metadata about the raw files and the raw objects themselves. Used for searching and other metadata-related tasks.

mod location_helper;
mod numeric_token;
mod parser_options;
mod raw_location;
mod raw_metadata;
mod raw_object;

pub use location_helper::LocationHelper;
pub use numeric_token::NumericToken;
pub use parser_options::ParserOptions;
pub use raw_location::RawModuleLocation;

/// Metadata about the raw file
pub use raw_metadata::Metadata as RawMetadata;
pub use raw_object::RawObject;
