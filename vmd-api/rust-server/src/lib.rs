#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
#![allow(unused_imports, unused_attributes)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::blacklisted_name)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};
use serde::{Serialize, Deserialize};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &str = "/api/v1";
pub const API_VERSION: &str = "1.0.1";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetVmInfoByIdResponse {
    /// successful operation
    SuccessfulOperation
    (models::VirtualMachineInfo)
    ,
    /// Invalid virtual machine ID
    InvalidVirtualMachineID
    ,
    /// Virtual machine ID is valid but not found
    VirtualMachineIDIsValidButNotFound
    ,
    /// error payload
    ErrorPayload
    (models::ErrorModel)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetVmListResponse {
    /// List of IDs for all virtual machines
    ListOfIDsForAllVirtualMachines
    (serde_json::Value)
    ,
    /// error payload
    ErrorPayload
    (models::ErrorModel)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum VmActionResponse {
    /// successful operation
    SuccessfulOperation
    (models::VirtualMachineInfo)
    ,
    /// Invalid virtual machine ID
    InvalidVirtualMachineID
    ,
    /// Virtual machine ID is valid but not found
    VirtualMachineIDIsValidButNotFound
    ,
    /// error payload
    ErrorPayload
    (models::ErrorModel)
}

/// API
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// Gets virtual machine info by ID
    async fn get_vm_info_by_id(
        &self,
        id: serde_json::Value,
        context: &C) -> Result<GetVmInfoByIdResponse, ApiError>;

    /// Get list IDs for all virtual machines
    async fn get_vm_list(
        &self,
        context: &C) -> Result<GetVmListResponse, ApiError>;

    /// Request to perform a control action on the virtual machine by its ID
    async fn vm_action(
        &self,
        action: serde_json::Value,
        id: serde_json::Value,
        context: &C) -> Result<VmActionResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// Gets virtual machine info by ID
    async fn get_vm_info_by_id(
        &self,
        id: serde_json::Value,
        ) -> Result<GetVmInfoByIdResponse, ApiError>;

    /// Get list IDs for all virtual machines
    async fn get_vm_list(
        &self,
        ) -> Result<GetVmListResponse, ApiError>;

    /// Request to perform a control action on the virtual machine by its ID
    async fn vm_action(
        &self,
        action: serde_json::Value,
        id: serde_json::Value,
        ) -> Result<VmActionResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// Gets virtual machine info by ID
    async fn get_vm_info_by_id(
        &self,
        id: serde_json::Value,
        ) -> Result<GetVmInfoByIdResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_vm_info_by_id(id, &context).await
    }

    /// Get list IDs for all virtual machines
    async fn get_vm_list(
        &self,
        ) -> Result<GetVmListResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_vm_list(&context).await
    }

    /// Request to perform a control action on the virtual machine by its ID
    async fn vm_action(
        &self,
        action: serde_json::Value,
        id: serde_json::Value,
        ) -> Result<VmActionResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().vm_action(action, id, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
