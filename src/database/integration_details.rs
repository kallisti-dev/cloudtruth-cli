use cloudtruth_restapi::models::{AwsIntegration, GitHubIntegration};

#[derive(Debug)]
pub struct IntegrationDetails {
    pub id: String,
    pub name: String,
    pub description: String,
    pub provider: String,
    pub fqn: String,
    pub status: String,
    pub status_detail: String,
    pub status_time: String,
    pub created_at: String,
    pub modified_at: String,
}

impl From<&AwsIntegration> for IntegrationDetails {
    fn from(aws: &AwsIntegration) -> Self {
        IntegrationDetails {
            id: aws.id.clone(),
            provider: "aws".to_string(),
            name: aws.name.clone(),
            description: aws.description.clone().unwrap_or_default(),
            fqn: aws.fqn.clone(),
            status: aws.status.clone(),
            status_detail: aws.status_detail.clone(),
            status_time: aws.status_last_checked_at.clone(),
            created_at: aws.created_at.clone(),
            modified_at: aws.modified_at.clone(),
        }
    }
}

impl From<&GitHubIntegration> for IntegrationDetails {
    fn from(github: &GitHubIntegration) -> Self {
        IntegrationDetails {
            id: github.id.clone(),
            provider: "github".to_string(),
            name: github.name.clone(),
            description: github.description.clone().unwrap_or_default(),
            fqn: github.fqn.clone(),
            status: github.status.clone(),
            status_detail: github.status_detail.clone(),
            status_time: github.status_last_checked_at.clone(),
            created_at: github.created_at.clone(),
            modified_at: github.modified_at.clone(),
        }
    }
}