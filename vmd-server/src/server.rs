// === External crates ========================================================

#![allow(unused_imports)]
use tokio::net::TcpListener;
use hyper::{
    Server as HyperServer,
    server::conn::{AddrIncoming, AddrStream},
    service::Service,
};
use swagger::{
    auth::MakeAllowAllAuthenticator,
    EmptyContext,
};
use vmd_api::{
    server::MakeService,
    server::context::MakeAddContext,
};
use std::{
    path::Path,
    net::ToSocketAddrs,
};
use log::{error, info, trace};

// === Internal modules =======================================================

use crate::{
    util::VmdResult,
    api::ApiImpl,
    tls::VmdTlsAcceptor,
    cli::Args,
};

// === Implementations ========================================================

pub(crate) async fn run(args: &Args) -> VmdResult<()>
{
    println!("⚡    Virtual Machine Daemon Server   ⚡\n");
    println!("{}", args);
    let addr = format!("{}:{}", args.addr, args.port);
    let addr = addr.to_socket_addrs()?.next().unwrap();
    info!("Listening on {}", addr);
    let service = MakeService::new(ApiImpl::new());
    let service = MakeAddContext::<_, EmptyContext>::new(service);
    let incoming = AddrIncoming::bind(&addr)?;
    let acceptor = VmdTlsAcceptor::new(
        incoming,
        &args.cacert.as_path(),
        &args.cert.as_path(),
        &args.key.as_path(),
    )?;
    let server = HyperServer::builder(acceptor).serve(service);
    server.await.map_err(|e| e.into())
}

// ===  EOF  ==================================================================
