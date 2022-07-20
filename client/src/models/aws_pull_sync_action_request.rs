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
pub struct AwsPullSyncActionRequest {
    /// Allows you to set the dry_run flag on the pull action before triggering a sync.
    #[serde(rename = "dry_run", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl AwsPullSyncActionRequest {
    pub fn new() -> AwsPullSyncActionRequest {
        AwsPullSyncActionRequest { dry_run: None }
    }
}
