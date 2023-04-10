/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your configuration parameters and secrets making them easier to manage and use as a team.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

/// AwsPush : Push actions can be configured to send configuration and secrets to integrations when tags are updated.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AwsPush {
    #[serde(rename = "url")]
    pub url: String,
    /// Unique identifier for the action.
    #[serde(rename = "id")]
    pub id: String,
    /// The action name.
    #[serde(rename = "name")]
    pub name: String,
    /// The optional description for the action.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The most recent task run for this action.
    #[serde(rename = "latest_task")]
    pub latest_task: Option<Box<crate::models::AwsPushTask>>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "modified_at")]
    pub modified_at: Option<String>,
    /// This setting allows parameters (non-secrets) to be pushed to a destination that only supports storing secrets.  This may increase your overall cost from the cloud provider as some cloud providers charge a premium for secrets-only storage.
    #[serde(rename = "coerce_parameters", skip_serializing_if = "Option::is_none")]
    pub coerce_parameters: Option<bool>,
    /// Include parameters (non-secrets) in the values being pushed.  This setting requires the destination to support parameters or for the `coerce_parameters` flag to be enabled, otherwise the push will fail.
    #[serde(rename = "include_parameters", skip_serializing_if = "Option::is_none")]
    pub include_parameters: Option<bool>,
    /// Include secrets in the values being pushed.  This setting requires the destination to support secrets, otherwise the push will fail.
    #[serde(rename = "include_secrets", skip_serializing_if = "Option::is_none")]
    pub include_secrets: Option<bool>,
    /// Include templates in the values being pushed.
    #[serde(rename = "include_templates", skip_serializing_if = "Option::is_none")]
    pub include_templates: Option<bool>,
    /// When set to dry-run mode an action will report the changes that it would have made in task steps, however those changes are not actually performed.
    #[serde(rename = "dry_run", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// Normally, push will check to see if it originated the values in the destination before making changes to them.  Forcing a push disables the ownership check.
    #[serde(rename = "force", skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// Normally, push will process all parameters including those that flow in from project dependencies.  Declaring a push as `local` will cause it to only process the parameters defined in the selected projects.
    #[serde(rename = "local", skip_serializing_if = "Option::is_none")]
    pub local: Option<bool>,
    /// Projects that are included in the push.
    #[serde(rename = "projects")]
    pub projects: Vec<String>,
    /// Tags are used to select parameters by environment from the projects included in the push.  You cannot have two tags from the same environment in the same push.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// The AWS region this push targets.  This region must be enabled in the integration.
    #[serde(rename = "region")]
    pub region: Option<Box<crate::models::AwsRegionEnum>>,
    /// The AWS service this push targets.  This service must be enabled in the integration.
    #[serde(rename = "service")]
    pub service: Option<Box<crate::models::AwsServiceEnum>>,
    /// Defines a path through the integration to the location where values will be pushed.  The following mustache-style substitutions can be used in the string:    - ``{{ environment }}`` to insert the environment name   - ``{{ parameter }}`` to insert the parameter name   - ``{{ project }}`` to insert the project name   - ``{{ push }}`` to insert the push name   - ``{{ tag }}`` to insert the tag name  We recommend that you use project, environment, and parameter at a minimum to disambiguate your pushed resource identifiers.  If you include multiple projects in the push, the `project` substitution is required.  If you include multiple tags from different environments in the push, the `environment` substitution is required.  If you include multiple tags from the same environment in the push, the `tag` substitution is required.  In all cases, the `parameter` substitution is always required.
    #[serde(rename = "resource")]
    pub resource: Option<String>,
}

impl AwsPush {
    /// Push actions can be configured to send configuration and secrets to integrations when tags are updated.
    pub fn new(
        url: String,
        id: String,
        name: String,
        latest_task: Option<crate::models::AwsPushTask>,
        created_at: String,
        modified_at: Option<String>,
        projects: Vec<String>,
        tags: Vec<String>,
        region: Option<crate::models::AwsRegionEnum>,
        service: Option<crate::models::AwsServiceEnum>,
        resource: Option<String>,
    ) -> AwsPush {
        AwsPush {
            url,
            id,
            name,
            description: None,
            latest_task: latest_task.map(Box::new),
            created_at,
            modified_at,
            coerce_parameters: None,
            include_parameters: None,
            include_secrets: None,
            include_templates: None,
            dry_run: None,
            force: None,
            local: None,
            projects,
            tags,
            region: region.map(Box::new),
            service: service.map(Box::new),
            resource,
        }
    }
}