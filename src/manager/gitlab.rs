use std::error::Error;

use crate::accessor::gitlab;

#[derive(Debug)]
pub struct MergeRequest {
    pub id: i64,
    pub link: String,
}

#[derive(Clone)]
pub struct Manager {
    accessor: gitlab::Accessor,
}

impl Manager {
    pub fn new(accessor: gitlab::Accessor) -> Manager {
        Manager { accessor }
    }
    pub fn create(
        self,
        repository: String,
        source_branch: String,
        target_branch: String,
        title: String,
        description: String,
        jira_ticket_ids: Vec<String>,
    ) -> Result<MergeRequest, Box<dyn Error>> {
        let result = self
            .accessor
            .create(
                repository,
                source_branch,
                target_branch,
                title,
                description,
                jira_ticket_ids,
            )
            .unwrap();

        Ok(MergeRequest {
            id: result.id,
            link: result.web_url,
        })
    }
}
