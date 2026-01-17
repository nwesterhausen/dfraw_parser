//! Helper to setup a tracing subscriber exactly once

use std::{fs::File, sync::Once};

static INIT: Once = Once::new();

pub fn setup_tracing() {
    INIT.call_once(|| {
        let file = File::create("./tests.log").expect("failed to create log file");
        // One-time tracing setup
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .with_writer(file)
            .with_ansi(false)
            .compact()
            .try_init();
    });
}
