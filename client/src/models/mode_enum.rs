/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your configuration parameters and secrets making them easier to manage and use as a team.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ModeEnum {
    #[serde(rename = "mapped")]
    Mapped,
    #[serde(rename = "pattern")]
    Pattern,
    #[serde(rename = "unknown_default_open_api", other)]
    UnknownDefaultOpenApi,
}

impl ToString for ModeEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Mapped => String::from("mapped"),
            Self::Pattern => String::from("pattern"),
            Self::UnknownDefaultOpenApi => String::from("unknown_default_open_api"),
        }
    }
}

impl Default for ModeEnum {
    fn default() -> ModeEnum {
        Self::Mapped
    }
}
