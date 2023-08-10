// === Api ====================================================================
//
// This module contains API hooks. The API definitions are generated by the
// openapi-generator. The generated request handlers call the functions in this
// module in order to query the server state and to perform actions. In case
// the API definitions change, this module needs to be updated accordingly.
//
// The API interface is provided as a trait `Api<C>` which is implemented for
// the `ApiImpl<C>` struct. The trait is generic over a context type `C` which
// is used to pass additional information to the API handlers.
//
// === External crates ========================================================

use swagger::{
    Has,
    XSpanIdString,
    ApiError,
};
use async_trait::async_trait;
use std::marker::PhantomData;
use log::info;
use sysinfo::{ProcessExt, System, SystemExt, PidExt};

// === Internal modules =======================================================

use vmd_rust_server_api::{
    Api,
    GetVmInfoByIdResponse,
    GetVmListResponse,
    VmActionResponse,
};

// === Implementations ========================================================

const HYPERVISORS: [&str; 1] = ["qemu"];

#[derive(Copy, Clone)]
pub struct ApiImpl<C> {
    marker: PhantomData<C>,
}

impl<C> ApiImpl<C> {
    pub fn new() -> Self {
        ApiImpl{marker: PhantomData}
    }
}

#[async_trait]
impl<C> Api<C> for ApiImpl<C> where C: Has<XSpanIdString> + Send + Sync
{
    async fn get_vm_info_by_id(
        &self,
        id: serde_json::Value,
        context: &C) -> Result<GetVmInfoByIdResponse, ApiError>
    {
        let context = context.clone();
        info!("get_vm_info_by_id({:?}) - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn get_vm_list(&self, context: &C) -> Result<GetVmListResponse, ApiError> {
        let context = context.clone();
        info!("get_vm_list() - X-Span-ID: {:?}", context.get().0.clone());

        // Initialize the sysinfo system and refresh process information
        let mut sys = System::new();
        sys.refresh_processes();

        // Filter processes based on hypervisor command names
        let pids: Vec<u32> = sys.processes()
            .iter()
            .filter(|(_pid, process)| {
                HYPERVISORS.iter().any(|h| process.cmd().iter().any(|cmd| cmd.contains(h)))
            })
            .map(|(pid, _process)| pid.as_u32())
            .collect();

        let json = serde_json::to_string(&pids).unwrap();
        let value = serde_json::from_str(&json).unwrap();

        Ok(GetVmListResponse::ListOfIDsForAllVirtualMachines(value))
    }

    async fn vm_action(
        &self,
        action: serde_json::Value,
        id: serde_json::Value,
        context: &C) -> Result<VmActionResponse, ApiError>
    {
        let context = context.clone();
        info!("vm_action({:?}, {:?}) - X-Span-ID: {:?}", action, id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }
}

// ===  EOF  ==================================================================
