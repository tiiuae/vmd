mod server;
mod util;
mod tls;
mod run;
mod cli;
mod kvm_util;

use clap::Parser;
use std::process::exit;
use log::error;

const INFO_HEADER: &str = "
=== Virtual Machine Daemon Server ===
";

#[tokio::main]
async fn main() {
    let args = crate::cli::Args::parse();
    env_logger::init();
    println!("{}", INFO_HEADER);
    crate::run::mtls_server(
        &args.addr,
        args.port,
        &args.cacert,
        &args.cert,
        &args.key,
    ).await.unwrap_or_else(|e| {
        error!("VIRTUAL MACHINE DAEMON HAS ENCOUNTERED AN ERROR AND MUST EXIT");
        error!("Error: {}", e);
        exit(1);
    });
}
