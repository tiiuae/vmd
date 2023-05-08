# vm_api

All URIs are relative to *https://localhost/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
**getVmInfoById**](vm_api.md#getVmInfoById) | **GET** /vm/info/{id} | Gets virtual machine info by ID
**getVmList**](vm_api.md#getVmList) | **GET** /vm/list | Get list IDs for all virtual machines
**vmAction**](vm_api.md#vmAction) | **PUT** /vm/{action}/{id} | Request to perform a control action on the virtual machine by its ID


# **getVmInfoById**
> models::VirtualMachineInfo getVmInfoById(id)
Gets virtual machine info by ID

Gets the virtual machine information by ID

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | [****](.md)| The ID of the virtual machine | 

### Return type

[**models::VirtualMachineInfo**](VirtualMachineInfo.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getVmList**
> serde_json::Value getVmList()
Get list IDs for all virtual machines

Gets list of IDs for all virtual machines

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](AnyType.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **vmAction**
> models::VirtualMachineInfo vmAction(action, id)
Request to perform a control action on the virtual machine by its ID

Control the virtual machine by ID

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **action** | [****](.md)| An action is a command used to control the virtual machine | 
  **id** | [****](.md)| ID of the virtual machine | 

### Return type

[**models::VirtualMachineInfo**](VirtualMachineInfo.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

