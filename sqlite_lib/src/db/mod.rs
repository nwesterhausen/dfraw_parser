//! Provides database functionality.

pub mod client;
pub mod client_options;
pub mod metadata;
mod metadata_markers;
mod migrate;
mod migrations;
pub mod models;
mod queries;
mod rusqlite_extensions;
mod util;
