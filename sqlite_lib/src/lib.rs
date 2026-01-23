//! This library provides a way to interact with a database, specifically for storing the DF Raw data.
//!
//! # Note
//!
//! This is an experimental library, it is not yet ready for production use. Currently it only will create a database and may function to insert
//! or read data but has not been thoroughly tested.

mod db;
mod numeric_filter;
pub(crate) mod search_helpers;
mod search_query;
mod search_results;

pub use db::client::DbClient;
pub use db::client_options::ClientOptions;
pub use db::metadata;
pub use db::models;
pub use numeric_filter::*;
pub use search_query::SearchQuery;
pub use search_results::*;
