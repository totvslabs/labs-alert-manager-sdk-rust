# \NotificationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_notification_log_get**](NotificationsApi.md#get_notification_log_get) | **GET** /notifications/{id} | 
[**get_notification_log_get_all**](NotificationsApi.md#get_notification_log_get_all) | **GET** /notifications | 
[**post_notification_log_resend**](NotificationsApi.md#post_notification_log_resend) | **POST** /notifications/{id}/resend | 



## get_notification_log_get

> crate::models::NotificationSchema get_notification_log_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::NotificationSchema**](NotificationSchema.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification_log_get_all

> crate::models::PaginationSchema get_notification_log_get_all()


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


## post_notification_log_resend

> crate::models::NotificationSchema post_notification_log_resend(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::NotificationSchema**](NotificationSchema.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

