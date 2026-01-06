//! Provides database functionality.

pub mod client;
pub mod client_options;
mod migrate;
mod migrations;
mod queries;
mod rusqlite_extensions;
pub mod search_query;
pub mod search_results;
mod util;
