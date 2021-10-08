/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your configuration parameters and secrets making them easier to manage and use as a team.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

/// AwsPushTask : Push task for an AWS integration.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsPushTask {
    #[serde(rename = "url")]
    pub url: String,
    /// The unique identifier for the push action task.
    #[serde(rename = "id")]
    pub id: String,
    /// The reason why this task was triggered.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// The current state of this task.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<Box<crate::models::StateEnum>>,
    /// An error code, if not successful.  This usually indicates an error occurred very early during processing, before attempting to push.
    #[serde(rename = "error_code", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// Details on the error from the integration.  This usually indicates an error occurred very early during processing, before attempting to push.
    #[serde(rename = "error_detail", skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "modified_at")]
    pub modified_at: String,
}

impl AwsPushTask {
    /// Push task for an AWS integration.
    pub fn new(url: String, id: String, created_at: String, modified_at: String) -> AwsPushTask {
        AwsPushTask {
            url,
            id,
            reason: None,
            state: None,
            error_code: None,
            error_detail: None,
            created_at,
            modified_at,
        }
    }
}
