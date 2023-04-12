#![allow(dead_code)]
use kvm_ioctls::{
    Kvm, Cap
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KvmError {
    #[error("No KVM support")]
    NoKvmPresent,
    #[error("Requires KVM version {REQUIRED_KVM_API_VERSION} or higher")]
    IncorrectKvmApiVersion(i32),
    #[error("Missing the following KVM extensions {0:?}")]
    MissingExtensions(Vec<Cap>),
}

pub const REQUIRED_KVM_API_VERSION: i32 = 12;

pub const REQUIRED_KVM_EXTENSIONS: [Cap; 1] = [
    Cap::UserMemory,
];

/// Checks if the KVM API version is high enough. Required version
/// is defined by `REQUIRED_KVM_API_VERSION`.
///
/// https://www.kernel.org/doc/Documentation/virtual/kvm/api.txt
///
/// # Example
///
/// ```
/// match check_kvm_api_version() {
///     Ok(version) => println!("KVM API version: {}", version),
///     Err(e) => eprintln!("Error: {:?}", e),
/// }
/// ```
pub fn check_kvm_api_version() -> Result<i32, KvmError> {
    let kvm = Kvm::new().map_err(|_| KvmError::NoKvmPresent)?;
    let version = kvm.get_api_version();
    if version < REQUIRED_KVM_API_VERSION {
        return Err(KvmError::IncorrectKvmApiVersion(version));
    }
    Ok(version)
}

/// Check if device has the required KVM extensions. In case of
/// missing extensions, returns a list of missing extensions wrapped
/// in a `KvmError::MissingExtensions(Vec<Cap>)` error.
///
/// # Example
///
/// ```
/// match check_kvm_extensions() {
///    Ok(_) => println!("Has all required KVM extensions"),
///    Err(e) => eprintln!("Error: {:?}", e),
/// }
/// ```
pub fn check_kvm_extensions() -> Result<(), KvmError> {
    let kvm = Kvm::new().map_err(|_| KvmError::NoKvmPresent)?;
    let mut missing_extensions = Vec::new();
    for extension in REQUIRED_KVM_EXTENSIONS.iter() {
        if !kvm.check_extension(*extension) {
            missing_extensions.push(*extension);
        }
    }
    if missing_extensions.is_empty() {
        Ok(())
    } else {
        Err(KvmError::MissingExtensions(missing_extensions))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_kvm_api_version() {
        match check_kvm_api_version() {
            Ok(version) => println!("KVM API version: {}", version),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    #[test]
    fn test_check_kvm_extensions() {
        match check_kvm_extensions() {
            Ok(()) => println!("KVM extensions OK"),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
}
