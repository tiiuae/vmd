# \VmApi

All URIs are relative to *https://localhost/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_vm_info_by_id**](VmApi.md#get_vm_info_by_id) | **GET** /vm/info/{id} | Gets virtual machine info by ID
[**get_vm_list**](VmApi.md#get_vm_list) | **GET** /vm/list | Get list IDs for all virtual machines
[**vm_action**](VmApi.md#vm_action) | **PUT** /vm/{action}/{id} | Request to perform a control action on the virtual machine by its ID



## get_vm_info_by_id

> crate::models::VirtualMachineInfo get_vm_info_by_id(id)
Gets virtual machine info by ID

Gets the virtual machine information by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | The ID of the virtual machine | [required] |

### Return type

[**crate::models::VirtualMachineInfo**](VirtualMachineInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vm_list

> serde_json::Value get_vm_list()
Get list IDs for all virtual machines

Gets list of IDs for all virtual machines

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vm_action

> crate::models::VirtualMachineInfo vm_action(action, id)
Request to perform a control action on the virtual machine by its ID

Control the virtual machine by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action** | [**serde_json::Value**](.md) | An action is a command used to control the virtual machine | [required] |
**id** | [**serde_json::Value**](.md) | ID of the virtual machine | [required] |

### Return type

[**crate::models::VirtualMachineInfo**](VirtualMachineInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

