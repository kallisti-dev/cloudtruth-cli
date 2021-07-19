/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your parameters and secrets making them easier to manage and use.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

/// IntegrationExplorer : Describes the content available at a given location.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationExplorer {
    #[serde(rename = "fqn")]
    pub fqn: String,
    #[serde(rename = "node_type")]
    pub node_type: crate::models::NodeTypeEnum,
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "content_type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "content_data", skip_serializing_if = "Option::is_none")]
    pub content_data: Option<String>,
    #[serde(rename = "content_size", skip_serializing_if = "Option::is_none")]
    pub content_size: Option<i32>,
    #[serde(rename = "content_keys", skip_serializing_if = "Option::is_none")]
    pub content_keys: Option<Vec<String>>,
}

impl IntegrationExplorer {
    /// Describes the content available at a given location.
    pub fn new(fqn: String, node_type: crate::models::NodeTypeEnum) -> IntegrationExplorer {
        IntegrationExplorer {
            fqn,
            node_type,
            secret: None,
            name: None,
            content_type: None,
            content_data: None,
            content_size: None,
            content_keys: None,
        }
    }
}