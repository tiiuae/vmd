// === Module declarations ====================================================

pub(crate) mod run;
pub(crate) mod cli;
mod tls;
mod util;
mod api;

// === External crates ========================================================

use clap::Parser;
use std::process::exit;
use log::error;

use crate::cli::Args;
use crate::run::run;

// === Implementations ========================================================

#[tokio::main]
async fn main() {
    env_logger::init();
    run(&Args::parse()).await.unwrap_or_else(|e| {
        error!("VMD server has encountered a critical error and will now exit!");
        error!("Error: {}", e);
        exit(1);
    });
}

// ===  EOF  ==================================================================
