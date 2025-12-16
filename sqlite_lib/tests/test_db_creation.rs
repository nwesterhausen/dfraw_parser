//! Test the database creation/initialization process

use std::{fs, process::ExitCode};

const TEST_DB_NAME: &str = "test.db";

/// Create and test initialize the database
#[pollster::test]
async fn create_and_init_database() -> ExitCode {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(tracing::Level::TRACE)
        // make it pretty
        .compact()
        // completes the builder.
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let Ok(client) = sqlite_lib::client::DbClient::new(TEST_DB_NAME).await else {
        return ExitCode::FAILURE;
    };

    if client.init().await.is_ok() {
        // cleanup_test_db();
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

#[allow(dead_code)]
fn cleanup_test_db() {
    match fs::remove_file(TEST_DB_NAME) {
        Ok(()) => println!("Removed test file."),
        Err(error) => println!("{error:?}"),
    }
}
