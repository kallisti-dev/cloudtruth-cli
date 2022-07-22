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

/// struct for typed errors of method [`utils_generate_password_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UtilsGeneratePasswordCreateError {
    UnknownValue(serde_json::Value),
}

/// Endpoint for accessing utility functions
pub fn utils_generate_password_create(
    configuration: &configuration::Configuration,
    length: i32,
    require_hardware_generation: Option<bool>,
    require_lowercase: Option<bool>,
    require_numbers: Option<bool>,
    require_spaces: Option<bool>,
    require_symbols: Option<bool>,
    require_uppercase: Option<bool>,
) -> Result<crate::models::GeneratedPasswordResponse, Error<UtilsGeneratePasswordCreateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/utils/generate_password/",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("length", &length.to_string())]);
    if let Some(ref local_var_str) = require_hardware_generation {
        local_var_req_builder = local_var_req_builder
            .query(&[("require_hardware_generation", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = require_lowercase {
        local_var_req_builder =
            local_var_req_builder.query(&[("require_lowercase", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = require_numbers {
        local_var_req_builder =
            local_var_req_builder.query(&[("require_numbers", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = require_spaces {
        local_var_req_builder =
            local_var_req_builder.query(&[("require_spaces", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = require_symbols {
        local_var_req_builder =
            local_var_req_builder.query(&[("require_symbols", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = require_uppercase {
        local_var_req_builder =
            local_var_req_builder.query(&[("require_uppercase", &local_var_str.to_string())]);
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
        let local_var_entity: Option<UtilsGeneratePasswordCreateError> =
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
