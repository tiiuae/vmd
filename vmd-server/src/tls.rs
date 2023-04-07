// === External crates ========================================================

use std::{
    fs::File,
    io::BufReader,
    path::Path,
    sync::Arc,
};
use rustls_pemfile::{
    certs,
};
use tokio_rustls::{
    rustls::{
        ServerConfig,
        RootCertStore,
        Certificate,
        PrivateKey,
        server::AllowAnyAuthenticatedClient as ClientAuth,
    },
    TlsAcceptor,
    server::TlsStream,
};
use tokio::net::TcpStream;
use tokio_rustls::rustls::Error as TlsError;

// === Internal modules =======================================================

use crate::util::{VmdError, VmdResult};

// === Implementations ========================================================

pub(crate) struct VmdTlsAcceptor(TlsAcceptor);

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
                    println!("Found RSA key");
                    return Ok(vec![PrivateKey(key)])
                }
                Some(rustls_pemfile::Item::PKCS8Key(key)) => {
                    println!("Found PKCS8 key");
                    return Ok(vec![PrivateKey(key)])
                }
                Some(rustls_pemfile::Item::ECKey(key)) => {
                    println!("Found EC key");
                    return Ok(vec![PrivateKey(key)])
                }
                _ => {
                    return Err(TlsError::NoCertificatesPresented.into())
                }
            }
        }
    }

    fn load_client_auth(path: &Path) -> VmdResult<Arc<ClientAuth>> {
        let certs = Self::load_certs(path)?;
        let mut store = RootCertStore::empty();
        for cert in certs {
            store.add(&cert)?;
        }
        Ok(Arc::new(ClientAuth::new(store)))
    }

    fn make_config(ca: &Path, crt: &Path, key: &Path) -> VmdResult<Arc<ServerConfig>> {
        let config = ServerConfig::builder()
            .with_safe_default_cipher_suites()
            .with_safe_default_kx_groups()
            .with_safe_default_protocol_versions()?
            .with_client_cert_verifier(
                Self::load_client_auth(ca)?
            )
            .with_single_cert(
                Self::load_certs(crt)?,
                Self::load_keys(key)?.remove(0)
            )?;
        Ok(Arc::new(config))
    }

    pub fn new(ca: &Path, crt: &Path, key: &Path) -> VmdResult<Self> {
        Ok(Self(TlsAcceptor::from(Self::make_config(ca, crt, key)?)))
    }

    pub async fn accept(&self, stream: TcpStream) -> VmdResult<TlsStream<TcpStream>> {
        self.0.accept(stream).await.map_err(VmdError::from)
    }
}

// ===  EOF  ==================================================================
