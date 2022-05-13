use std::error::Error;

use clap::ArgMatches;

use crate::manager::gitlab;

#[derive(Debug)]
pub struct Args {
    repository: String,
    source_branch: String,
    target_branches: Vec<String>,
    title: String,
    description: String,
    jira_ticket_ids: Vec<String>,
}

pub struct Module {
    manager: gitlab::Manager,
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
            description: args.value_of("description").unwrap_or_default().to_string(),
            jira_ticket_ids: args
                .value_of("jira")
                .unwrap()
                .split(",")
                .map(str::to_string)
                .collect(),
        }
    }
}

impl Module {
    pub fn new(manager: gitlab::Manager) -> Module {
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
                    args.jira_ticket_ids.clone(),
                )
                .unwrap();

            println!("result {:?}", result);
        }
        Ok(())
    }
}
