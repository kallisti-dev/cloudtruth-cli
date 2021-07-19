/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your parameters and secrets making them easier to manage and use.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceAccountCreateRequest {
    /// The name of the process or system using the service account.
    #[serde(rename = "name")]
    pub name: String,
    /// An optional description of the process or system using the service account.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ServiceAccountCreateRequest {
    pub fn new(name: String) -> ServiceAccountCreateRequest {
        ServiceAccountCreateRequest {
            name,
            description: None,
        }
    }
}