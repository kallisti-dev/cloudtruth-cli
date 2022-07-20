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
pub struct Invitation {
    #[serde(rename = "url")]
    pub url: String,
    /// The unique identifier of an invitation.
    #[serde(rename = "id")]
    pub id: String,
    /// The email address of the user to be invited.
    #[serde(rename = "email")]
    pub email: String,
    /// The role that the user will have in the organization, should the user accept.
    #[serde(rename = "role")]
    pub role: Option<Box<crate::models::RoleEnum>>,
    /// The user that created the invitation.
    #[serde(rename = "inviter")]
    pub inviter: String,
    /// The name of the user that created the invitation.
    #[serde(rename = "inviter_name")]
    pub inviter_name: String,
    /// The current state of the invitation.
    #[serde(rename = "state")]
    pub state: String,
    /// Additional details about the state of the invitation.
    #[serde(rename = "state_detail")]
    pub state_detail: String,
    /// The resulting membership, should the user accept.
    #[serde(rename = "membership")]
    pub membership: Option<String>,
    /// The organization that the user will become a member of, should the user accept.
    #[serde(rename = "organization")]
    pub organization: String,
}

impl Invitation {
    pub fn new(
        url: String,
        id: String,
        email: String,
        role: Option<crate::models::RoleEnum>,
        inviter: String,
        inviter_name: String,
        state: String,
        state_detail: String,
        membership: Option<String>,
        organization: String,
    ) -> Invitation {
        Invitation {
            url,
            id,
            email,
            role: role.map(Box::new),
            inviter,
            inviter_name,
            state,
            state_detail,
            membership,
            organization,
        }
    }
}
