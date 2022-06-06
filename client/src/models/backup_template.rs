/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your configuration parameters and secrets making them easier to manage and use as a team.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

/// BackupTemplate : Template data at a given point in time.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BackupTemplate {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "description")]
    pub description: Option<String>,
}

impl BackupTemplate {
    /// Template data at a given point in time.
    pub fn new(name: String, text: String, description: Option<String>) -> BackupTemplate {
        BackupTemplate {
            name,
            text,
            description,
        }
    }
}