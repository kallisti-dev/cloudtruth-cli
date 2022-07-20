/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your configuration parameters and secrets making them easier to manage and use as a team.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

/// BackupParameterValue : Parameter value data at a point in time.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BackupParameterValue {
    #[serde(rename = "external")]
    pub external: Option<Box<crate::models::BackupExternalReference>>,
    #[serde(rename = "environment")]
    pub environment: String,
    #[serde(rename = "evaluated")]
    pub evaluated: bool,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "raw", skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
}

impl BackupParameterValue {
    /// Parameter value data at a point in time.
    pub fn new(
        external: Option<crate::models::BackupExternalReference>,
        environment: String,
        evaluated: bool,
    ) -> BackupParameterValue {
        BackupParameterValue {
            external: external.map(Box::new),
            environment,
            evaluated,
            source: None,
            project: None,
            value: None,
            raw: None,
        }
    }
}
