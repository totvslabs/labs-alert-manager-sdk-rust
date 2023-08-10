# \PoliciesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_policies_delete**](PoliciesApi.md#delete_policies_delete) | **DELETE** /policies/{id} | 
[**get_policies_get**](PoliciesApi.md#get_policies_get) | **GET** /policies/{id} | 
[**get_policies_get_all**](PoliciesApi.md#get_policies_get_all) | **GET** /policies | 
[**post_policies_post**](PoliciesApi.md#post_policies_post) | **POST** /policies | 
[**put_policies_put**](PoliciesApi.md#put_policies_put) | **PUT** /policies/{id} | 



## delete_policies_delete

> crate::models::PolicySchema delete_policies_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::PolicySchema**](PolicySchema.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_policies_get

> crate::models::PolicySchema get_policies_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::PolicySchema**](PolicySchema.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_policies_get_all

> crate::models::PaginationSchema get_policies_get_all()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PaginationSchema**](PaginationSchema.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_policies_post

> crate::models::PolicySchema post_policies_post(request_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | Option<[**PolicySchema**](PolicySchema.md)> |  |  |

### Return type

[**crate::models::PolicySchema**](PolicySchema.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_policies_put

> crate::models::PolicySchema put_policies_put(id, request_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**request_body** | Option<[**PolicySchema**](PolicySchema.md)> |  |  |

### Return type

[**crate::models::PolicySchema**](PolicySchema.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

