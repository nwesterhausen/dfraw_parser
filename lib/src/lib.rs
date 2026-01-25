//! This library provides an API for parsing Dwarf Fortress raw files.

mod error;
mod parsed_definitions;
mod parser;
mod reader;

pub mod constants;
pub mod legends_export;
pub mod metadata;
pub mod raw_definitions;
pub mod regex;
pub mod traits;
pub mod utilities;

pub use error::Parser as ParserError;
pub use parsed_definitions::custom_types;
pub use parsed_definitions::*;
pub use parser::ParseResult;
pub use parser::parse::parse;
pub use parser::parse_location;
pub use parser::parse_module;
pub use parser::parse_module_info_file_in_module;
pub use parser::parse_module_info_files;
pub use parser::parse_module_info_files_at_location;
pub use reader::FileParseResult;
pub use reader::UnprocessedRaw;
pub use reader::parse_raw_file;
