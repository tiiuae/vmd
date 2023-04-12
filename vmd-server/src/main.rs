mod api;
mod util;
mod tls;
mod server;
mod cli;
mod kvm_util;

use clap::Parser;
use std::process::exit;
use log::error;

#[tokio::main]
async fn main() {
    env_logger::init();
    crate::server::run(&crate::cli::Args::parse()).await.unwrap_or_else(|e| {
        error!("VIRTUAL MACHINE DAEMON HAS ENCOUNTERED AN ERROR AND MUST EXIT");
        error!("Error: {}", e);
        exit(1);
    });
}
