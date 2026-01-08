//! Helper to setup a tracing subscriber exactly once

use std::sync::Once;

static INIT: Once = Once::new();

pub fn setup_tracing() {
    INIT.call_once(|| {
        // One-time tracing setup
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .compact()
            .try_init();
    });
}
