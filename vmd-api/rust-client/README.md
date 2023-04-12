# Rust API client for vmd_rust_client_api

OpenAPI specification for virtual machine management on Ghaf platform


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.1
- Package version: 1.0.1
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `vmd_rust_client_api` and add the following to `Cargo.toml` under `[dependencies]`:

```
vmd_rust_client_api = { path = "./vmd_rust_client_api" }
```

## Documentation for API Endpoints

All URIs are relative to *https://localhost/api/v1*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*VmApi* | [**get_vm_info_by_id**](docs/VmApi.md#get_vm_info_by_id) | **GET** /vm/info/{id} | Gets virtual machine info by ID
*VmApi* | [**get_vm_list**](docs/VmApi.md#get_vm_list) | **GET** /vm/list | Get list IDs for all virtual machines
*VmApi* | [**vm_action**](docs/VmApi.md#vm_action) | **PUT** /vm/{action}/{id} | Request to perform a control action on the virtual machine by its ID


## Documentation For Models

 - [ErrorModel](docs/ErrorModel.md)
 - [VirtualMachineBasicDetails](docs/VirtualMachineBasicDetails.md)
 - [VirtualMachineCpu](docs/VirtualMachineCpu.md)
 - [VirtualMachineDevices](docs/VirtualMachineDevices.md)
 - [VirtualMachineHypervisorDetails](docs/VirtualMachineHypervisorDetails.md)
 - [VirtualMachineInfo](docs/VirtualMachineInfo.md)
 - [VirtualMachineMemory](docs/VirtualMachineMemory.md)
 - [VirtualMachineOs](docs/VirtualMachineOs.md)
 - [VirtualMachinePerformance](docs/VirtualMachinePerformance.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

nikita.bazulin@unikie.com
