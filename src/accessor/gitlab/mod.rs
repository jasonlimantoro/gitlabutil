use std::env;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use reqwest;
use reqwest::StatusCode;
use urlencoding::encode;

mod models;

#[derive(Debug)]
pub struct ApiError {
    url: String,
    method: String,
    status_code: u16,
    message: String,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ApiError{{ url: {}, method: {} code: {}, message: {} }}",
            self.url, self.method, self.status_code, self.message
        )
    }
}

impl Error for ApiError {}

const DOMAIN: &str = "https://gitlab.com";
const BASE_PATH: &str = "/api/v4";

const GITLAB_PRIVATE_TOKEN: &'static str = env!("GITLAB_PRIVATE_TOKEN");

fn route_get_projects_by_path(path: String) -> String {
    let path = format!("/projects/{}", encode(path.as_str()));

    return format!(
        "{domain}{base_path}{path}",
        domain = DOMAIN,
        base_path = BASE_PATH,
        path = path
    );
}

fn route_create_merge_request(project_id: i64) -> String {
    let path = format!("/projects/{}/merge_requests", project_id);

    return format!(
        "{domain}{base_path}{path}",
        domain = DOMAIN,
        base_path = BASE_PATH,
        path = path
    );
}

#[derive(Clone)]
pub struct Accessor {
    http_client: HttpClient,
}

#[derive(Clone)]
pub struct HttpClient {
    c: reqwest::blocking::Client,
}

impl HttpClient {
    pub fn new() -> HttpClient {
        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(
                format!("Bearer {}", GITLAB_PRIVATE_TOKEN).as_str(),
            )
            .unwrap(),
        );

        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        HttpClient { c: client }
    }
}

impl Accessor {
    pub fn new(http_client: HttpClient) -> Accessor {
        Accessor { http_client }
    }
    pub fn create_merge_request(
        self,
        repository: String,
        source_branch: String,
        target_branch: String,
        title: String,
        description: String,
        jira_ticket_ids: Vec<String>,
    ) -> Result<models::MergeRequest, Box<dyn Error>> {
        let get_projects_endpoint = route_get_projects_by_path(repository);
        let get_project_res = match self.http_client.c.get(get_projects_endpoint.clone()).send() {
            Ok(res) => res,
            Err(err) => {
                return Err(Box::new(ApiError {
                    url: get_projects_endpoint.clone(),
                    message: err.to_string(),
                    status_code: 0,
                    method: "get".to_string(),
                }));
            }
        };

        let status_code = get_project_res.status();
        let project = match status_code {
            StatusCode::OK => match get_project_res.json::<models::Project>() {
                Ok(res) => res,
                Err(err) => {
                    return Err(Box::new(ApiError {
                        url: get_projects_endpoint.clone(),
                        message: format!("unmarshalling: {}", err.to_string()),
                        status_code: err.status().unwrap().as_u16(),
                        method: "get".to_string(),
                    }));
                }
            },

            _ => {
                return Err(Box::new(ApiError {
                    url: get_projects_endpoint.clone(),
                    message: get_project_res.text().unwrap(),
                    status_code: status_code.as_u16(),
                    method: "get".to_string(),
                }));
            }
        };

        let request = models::CreateMergeRequestRequest {
            id: project.id,
            source_branch,
            target_branch: target_branch.clone(),
            title: create_title(title.as_str(), target_branch.as_str(), jira_ticket_ids),
            description,
        };

        let create_merge_request_endpoint = route_create_merge_request(project.id);
        let response = self
            .http_client
            .c
            .post(create_merge_request_endpoint.clone())
            .json(&request)
            .send();

        let result = match response {
            Ok(res) => res,
            Err(err) => {
                return Err(Box::new(ApiError {
                    url: create_merge_request_endpoint.clone(),
                    message: err.to_string(),
                    status_code: 0,
                    method: "post".to_string(),
                }));
            }
        };

        let status_code = result.status();

        let merge_request = match status_code {
            StatusCode::OK => match result.json::<models::MergeRequest>() {
                Ok(res) => res,
                Err(err) => {
                    return Err(Box::new(ApiError {
                        url: create_merge_request_endpoint,
                        method: "post".to_string(),
                        status_code: status_code.as_u16(),
                        message: err.to_string(),
                    }))
                }
            },

            _ => {
                return Err(Box::new(ApiError {
                    url: create_merge_request_endpoint,
                    method: "post".to_string(),
                    status_code: result.status().as_u16(),
                    message: result.text().unwrap(),
                }));
            }
        };

        Ok(merge_request)
    }
}

fn create_title(plain_title: &str, target_branch: &str, jira_ticket_ids: Vec<String>) -> String {
    let jira_sections: Vec<String> = jira_ticket_ids
        .into_iter()
        .map(|j| -> String { format!("[{}]", j) })
        .collect();

    return format!(
        "{jira}{target_branch} {title}",
        jira = jira_sections.concat(),
        target_branch = format!("[{}]", target_branch),
        title = plain_title
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_correctly_create_title() {
        assert_eq!(
            create_title(
                "test MR",
                "uat",
                vec!["ES-123", "ES-234"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
            ),
            "[ES-123][ES-234][uat] test MR"
        );
    }
}
