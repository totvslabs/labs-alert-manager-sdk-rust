# EventSchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_source** | **String** | Event souce | 
**client_uuid** | **String** | Client uuid. This is the id defined by client app | 
**created_at** | Option<**String**> |  | [optional]
**data** | [**serde_json::Value**](.md) | Event data | 
**event_type** | **String** | Event type | 
**id** | **String** | Id | 
**labels** | [**serde_json::Value**](.md) | Event labels | 
**schema_version** | **String** | Event schema version. Can be used by client app to know how to parse the event | 
**severity** | **String** | Event severity | 
**status** | **String** | Event status, Received, Stored | 
**updated_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


