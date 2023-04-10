/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your configuration parameters and secrets making them easier to manage and use as a team.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceAccountCreateRequest {
    /// The name of the process or system using the service account.
    #[serde(rename = "name")]
    pub name: String,
    /// An optional description of the process or system using the service account.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The role for the service acount
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

impl ServiceAccountCreateRequest {
    pub fn new(name: String) -> ServiceAccountCreateRequest {
        ServiceAccountCreateRequest {
            name,
            description: None,
            role: None,
        }
    }
}