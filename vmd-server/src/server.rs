use async_trait::async_trait;
// use log::info;
use std::marker::PhantomData;
use swagger::{Has, XSpanIdString};
use swagger::ApiError;
use log::trace;
use vmd_api::{
    Api,
    GetVmInfoByIdResponse,
    GetVmListResponse,
    VmActionResponse,
};
use serde_json::Value;

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    /// Gets virtual machine info by ID
    async fn get_vm_info_by_id(
        &self,
        id: serde_json::Value,
        context: &C) -> Result<GetVmInfoByIdResponse, ApiError>
    {
        let context = context.clone();
        trace!("get_vm_info_by_id({:?}) - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get list IDs for all virtual machines
    async fn get_vm_list(
        &self,
        context: &C) -> Result<GetVmListResponse, ApiError>
    {
        let context = context.clone();
        trace!("get_vm_list() - X-Span-ID: {:?}", context.get().0.clone());
        let ids = [1, 2, 3];
        let json = serde_json::to_string(&ids).unwrap();
        let value = serde_json::from_str(&json).unwrap();
        Ok(GetVmListResponse::ListOfIDsForAllVirtualMachines(value))
    }

    /// Request to perform a control action on the virtual machine by its ID
    async fn vm_action(
        &self,
        action: serde_json::Value,
        id: serde_json::Value,
        context: &C) -> Result<VmActionResponse, ApiError>
    {
        let context = context.clone();
        trace!("vm_action({:?}, {:?}) - X-Span-ID: {:?}", action, id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }
}
