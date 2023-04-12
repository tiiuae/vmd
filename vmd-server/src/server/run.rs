// === External crates ========================================================

use hyper::{
    Server as HyperServer,
    server::conn::AddrIncoming,
};
use swagger::EmptyContext;
use std::net::ToSocketAddrs;
use log::info;

// === Internal modules =======================================================

use vmd_rust_server_api::{
    server::MakeService,
    server::context::MakeAddContext,
};

use super::{
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
    let service = MakeService::new(ApiImpl::new());
    let service = MakeAddContext::<_, EmptyContext>::new(service);
    let incoming = AddrIncoming::bind(&addr)?;
    info!("Listening on {}", addr);
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
