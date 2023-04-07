pub type VmdResult<T> = std::result::Result<T, VmdError>;

#[derive(Debug, thiserror::Error)]
pub enum VmdError {
    #[error("IO error: {0}")]
    IoError(std::io::Error),
    #[error("TLS error: {0}")]
    TlsError(tokio_rustls::rustls::Error),
    #[error("API error: {0}")]
    ApiError(swagger::ApiError),
    #[error("KVM error: {0}")]
    KvmError(crate::kvm_util::KvmError),
}

macro_rules ! impl_from {
    ($from:ty, $to:ident) => {
        impl From<$from> for VmdError {
            fn from(e: $from) -> Self {
                VmdError::$to(e)
            }
        }
    };
}

impl_from!(std::io::Error, IoError);
impl_from!(tokio_rustls::rustls::Error, TlsError);
impl_from!(swagger::ApiError, ApiError);
impl_from!(crate::kvm_util::KvmError, KvmError);
