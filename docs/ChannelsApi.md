# \ChannelsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_policy_channels_delete**](ChannelsApi.md#delete_policy_channels_delete) | **DELETE** /channels/{id} | 
[**get_policy_channels_get**](ChannelsApi.md#get_policy_channels_get) | **GET** /channels/{id} | 
[**get_policy_channels_get_all**](ChannelsApi.md#get_policy_channels_get_all) | **GET** /channels | 
[**post_policy_channels_post**](ChannelsApi.md#post_policy_channels_post) | **POST** /channels | 
[**post_policy_channels_test**](ChannelsApi.md#post_policy_channels_test) | **POST** /channels/test | 
[**put_policy_channels_put**](ChannelsApi.md#put_policy_channels_put) | **PUT** /channels/{id} | 



## delete_policy_channels_delete

> crate::models::PolicyChannelSchema delete_policy_channels_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::PolicyChannelSchema**](PolicyChannelSchema.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_policy_channels_get

> crate::models::PolicyChannelSchema get_policy_channels_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::PolicyChannelSchema**](PolicyChannelSchema.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_policy_channels_get_all

> crate::models::PaginationSchema get_policy_channels_get_all()


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


## post_policy_channels_post

> crate::models::PolicyChannelSchema post_policy_channels_post(request_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | Option<[**PolicyChannelSchema**](PolicyChannelSchema.md)> |  |  |

### Return type

[**crate::models::PolicyChannelSchema**](PolicyChannelSchema.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_policy_channels_test

> crate::models::PolicyChannelSchema post_policy_channels_test(request_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | Option<[**PolicyChannelSchema**](PolicyChannelSchema.md)> |  |  |

### Return type

[**crate::models::PolicyChannelSchema**](PolicyChannelSchema.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_policy_channels_put

> crate::models::PolicyChannelSchema put_policy_channels_put(id, request_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**request_body** | Option<[**PolicyChannelSchema**](PolicyChannelSchema.md)> |  |  |

### Return type

[**crate::models::PolicyChannelSchema**](PolicyChannelSchema.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

