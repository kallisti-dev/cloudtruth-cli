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
pub struct PatchedGroup {
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The unique identifier of a group.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The group name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A description of the group.  You may find it helpful to document how this group is used to assist others when they need to maintain this organization.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
}

impl PatchedGroup {
    pub fn new() -> PatchedGroup {
        PatchedGroup {
            url: None,
            id: None,
            name: None,
            description: None,
            users: None,
            created_at: None,
            modified_at: None,
        }
    }
}