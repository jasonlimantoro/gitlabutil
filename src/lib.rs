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
                }
            }
        }

        impl Module {
            pub fn create(self, args: &Args) -> Result<(), Box<dyn Error>> {
                let result = self
                    .manager
                    .create(
                        args.repository.clone(),
                        args.source_branch.clone(),
                        args.target_branches.clone(),
                    )
                    .unwrap();

                println!("result {:?}", result);
                Ok(())
            }
        }
    }

    pub mod gitlab_manager {
        use crate::merge_request::gitlab_accessor::Accessor;
        use std::error::Error;

        #[derive(Debug)]
        pub struct MergeRequest {
            pub id: String,
            pub link: String,
        }

        pub struct Manager {
            accessor: Accessor,
        }

        impl Manager {
            pub fn create(
                self,
                repository: String,
                source_branch: String,
                target_branches: Vec<String>,
            ) -> Result<MergeRequest, Box<dyn Error>> {
                let result = self
                    .accessor
                    .create(repository, source_branch, target_branches)
                    .unwrap();

                Ok(MergeRequest {
                    id: result.id,
                    link: result.link,
                })
            }
        }
    }

    pub mod gitlab_accessor {
        use std::error::Error;

        pub struct MergeRequest {
            pub id: String,
            pub link: String,
        }

        pub struct Accessor {}

        impl Accessor {
            pub fn create(
                self,
                repository: String,
                source_branch: String,
                target_branches: Vec<String>,
            ) -> Result<MergeRequest, Box<dyn Error>> {
                Ok(MergeRequest {
                    id: "".to_string(),
                    link: "".to_string(),
                })
            }
        }
    }
}
