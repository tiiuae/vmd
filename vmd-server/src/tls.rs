// === Tls ====================================================================
//
// This module implements a `VmdTlsAcceptor` for the VMD server. It is used to
// accept TLS connections and to perform the TLS handshake. Configuration of
// the TLS server can be found in the function `VmdTlsAcceptor::make_config`.
//
// === External crates ========================================================

use core::task::{Context, Poll};
use std::{
    fs::File,
    io::BufReader,
    path::Path,
    sync::Arc,
    pin::Pin,
    future::Future,
    io::Result as IoResult,
};
use tokio_rustls::{
    Accept as TlsAccept,
    TlsAcceptor,
    server::TlsStream,
    rustls::{
        ServerConfig,
        RootCertStore,
        Certificate,
        PrivateKey,
        KeyLogFile,
        Ticketer,
        Error as TlsError,
        server::{
            AllowAnyAuthenticatedClient,
            ServerSessionMemoryCache,
        }
    },
};
use tokio::io::{
    AsyncRead,
    AsyncWrite,
    ReadBuf,
};
use hyper::server::{
    conn::{AddrIncoming, AddrStream},
    accept::Accept,
};
use log::{error, trace};
use rustls_pemfile::certs;
use futures_util::ready;

// === Internal modules =======================================================

use super::util::{VmdError, VmdResult};

// === Implementations ========================================================

enum State {
    Handshaking(TlsAccept<AddrStream>),
    Streaming(TlsStream<AddrStream>),
}

pub struct VmdTlsStream {
    state: State,
}

impl VmdTlsStream {
    fn new(stream: AddrStream, config: Arc<ServerConfig>) -> VmdTlsStream {
        let accept = TlsAcceptor::from(config).accept(stream);
        VmdTlsStream {
            state: State::Handshaking(accept),
        }
    }
}

impl AsyncRead for VmdTlsStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut ReadBuf,
    ) -> Poll<IoResult<()>> {
        let pin = self.get_mut();
        match pin.state {
            State::Handshaking(ref mut accept) => match ready!(Pin::new(accept).poll(cx)) {
                Ok(mut stream) => {
                    let result = Pin::new(&mut stream).poll_read(cx, buf);
                    pin.state = State::Streaming(stream);
                    result
                }
                Err(err) => Poll::Ready(Err(err)),
            },
            State::Streaming(ref mut stream) => Pin::new(stream).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for VmdTlsStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<IoResult<usize>> {
        let pin = self.get_mut();
        match pin.state {
            State::Handshaking(ref mut accept) => match ready!(Pin::new(accept).poll(cx)) {
                Ok(mut stream) => {
                    let result = Pin::new(&mut stream).poll_write(cx, buf);
                    pin.state = State::Streaming(stream);
                    result
                }
                Err(err) => Poll::Ready(Err(err)),
            },
            State::Streaming(ref mut stream) => Pin::new(stream).poll_write(cx, buf),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<IoResult<()>> {
        match self.state {
            State::Handshaking(_) => Poll::Ready(Ok(())),
            State::Streaming(ref mut stream) => Pin::new(stream).poll_flush(cx),
        }
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<IoResult<()>> {
        match self.state {
            State::Handshaking(_) => Poll::Ready(Ok(())),
            State::Streaming(ref mut stream) => Pin::new(stream).poll_shutdown(cx),
        }
    }
}

pub(crate) struct VmdTlsAcceptor {
    config: Arc<ServerConfig>,
    incoming: AddrIncoming,
}

impl VmdTlsAcceptor {
    fn load_certs(path: &Path) -> VmdResult<Vec<Certificate>> {
        let mut buf = BufReader::new(File::open(path)?);
        let certs = certs(&mut buf)?;
        Ok(certs.into_iter().map(Certificate).collect())
    }

    fn load_keys(path: &Path) -> VmdResult<Vec<PrivateKey>> {
        let mut buf = BufReader::new(File::open(path)?);
        loop {
            match rustls_pemfile::read_one(&mut buf)? {
                Some(rustls_pemfile::Item::RSAKey(key)) => {
                    trace!("Found RSA key");
                    return Ok(vec![PrivateKey(key)])
                }
                Some(rustls_pemfile::Item::PKCS8Key(key)) => {
                    trace!("Found PKCS8 key");
                    return Ok(vec![PrivateKey(key)])
                }
                Some(rustls_pemfile::Item::ECKey(key)) => {
                    trace!("Found EC key");
                    return Ok(vec![PrivateKey(key)])
                }
                _ => {
                    return Err(TlsError::NoCertificatesPresented.into())
                }
            }
        }
    }

    fn load_client_auth(path: &Path) -> VmdResult<Arc<AllowAnyAuthenticatedClient>> {
        let mut buf = BufReader::new(File::open(path)?);
        let certs = certs(&mut buf)?;
        let mut store = RootCertStore::empty();
        let (_, invalid) = store.add_parsable_certificates(&certs);
        if invalid > 0 {
            return Err(VmdError::InvalidCertificate(format!("{} invalid certificates", invalid)))
        }
        trace!("Certificate authority found");
        Ok(Arc::new(AllowAnyAuthenticatedClient::new(store)))
    }

    fn make_config(ca: &Path, crt: &Path, key: &Path) -> VmdResult<Arc<ServerConfig>> {
        let mut config = ServerConfig::builder()
            .with_safe_defaults()
            .with_client_cert_verifier(
                Self::load_client_auth(ca)?
            )
            .with_single_cert(
                Self::load_certs(crt)?,
                Self::load_keys(key)?.remove(0)
            )?;
        config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
        config.key_log = Arc::new(KeyLogFile::new());
        config.session_storage = ServerSessionMemoryCache::new(256);
        config.ticketer = Ticketer::new().unwrap();
        trace!("{:#?}", config);
        Ok(Arc::new(config))
    }

    pub fn new(incoming: AddrIncoming, ca: &Path, crt: &Path, key: &Path) -> VmdResult<Self> {
        let config = Self::make_config(ca, crt, key)?;
        Ok(Self { config, incoming })
    }
}

impl Accept for VmdTlsAcceptor {
    type Conn = VmdTlsStream;
    type Error = VmdError;

    fn poll_accept(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
        let pin = self.get_mut();
        match ready!(Pin::new(&mut pin.incoming).poll_accept(cx)) {
            Some(Ok(sock)) => {
                trace!("Ready to accept TLS connection from {:#?}", sock);
                Poll::Ready(Some(Ok(VmdTlsStream::new(sock, pin.config.clone()))))
            }
            Some(Err(err)) => {
                error!("TLS accept error: {}", err);
                Poll::Ready(Some(Err(err.into())))
            }
            None => {
                trace!("Poll accept returned None");
                Poll::Ready(None)
            }
        }
    }
}

// ===  EOF  ==================================================================
