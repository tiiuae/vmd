// === Util ===================================================================
//
// Error and Result types used by the VMD.
//
// === Implementations ========================================================

pub type VmdResult<T> = std::result::Result<T, VmdError>;

#[derive(Debug, thiserror::Error)]
pub enum VmdError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("TLS error: {0}")]
    TlsError(#[from] tokio_rustls::rustls::Error),
    #[error("HTTP error: {0}")]
    ServerError(#[from] hyper::Error),
    #[error("API error: {0}")]
    ApiError(#[from] swagger::ApiError),
    #[error("Invalid certificate: {0}")]
    InvalidCertificate(String),
}

// === EOF ====================================================================
