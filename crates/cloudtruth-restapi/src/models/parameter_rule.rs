/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your configuration parameters and secrets making them easier to manage and use as a team.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

/// ParameterRule : A type of `ModelSerializer` that uses hyperlinked relationships with compound keys instead of primary key relationships.  Specifically:  * A 'url' field is included instead of the 'id' field. * Relationships to other instances are hyperlinks, instead of primary keys.  NOTE: this only works with DRF 3.1.0 and above.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ParameterRule {
    /// The URL for the parameter rule.
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "ledger_id")]
    pub ledger_id: String,
    /// The parameter this rule is for.
    #[serde(rename = "parameter")]
    pub parameter: String,
    #[serde(rename = "type")]
    pub _type: crate::models::ParameterRuleTypeEnum,
    #[serde(rename = "constraint")]
    pub constraint: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "modified_at")]
    pub modified_at: Option<String>,
}

impl ParameterRule {
    /// A type of `ModelSerializer` that uses hyperlinked relationships with compound keys instead of primary key relationships.  Specifically:  * A 'url' field is included instead of the 'id' field. * Relationships to other instances are hyperlinks, instead of primary keys.  NOTE: this only works with DRF 3.1.0 and above.
    pub fn new(
        url: String,
        id: String,
        ledger_id: String,
        parameter: String,
        _type: crate::models::ParameterRuleTypeEnum,
        constraint: String,
        created_at: String,
        modified_at: Option<String>,
    ) -> ParameterRule {
        ParameterRule {
            url,
            id,
            ledger_id,
            parameter,
            _type,
            constraint,
            created_at,
            modified_at,
        }
    }
}