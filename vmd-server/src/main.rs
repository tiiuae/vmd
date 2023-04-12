// === Module declarations ====================================================

mod server;
mod vmd;

// === External crates ========================================================

use clap::Parser;
use std::process::exit;
use log::error;

use crate::server::cli::Args;
use crate::server::run::run;

// === Implementations ========================================================

#[tokio::main]
async fn main() {
    env_logger::init();
    run(&Args::parse()).await.unwrap_or_else(|e| {
        error!("VIRTUAL MACHINE DAEMON HAS ENCOUNTERED AN ERROR AND MUST EXIT");
        error!("Error: {}", e);
        exit(1);
    });
}

// ===  EOF  ==================================================================
