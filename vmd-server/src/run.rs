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
    server::Server,
    tls::VmdTlsAcceptor,
};

// === Implementations ========================================================
pub(crate) async fn mtls_server(
    addr: &str,
    port: u16,
    ca: &Path,
    crt: &Path,
    key: &Path,
) -> VmdResult<()> {
    let addr = format!("{}:{}", addr, port);
    let addr = addr.to_socket_addrs()?.next().unwrap();
    info!("Listening on {}", addr);
    let api = Server::new();
    let service = MakeService::new(api);
    // let service = MakeAllowAllAuthenticator::new(service, "vm");
    let service = MakeAddContext::<_, EmptyContext>::new(service);
    let incoming = AddrIncoming::bind(&addr)?;
    let acceptor = VmdTlsAcceptor::new(incoming, ca, crt, key)?;
    let server = HyperServer::builder(acceptor).serve(service);
    server.await.map_err(|e| e.into())
}

// ===  EOF  ==================================================================
