# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_client_app_get_delete**](DefaultApi.md#delete_client_app_get_delete) | **DELETE** /client_apps/{id} | 
[**get_client_app_get**](DefaultApi.md#get_client_app_get) | **GET** /client_apps/{id} | 
[**get_client_app_get_all**](DefaultApi.md#get_client_app_get_all) | **GET** /client_apps | 
[**get_docs**](DefaultApi.md#get_docs) | **GET** /docs | 
[**get_health_check**](DefaultApi.md#get_health_check) | **GET** /health_check | 
[**get_swagger_json**](DefaultApi.md#get_swagger_json) | **GET** /docs/swagger.json | 
[**post_client_app_post**](DefaultApi.md#post_client_app_post) | **POST** /client_apps/{id} | 
[**put_client_app_put**](DefaultApi.md#put_client_app_put) | **PUT** /client_apps | 



## delete_client_app_get_delete

> String delete_client_app_get_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_client_app_get

> crate::models::ClientAppSchema get_client_app_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::ClientAppSchema**](ClientAppSchema.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_client_app_get_all

> Vec<crate::models::ClientAppSchema> get_client_app_get_all()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ClientAppSchema>**](ClientAppSchema.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_docs

> get_docs()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_health_check

> get_health_check()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_swagger_json

> get_swagger_json()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_client_app_post

> crate::models::ClientAppSchema post_client_app_post(id, request_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**request_body** | Option<[**ClientAppSchema**](ClientAppSchema.md)> |  |  |

### Return type

[**crate::models::ClientAppSchema**](ClientAppSchema.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_client_app_put

> crate::models::ClientAppSchema put_client_app_put()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ClientAppSchema**](ClientAppSchema.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

