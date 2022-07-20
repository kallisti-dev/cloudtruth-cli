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
pub struct PatchedGrant {
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// A unique identifier for the grant.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The URI of a principal for the grant; this must reference a user or group.
    #[serde(rename = "principal", skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
    /// The URI of a scope for the grant; this must reference a project or environment.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// The role that the principal has in the given scope.
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Box<crate::models::RoleEnum>>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
}

impl PatchedGrant {
    pub fn new() -> PatchedGrant {
        PatchedGrant {
            url: None,
            id: None,
            principal: None,
            scope: None,
            role: None,
            created_at: None,
            modified_at: None,
        }
    }
}
