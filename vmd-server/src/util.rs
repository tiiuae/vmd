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
    #[error("KVM error: {0}")]
    KvmError(#[from] crate::kvm_util::KvmError),
    #[error("Invalid certificate: {0}")]
    InvalidCertificate(String),
}

