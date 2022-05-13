use serde::{Deserialize, Serialize};

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
