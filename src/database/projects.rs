use crate::database::{
    auth_details, response_message, OpenApiConfig, ProjectDetails, ProjectError, PAGE_SIZE,
};

use cloudtruth_restapi::apis::projects_api::*;
use cloudtruth_restapi::apis::Error::ResponseError;
use cloudtruth_restapi::models::{PatchedProject, ProjectCreate};
use std::collections::HashMap;
use std::result::Result;

const NO_DESC_CONTAINS: Option<&str> = None;
const NO_NAME_CONTAINS: Option<&str> = None;
const NO_ORDERING: Option<&str> = None;

pub struct Projects {}

/// This is used to map from an Project's URL to the Name.
pub type ProjectUrlMap = HashMap<String, String>;

fn response_error(status: &reqwest::StatusCode, content: &str) -> ProjectError {
    ProjectError::ResponseError(response_message(status, content))
}

fn auth_error(content: &str) -> ProjectError {
    ProjectError::Authentication(auth_details(content))
}

impl Projects {
    pub fn new() -> Self {
        Self {}
    }

    /// This provides a means to get an entire list of project URLs to names.
    pub fn get_url_name_map(&self, rest_cfg: &OpenApiConfig) -> ProjectUrlMap {
        let response = projects_list(
            rest_cfg,
            NO_DESC_CONTAINS,
            None,
            NO_NAME_CONTAINS,
            NO_ORDERING,
            None,
            PAGE_SIZE,
        );
        let mut result: ProjectUrlMap = ProjectUrlMap::new();
        if let Ok(list) = response {
            if let Some(projects) = list.results {
                for prj in projects {
                    result.insert(prj.url, prj.name);
                }
            }
        }
        result
    }

    /// Use the project URL to get the corresponding name.
    pub fn get_name_from_url(&self, rest_cfg: &OpenApiConfig, url: &str) -> String {
        let id = url
            .split('/')
            .filter(|&x| !x.is_empty())
            .last()
            .unwrap_or_default();
        if id.is_empty() {
            "".to_owned()
        } else {
            let response = projects_retrieve(rest_cfg, id);
            if let Ok(project) = response {
                project.name
            } else {
                "".to_owned()
            }
        }
    }

    /// Get the details for `proj_name`
    pub fn get_details_by_name(
        &self,
        rest_cfg: &OpenApiConfig,
        proj_name: &str,
        resolve_parent: bool,
    ) -> Result<Option<ProjectDetails>, ProjectError> {
        let response = projects_list(
            rest_cfg,
            NO_DESC_CONTAINS,
            Some(proj_name),
            NO_NAME_CONTAINS,
            NO_ORDERING,
            None,
            PAGE_SIZE,
        );

        match response {
            Ok(data) => match data.results {
                Some(list) => {
                    if list.is_empty() {
                        Ok(None)
                    } else {
                        // TODO: handle more than one??
                        let proj = &list[0];
                        let mut details = ProjectDetails::from(proj);
                        if resolve_parent {
                            details.parent_name =
                                self.get_name_from_url(rest_cfg, &details.parent_url);
                        }
                        Ok(Some(details))
                    }
                }
                _ => Ok(None),
            },
            Err(ResponseError(ref content)) => match content.status.as_u16() {
                401 => Err(auth_error(&content.content)),
                403 => Err(auth_error(&content.content)),
                _ => Err(response_error(&content.status, &content.content)),
            },
            Err(e) => Err(ProjectError::UnhandledError(e.to_string())),
        }
    }

    /// Resolve the `proj_name` to a String
    pub fn get_id(
        &self,
        rest_cfg: &OpenApiConfig,
        proj_name: &str,
    ) -> Result<Option<String>, ProjectError> {
        if let Some(details) = self.get_details_by_name(rest_cfg, proj_name, false)? {
            Ok(Some(details.id))
        } else {
            Ok(None)
        }
    }

    /// Get a complete list of projects for this organization.
    pub fn get_project_details(
        &self,
        rest_cfg: &OpenApiConfig,
    ) -> Result<Vec<ProjectDetails>, ProjectError> {
        let response = projects_list(
            rest_cfg,
            NO_DESC_CONTAINS,
            None,
            NO_NAME_CONTAINS,
            NO_ORDERING,
            None,
            PAGE_SIZE,
        );

        match response {
            Ok(data) => match data.results {
                Some(list) => {
                    let mut projects: Vec<ProjectDetails> = vec![];

                    // create the list of ProjectDetails and a map of url to name
                    let mut url_map: ProjectUrlMap = ProjectUrlMap::new();
                    for api_prj in list {
                        let details = ProjectDetails::from(&api_prj);
                        url_map.insert(details.url.clone(), details.name.clone());
                        projects.push(details);
                    }

                    // populate the parent names
                    for prj in &mut projects {
                        if !prj.parent_url.is_empty() {
                            prj.parent_name = url_map.get(&prj.parent_url).unwrap().clone();
                        }
                    }
                    projects.sort_by(|l, r| l.name.cmp(&r.name));
                    Ok(projects)
                }
                None => Ok(vec![]),
            },
            Err(ResponseError(ref content)) => match content.status.as_u16() {
                401 => Err(auth_error(&content.content)),
                403 => Err(auth_error(&content.content)),
                _ => Err(response_error(&content.status, &content.content)),
            },
            Err(e) => Err(ProjectError::UnhandledError(e.to_string())),
        }
    }

    /// Create a project with the specified name/description
    pub fn create_project(
        &self,
        rest_cfg: &OpenApiConfig,
        proj_name: &str,
        description: Option<&str>,
        parent_url: Option<&str>,
    ) -> Result<Option<String>, ProjectError> {
        let proj = ProjectCreate {
            name: proj_name.to_string(),
            description: description.map(String::from),
            depends_on: parent_url.map(String::from),
        };
        let response = projects_create(rest_cfg, proj);
        match response {
            // return the project id of the newly minted project
            Ok(project) => Ok(Some(project.id)),
            Err(ResponseError(ref content)) => {
                Err(response_error(&content.status, &content.content))
            }
            Err(e) => Err(ProjectError::UnhandledError(e.to_string())),
        }
    }

    /// Delete the specified project
    pub fn delete_project(
        &self,
        rest_cfg: &OpenApiConfig,
        project_id: &str,
    ) -> Result<Option<String>, ProjectError> {
        let response = projects_destroy(rest_cfg, project_id);
        match response {
            Ok(_) => Ok(Some(project_id.to_string())),
            Err(ResponseError(ref content)) => {
                Err(response_error(&content.status, &content.content))
            }
            Err(e) => Err(ProjectError::UnhandledError(e.to_string())),
        }
    }

    /// Update the specified project
    pub fn update_project(
        &self,
        rest_cfg: &OpenApiConfig,
        project_name: &str,
        project_id: &str,
        description: Option<&str>,
        parent_url: Option<&str>,
    ) -> Result<Option<String>, ProjectError> {
        let proj = PatchedProject {
            url: None,
            id: None,
            name: Some(project_name.to_string()),
            description: description.map(|d| d.to_string()),
            created_at: None,
            modified_at: None,
            pushes: None,
            depends_on: parent_url.map(String::from),
            dependents: None,
        };
        let response = projects_partial_update(rest_cfg, project_id, Some(proj));
        match response {
            Ok(project) => Ok(Some(project.id)),
            Err(ResponseError(ref content)) => {
                Err(response_error(&content.status, &content.content))
            }
            Err(e) => Err(ProjectError::UnhandledError(e.to_string())),
        }
    }
}
