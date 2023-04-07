// === External crates ========================================================

use tokio::net::TcpListener;
use hyper::{
    server::conn::Http,
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
    let addr = (addr, port).to_socket_addrs()?.next().unwrap();
    let server = Server::new();
    let service = MakeService::new(server);
    let service = MakeAllowAllAuthenticator::new(service, "vm");
    let mut service = MakeAddContext::<_, EmptyContext>::new(service);
    let acceptor = VmdTlsAcceptor::new(ca, crt, key)?;
    let listener = TcpListener::bind(&addr).await?;
    loop {
        let (stream, peer_addr) = listener.accept().await?;
        match acceptor.accept(stream).await {
            Ok(stream) => {
                println!("Accepted connection from {}", peer_addr);
                let service = service.call(addr);
                tokio::spawn(async move {
                    let service = service.await.expect("Error creating service");
                    Http::new()
                        .serve_connection(stream, service)
                        .await
                        .expect("Error serving connection");
                });
            }
            Err(e) => {
                panic!("Error accepting connection from {}: {}", peer_addr, e);
            }
        }
    }
}

// ===  EOF  ==================================================================
