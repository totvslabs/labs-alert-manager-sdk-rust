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
pub struct PolicySchema {
    /// List of channel notification
    #[serde(rename = "channels")]
    pub channels: serde_json::Value,
    /// Policy Client source
    #[serde(rename = "client_source")]
    pub client_source: String,
    /// Policy Client UUID
    #[serde(rename = "client_uuid")]
    pub client_uuid: String,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Policy deleted
    #[serde(rename = "deleted")]
    pub deleted: bool,
    /// Policy enabled
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Policy filters
    #[serde(rename = "filters")]
    pub filters: serde_json::Value,
    /// Enable alert frequency for the policy
    #[serde(rename = "frequency")]
    pub frequency: bool,
    /// Quantity of alert interval time
    #[serde(rename = "frequency_minutes")]
    pub frequency_minutes: i32,
    /// Quantity of alert occurrencies
    #[serde(rename = "frequency_occurrences")]
    pub frequency_occurrences: i32,
    /// Id
    #[serde(rename = "id")]
    pub id: String,
    /// Policy labels
    #[serde(rename = "labels")]
    pub labels: serde_json::Value,
    /// Policy name
    #[serde(rename = "name")]
    pub name: String,
    /// Policy severity
    #[serde(rename = "severity")]
    pub severity: String,
    /// Policy type
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl PolicySchema {
    pub fn new(channels: serde_json::Value, client_source: String, client_uuid: String, deleted: bool, enabled: bool, filters: serde_json::Value, frequency: bool, frequency_minutes: i32, frequency_occurrences: i32, id: String, labels: serde_json::Value, name: String, severity: String, r#type: String) -> PolicySchema {
        PolicySchema {
            channels,
            client_source,
            client_uuid,
            created_at: None,
            deleted,
            enabled,
            filters,
            frequency,
            frequency_minutes,
            frequency_occurrences,
            id,
            labels,
            name,
            severity,
            r#type,
            updated_at: None,
        }
    }
}

