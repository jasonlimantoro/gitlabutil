pub mod merge_request {
    pub mod module {
        use crate::merge_request::gitlab_manager::Manager;
        use clap::ArgMatches;
        use std::error::Error;

        #[derive(Debug)]
        pub struct Args {
            repository: String,
            source_branch: String,
            target_branches: Vec<String>,
            title: String,
            description: String,
        }

        pub struct Module {
            manager: Manager,
        }

        impl Args {
            pub fn parse(args: &ArgMatches) -> Args {
                Args {
                    repository: args.value_of("repository").unwrap().to_string(),
                    source_branch: args.value_of("source-branch").unwrap().to_string(),
                    target_branches: args
                        .value_of("target-branches")
                        .unwrap()
                        .split(",")
                        .map(str::to_string)
                        .collect(),

                    title: args.value_of("title").unwrap().to_string(),
                    description: args.value_of("description").unwrap().to_string(),
                }
            }
        }

        impl Module {
            pub fn new(manager: Manager) -> Module {
                Module { manager }
            }
            pub fn create(self, args: &Args) -> Result<(), Box<dyn Error>> {
                for target_branch in &args.target_branches {
                    let result = self
                        .manager
                        .clone()
                        .create(
                            args.repository.clone(),
                            args.source_branch.clone(),
                            target_branch.to_string(),
                            args.title.clone(),
                            args.description.clone(),
                        )
                        .unwrap();

                    println!("result {:?}", result);
                }
                Ok(())
            }
        }
    }

    pub mod gitlab_manager {
        use crate::merge_request::gitlab_accessor::Accessor;
        use std::error::Error;

        #[derive(Debug)]
        pub struct MergeRequest {
            pub id: i64,
            pub link: String,
        }

        #[derive(Clone)]
        pub struct Manager {
            accessor: Accessor,
        }

        impl Manager {
            pub fn new(accessor: Accessor) -> Manager {
                Manager { accessor }
            }
            pub fn create(
                self,
                repository: String,
                source_branch: String,
                target_branch: String,
                title: String,
                description: String,
            ) -> Result<MergeRequest, Box<dyn Error>> {
                let result = self
                    .accessor
                    .create(repository, source_branch, target_branch, title, description)
                    .unwrap();

                Ok(MergeRequest {
                    id: result.id,
                    link: result.web_url,
                })
            }
        }
    }

    pub mod gitlab_accessor {
        use reqwest;
        use serde::{Deserialize, Serialize};
        use std::env;
        use std::error::Error;
        use std::ops::Deref;
        use urlencoding::encode;

        #[derive(Deserialize, Debug)]
        pub struct Project {
            pub id: i64,
            pub description: String,
            pub name: String,
            pub name_with_namespace: String,
            pub path: String,
            pub path_with_namespace: String,
            pub created_at: String,
            pub default_branch: String,
            pub ssh_url_to_repo: String,
            pub http_url_to_repo: String,
            pub web_url: String,
            pub readme_url: String,
            pub forks_count: i64,
            pub star_count: i64,
            pub last_activity_at: String,
            pub container_registry_image_prefix: String,
            pub packages_enabled: bool,
            pub empty_repo: bool,
            pub archived: bool,
            pub visibility: String,
            pub resolve_outdated_diff_discussions: bool,
            pub issues_enabled: bool,
            pub merge_requests_enabled: bool,
        }

        #[derive(Deserialize, Debug)]
        pub struct MergeRequest {
            pub id: i64,
            pub title: String,
            pub target_branch: String,
            pub source_branch: String,
            pub web_url: String,
        }

        #[derive(Deserialize, Serialize, Debug)]
        pub struct CreateMergeRequestRequest {
            pub id: i64,
            pub source_branch: String,
            pub target_branch: String,
            pub title: String,
            pub description: String,
        }

        const DOMAIN: &str = "https://gitlab.com";
        const BASE_PATH: &str = "/api/v4";

        const GITLAB_PRIVATE_TOKEN: &'static str = env!("GITLAB_PRIVATE_TOKEN");

        fn route_get_projects_by_path(path: String) -> String {
            let path = format!("/projects/{}", encode(&path.deref()));

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
            pub fn create(
                self,
                repository: String,
                source_branch: String,
                target_branch: String,
                title: String,
                description: String,
            ) -> Result<MergeRequest, Box<dyn Error>> {
                let project = self
                    .http_client
                    .c
                    .get(route_get_projects_by_path(repository))
                    .send()?
                    .json::<Project>()?;

                let request = CreateMergeRequestRequest {
                    id: project.id,
                    source_branch,
                    target_branch: target_branch.clone(),
                    title: format!(
                        "[{target_branch}]{title}",
                        target_branch = target_branch.clone(),
                        title = title
                    ),
                    description,
                };

                let merge_request = self
                    .http_client
                    .c
                    .post(route_create_merge_request(project.id))
                    .json(&request)
                    .send()?
                    .json::<MergeRequest>()?;

                Ok(merge_request)
            }
        }
    }
}
