# PolicySchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**channels** | **Vec<String>** | List of channel notification | 
**client_source** | **String** | Policy Client source | 
**client_uuid** | **String** | Policy Client UUID | 
**created_at** | Option<**String**> |  | [optional]
**deleted** | **bool** | Policy deleted | 
**enabled** | **bool** | Policy enabled | 
**filters** | [**serde_json::Value**](.md) | Policy filters | 
**frequency** | **bool** | Enable alert frequency for the policy | 
**frequency_minutes** | **i32** | Quantity of alert interval time | 
**frequency_occurrences** | **i32** | Quantity of alert occurrencies | 
**id** | **String** | Id | 
**labels** | [**serde_json::Value**](.md) | Policy labels | 
**name** | **String** | Policy name | 
**severity** | **String** | Policy severity | 
**r#type** | **String** | Policy type | 
**updated_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


