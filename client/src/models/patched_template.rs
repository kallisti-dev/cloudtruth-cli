/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your configuration parameters and secrets making them easier to manage and use as a team.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

/// PatchedTemplate : A parameter template in a given project, optionally instantiated against an environment.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PatchedTemplate {
    /// The templates this value references, if interpolated.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// A unique identifier for the template.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The template name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// ('A description of the template.  You may find it helpful to document how this template is used to assist others when they need to maintain software that uses this content.',)
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// If true, the `body` field has undergone evaluation.
    #[serde(rename = "evaluated", skip_serializing_if = "Option::is_none")]
    pub evaluated: Option<bool>,
    /// The content of the template.  Use mustache-style templating delimiters of `{{` and `}}` to reference parameter values by name for substitution into the template result.
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Projects (other than this template's project) that this template referenced
    #[serde(
        rename = "referenced_projects",
        skip_serializing_if = "Option::is_none"
    )]
    pub referenced_projects: Option<Vec<String>>,
    /// Parameters that this template references.
    #[serde(
        rename = "referenced_parameters",
        skip_serializing_if = "Option::is_none"
    )]
    pub referenced_parameters: Option<Vec<String>>,
    /// Other templates that this template references.
    #[serde(
        rename = "referenced_templates",
        skip_serializing_if = "Option::is_none"
    )]
    pub referenced_templates: Option<Vec<String>>,
    /// Other templates that reference this template.
    #[serde(
        rename = "referencing_templates",
        skip_serializing_if = "Option::is_none"
    )]
    pub referencing_templates: Option<Vec<String>>,
    /// The dynamic values that reference this template.
    #[serde(rename = "referencing_values", skip_serializing_if = "Option::is_none")]
    pub referencing_values: Option<Vec<String>>,
    /// If True, this template contains secrets.
    #[serde(rename = "has_secret", skip_serializing_if = "Option::is_none")]
    pub has_secret: Option<bool>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
}

impl PatchedTemplate {
    /// A parameter template in a given project, optionally instantiated against an environment.
    pub fn new() -> PatchedTemplate {
        PatchedTemplate {
            url: None,
            id: None,
            name: None,
            description: None,
            evaluated: None,
            body: None,
            referenced_projects: None,
            referenced_parameters: None,
            referenced_templates: None,
            referencing_templates: None,
            referencing_values: None,
            has_secret: None,
            created_at: None,
            modified_at: None,
        }
    }
}
