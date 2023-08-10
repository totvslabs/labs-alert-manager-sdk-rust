/*
 * alertmanager
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyChannelSchema {
    /// Client uuid. This is the id defined by client app
    #[serde(rename = "client_uuid")]
    pub client_uuid: String,
    /// Policy Channel config
    #[serde(rename = "config")]
    pub config: serde_json::Value,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Policy Channel deleted flag
    #[serde(rename = "deleted")]
    pub deleted: bool,
    /// Policy Channel enabled flag
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Id
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "last_notification", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_notification: Option<Option<String>>,
    /// Policy Channel name
    #[serde(rename = "name")]
    pub name: String,
    /// Policy Channel type
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl PolicyChannelSchema {
    pub fn new(client_uuid: String, config: serde_json::Value, deleted: bool, enabled: bool, id: String, name: String, r#type: String) -> PolicyChannelSchema {
        PolicyChannelSchema {
            client_uuid,
            config,
            created_at: None,
            deleted,
            enabled,
            id,
            last_notification: None,
            name,
            r#type,
            updated_at: None,
        }
    }
}


