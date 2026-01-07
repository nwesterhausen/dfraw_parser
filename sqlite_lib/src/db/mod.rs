//! Provides database functionality.

pub mod client;
pub mod client_options;
pub mod metadata;
mod metadata_markers;
mod migrate;
mod migrations;
mod queries;
mod rusqlite_extensions;
pub mod search_query;
pub mod search_results;
mod util;
