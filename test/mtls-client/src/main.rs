use log::info;
use clap::Parser;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use reqwest::{Client as ReqwestClient, Certificate as ReqwestCertificate, Identity as ReqwestIdentity};
use thiserror::Error;
use std::path::Path;
use vmd_rust_client_api::{
    apis::configuration::Configuration,
    apis::vm_api::{get_vm_info_by_id, get_vm_list, vm_action},
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("API error: {0}")]
    Api(String),
}

#[derive(Debug, clap::Parser)]
#[command(author, about, version)]
struct Args {
    #[clap(value_enum, required = true)]
    operation: String,
    #[clap(short, long, required = true)]
    port: u16,
    #[clap(long, required = true)]
    cacert: std::path::PathBuf,
    #[clap(long, required = true)]
    key: std::path::PathBuf,
    #[clap(long, required = true)]
    cert: std::path::PathBuf,
    #[clap(long, required = true)]
    hostname: String,
}

async fn read_pem(file: &Path) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::new();
    File::open(file)
        .await?
        .read_to_end(&mut buf)
        .await?;
    Ok(buf)
}

async fn run_client() -> Result<(), Error> {
    let args = Args::parse();

    info!("{:#?}", args);
    let certs = read_pem(&args.cert).await?;
    let certs = ReqwestIdentity::from_pem(&certs)?;
    let cacert = read_pem(&args.cacert).await?;
    let cacert = ReqwestCertificate::from_pem(&cacert)?;
    let client = ReqwestClient::builder()
        .use_rustls_tls()
        .tls_built_in_root_certs(false)
        .add_root_certificate(cacert)
        .identity(certs)
        .https_only(true)
        .build()?;

    let mut config = Configuration::new();
    config.base_path = format!("https://{}:{}/api/v1", args.hostname, args.port);
    config.client = client;

    match args.operation.as_str() {
        "list" => {
            let res = get_vm_list(&config).await.unwrap();
            println!("{}", res);
        }
        "info" => {
            let id = serde_json::json!(1);
            let res = get_vm_info_by_id(&config, id).await.unwrap();
            println!("{:?}", res);
        }
        "action" => {
            let id = serde_json::json!(1);
            let action = serde_json::json!("start");
            let res = vm_action(&config, id, action).await.unwrap();
            println!("{:?}", res);
        }
        _ => {
            println!("Unknown operation");
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let client = run_client();
    client.await.unwrap();
}
