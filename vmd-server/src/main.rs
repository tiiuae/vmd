mod server;
mod util;
mod tls;
mod run;
mod cli;
mod kvm_util;

use clap::Parser;
use std::process::exit;
use log::error;
use vmd_api::{
    BASE_PATH,
    API_VERSION,
};

#[tokio::main]
async fn main() {
    let args = crate::cli::Args::parse();
    env_logger::init();
    println!("=== Virtual Machine Daemon Server ===");
    println!("version: {}", API_VERSION);
    println!("hostname: {}", args.addr);
    println!("port: {}", args.port);
    println!("base: {}", BASE_PATH);
    println!("ca: {}", args.cacert.display());
    println!("cert: {}", args.cert.display());
    println!("key: {}", args.key.display());
    println!("");
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
