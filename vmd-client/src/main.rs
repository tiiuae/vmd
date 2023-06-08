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
	apis::Error as ApiError,
	models::ErrorModel,
};
use serde_json::json;

mod cli;

use cli::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
	#[error("YAML error: {0}")]
	Yaml(#[from] serde_yaml::Error),
    #[error("API error: {0}")]
    Api(String),
}

async fn read_pem(file: &Path) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::new();
    File::open(file)
        .await?
        .read_to_end(&mut buf)
        .await?;
    Ok(buf)
}

async fn print_output(result: Result<serde_json::Value, ApiError<ErrorModel>>, format: cli::OutputFormat) -> Result<(), Error> {
	match result {
		Ok(v) => {
			match format {
				cli::OutputFormat::Json => {
					println!("{}", serde_json::to_string_pretty(&v)?);
				}
				cli::OutputFormat::Yaml => {
					println!("{}", serde_yaml::to_string(&v)?);
				}
				cli::OutputFormat::Text => {
					println!("{}", v);
				}
			}
		}
		Err(e) => {
			println!("{}", e);
		}
	}
	Ok(())
}

async fn run_client() -> Result<(), Error> {
    let args = Cli::parse();

    info!("{:#?}", args);
    let certs = read_pem(&args.global.connection.cert).await?;
    let certs = ReqwestIdentity::from_pem(&certs)?;
    let cacert = read_pem(&args.global.connection.cacert).await?;
    let cacert = ReqwestCertificate::from_pem(&cacert)?;
    let client = ReqwestClient::builder()
        .use_rustls_tls()
        .tls_built_in_root_certs(false)
        .add_root_certificate(cacert)
        .identity(certs)
        .https_only(true)
        .build()?;

    let mut config = Configuration::new();
    config.base_path = format!("https://{}:{}/api/v1", args.global.connection.hostname, args.global.connection.port);
    config.client = client;

    match args.operation {
		cli::Operation::List => {
			let vms = get_vm_list(&config).await.unwrap();
			print_output(Ok(json!(vms)), args.global.output).await?;
		}
		cli::Operation::Info { id } => {
			let id = json!(id);
			let vm = get_vm_info_by_id(&config, id).await.unwrap();
			print_output(Ok(json!(vm)), args.global.output).await?;
		}
		cli::Operation::Action { id, action } => {
			let id = json!(id);
			let action = json!(action);
			let vm = vm_action(&config, id, action).await.unwrap();
			print_output(Ok(json!(vm)), args.global.output).await?;
		}
	}
    Ok(())
}

#[tokio::main]
async fn main() {
    let client = run_client();
    client.await.unwrap();
}
