mod server;
mod util;
mod tls;
mod run;
mod cli;
mod kvm_util;

use clap::Parser;
use std::process::exit;

#[tokio::main]
async fn main() {
    let args = crate::cli::Args::parse();
    crate::run::mtls_server(
        &args.addr,
        args.port,
        &args.ca,
        &args.crt,
        &args.key,
    ).await.unwrap_or_else(|e| {
        eprintln!("VIRTUAL MACHINE DAEMON HAS ENCOUNTERED AN ERROR AND MUST EXIT");
        eprintln!("Error: {}", e);
        exit(1);
    });
}
