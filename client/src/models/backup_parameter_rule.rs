/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your configuration parameters and secrets making them easier to manage and use as a team.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

/// BackupParameterRule : Rule that is applied to a parameter or parameter-type at a point in time.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BackupParameterRule {
    #[serde(rename = "rule_type")]
    pub rule_type: String,
    #[serde(rename = "constraint")]
    pub constraint: String,
}

impl BackupParameterRule {
    /// Rule that is applied to a parameter or parameter-type at a point in time.
    pub fn new(rule_type: String, constraint: String) -> BackupParameterRule {
        BackupParameterRule {
            rule_type,
            constraint,
        }
    }
}
