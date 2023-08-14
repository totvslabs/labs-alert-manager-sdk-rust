# \EventsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_events_parameters_delete**](EventsApi.md#delete_events_parameters_delete) | **DELETE** /events/parameters/{app_key} | 
[**get_events_get**](EventsApi.md#get_events_get) | **GET** /events/{id} | 
[**get_events_get_all**](EventsApi.md#get_events_get_all) | **GET** /events | 
[**get_events_parameters_get**](EventsApi.md#get_events_parameters_get) | **GET** /events/parameters | 
[**post_events_post**](EventsApi.md#post_events_post) | **POST** /events | 



## delete_events_parameters_delete

> String delete_events_parameters_delete(app_key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |

### Return type

**String**

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_events_get

> crate::models::EventSchema get_events_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::EventSchema**](EventSchema.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_events_get_all

> crate::models::PaginationSchema get_events_get_all()


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


## get_events_parameters_get

> Vec<crate::models::EventSchema> get_events_parameters_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::EventSchema>**](EventSchema.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_events_post

> crate::models::EventSchema post_events_post(request_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | Option<[**EventSchema**](EventSchema.md)> |  |  |

### Return type

[**crate::models::EventSchema**](EventSchema.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

