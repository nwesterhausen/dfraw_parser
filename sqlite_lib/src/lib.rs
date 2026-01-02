//! This library provides a way to interact with a database, specifically for storing the DF Raw data.
//!
//! # Note
//!
//! This is an experimental library, it is not yet ready for production use. Currently it only will create a database and may function to insert
//! or read data but has not been thoroughly tested.

pub mod client;
pub(crate) mod db_init;
pub mod queries;
pub mod util;
