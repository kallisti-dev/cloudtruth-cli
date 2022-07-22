/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your configuration parameters and secrets making them easier to manage and use as a team.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;
use std::time::Instant;

use super::{configuration, Error};
use crate::apis::{handle_serde_error, ResponseContent};

/// struct for typed errors of method [`audit_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuditListError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`audit_retrieve`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuditRetrieveError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`audit_summary_retrieve`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuditSummaryRetrieveError {
    UnknownValue(serde_json::Value),
}

/// A searchable log of all the actions taken by users and service accounts within the organization.
pub fn audit_list(
    configuration: &configuration::Configuration,
    action: Option<&str>,
    earliest: Option<String>,
    environment_id: Option<&str>,
    latest: Option<String>,
    object_id: Option<&str>,
    object_type: Option<&str>,
    ordering: Option<&str>,
    page: Option<i32>,
    page_size: Option<i32>,
    parameter_id: Option<&str>,
    project_id: Option<&str>,
    user_id: Option<&str>,
) -> Result<crate::models::PaginatedAuditTrailList, Error<AuditListError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/audit/", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = action {
        local_var_req_builder =
            local_var_req_builder.query(&[("action", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = earliest {
        local_var_req_builder =
            local_var_req_builder.query(&[("earliest", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = environment_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("environment_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = latest {
        local_var_req_builder =
            local_var_req_builder.query(&[("latest", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = object_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("object_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = object_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("object_type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = ordering {
        local_var_req_builder =
            local_var_req_builder.query(&[("ordering", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder =
            local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("page_size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = parameter_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("parameter_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = project_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("project_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("user_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let method = local_var_req.method().clone();
    let start = Instant::now();
    let mut local_var_resp = local_var_client.execute(local_var_req)?;
    if local_var_configuration.rest_debug {
        let duration = start.elapsed();
        println!(
            "URL {} {} elapsed: {:?}",
            method,
            &local_var_resp.url(),
            duration
        );
    }

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        if local_var_configuration.debug_success(super::function!()) {
            println!("RESP {} {}", &local_var_status, &local_var_content);
        }

        serde_json::from_str(&local_var_content)
            .map_err(|e| handle_serde_error(e, &method, local_var_resp.url(), &local_var_content))
    } else {
        let local_var_entity: Option<AuditListError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        if local_var_configuration.rest_debug {
            println!(
                "RESP {} {}",
                &local_var_error.status, &local_var_error.content
            );
        }
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve one record from the audit log.
pub fn audit_retrieve(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<crate::models::AuditTrail, Error<AuditRetrieveError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/audit/{id}/",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let method = local_var_req.method().clone();
    let start = Instant::now();
    let mut local_var_resp = local_var_client.execute(local_var_req)?;
    if local_var_configuration.rest_debug {
        let duration = start.elapsed();
        println!(
            "URL {} {} elapsed: {:?}",
            method,
            &local_var_resp.url(),
            duration
        );
    }

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        if local_var_configuration.debug_success(super::function!()) {
            println!("RESP {} {}", &local_var_status, &local_var_content);
        }

        serde_json::from_str(&local_var_content)
            .map_err(|e| handle_serde_error(e, &method, local_var_resp.url(), &local_var_content))
    } else {
        let local_var_entity: Option<AuditRetrieveError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        if local_var_configuration.rest_debug {
            println!(
                "RESP {} {}",
                &local_var_error.status, &local_var_error.content
            );
        }
        Err(Error::ResponseError(local_var_error))
    }
}

/// Summary information about the organization's audit trail.
pub fn audit_summary_retrieve(
    configuration: &configuration::Configuration,
) -> Result<crate::models::AuditTrailSummary, Error<AuditSummaryRetrieveError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/audit/summary/",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let method = local_var_req.method().clone();
    let start = Instant::now();
    let mut local_var_resp = local_var_client.execute(local_var_req)?;
    if local_var_configuration.rest_debug {
        let duration = start.elapsed();
        println!(
            "URL {} {} elapsed: {:?}",
            method,
            &local_var_resp.url(),
            duration
        );
    }

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        if local_var_configuration.debug_success(super::function!()) {
            println!("RESP {} {}", &local_var_status, &local_var_content);
        }

        serde_json::from_str(&local_var_content)
            .map_err(|e| handle_serde_error(e, &method, local_var_resp.url(), &local_var_content))
    } else {
        let local_var_entity: Option<AuditSummaryRetrieveError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        if local_var_configuration.rest_debug {
            println!(
                "RESP {} {}",
                &local_var_error.status, &local_var_error.content
            );
        }
        Err(Error::ResponseError(local_var_error))
    }
}
