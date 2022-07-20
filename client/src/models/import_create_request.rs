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
pub struct ImportCreateRequest {
    /// Project name or identifier
    #[serde(rename = "project")]
    pub project: String,
    /// Environment name or identifier
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /// Text to turn into variables
    #[serde(rename = "body")]
    pub body: String,
    /// Parameter names that should be secrets
    #[serde(rename = "secrets")]
    pub secrets: Vec<String>,
    /// Parameter names to be ignored
    #[serde(rename = "ignore")]
    pub ignore: Vec<String>,
    /// Create the project (if it does not exist)
    #[serde(rename = "add_project", skip_serializing_if = "Option::is_none")]
    pub add_project: Option<bool>,
    /// Create the environment (if it does not exist)
    #[serde(rename = "add_environment", skip_serializing_if = "Option::is_none")]
    pub add_environment: Option<bool>,
    /// Create the parameters (if they do not exist)
    #[serde(rename = "add_parameters", skip_serializing_if = "Option::is_none")]
    pub add_parameters: Option<bool>,
    /// Create the environment value override (if they do not exist)
    #[serde(rename = "add_overrides", skip_serializing_if = "Option::is_none")]
    pub add_overrides: Option<bool>,
    /// Skip adding a parameter override if it is the same
    #[serde(rename = "inherit_on_same", skip_serializing_if = "Option::is_none")]
    pub inherit_on_same: Option<bool>,
}

impl ImportCreateRequest {
    pub fn new(
        project: String,
        body: String,
        secrets: Vec<String>,
        ignore: Vec<String>,
    ) -> ImportCreateRequest {
        ImportCreateRequest {
            project,
            environment: None,
            body,
            secrets,
            ignore,
            add_project: None,
            add_environment: None,
            add_parameters: None,
            add_overrides: None,
            inherit_on_same: None,
        }
    }
}
