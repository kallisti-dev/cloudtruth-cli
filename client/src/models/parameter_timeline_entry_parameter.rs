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
pub struct ParameterTimelineEntryParameter {
    /// A unique identifier for the parameter.
    #[serde(rename = "parameter_id")]
    pub parameter_id: Option<String>,
    /// The parameter name.
    #[serde(rename = "name")]
    pub name: String,
}

impl ParameterTimelineEntryParameter {
    pub fn new(parameter_id: Option<String>, name: String) -> ParameterTimelineEntryParameter {
        ParameterTimelineEntryParameter { parameter_id, name }
    }
}
