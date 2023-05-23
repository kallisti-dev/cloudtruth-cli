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
pub struct ParameterTypeUpdate {
    #[serde(rename = "id")]
    pub id: String,
    /// The parameter type name.
    #[serde(rename = "name")]
    pub name: String,
    /// A description of the parameter type, provide documentation on how to use this type here.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The URL for this parameter type's parent
    #[serde(rename = "parent")]
    pub parent: String,
    /// Rules applied to this parameter.
    #[serde(rename = "rules")]
    pub rules: Vec<crate::models::ParameterTypeRule>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "modified_at")]
    pub modified_at: Option<String>,
}

impl ParameterTypeUpdate {
    pub fn new(
        id: String,
        name: String,
        parent: String,
        rules: Vec<crate::models::ParameterTypeRule>,
        created_at: String,
        modified_at: Option<String>,
    ) -> ParameterTypeUpdate {
        ParameterTypeUpdate {
            id,
            name,
            description: None,
            parent,
            rules,
            created_at,
            modified_at,
        }
    }
}
