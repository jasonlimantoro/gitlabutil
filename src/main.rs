use clap::{arg, Command};
use gitlabutil::merge_request::module as merge_request_module;

fn main() {
    let matches = Command::new("gitlab-util")
        .version("1.0")
        .about("Utilities for gitlab")
        .propagate_version(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("merge-request")
                .about("merge request related commands")
                .arg_required_else_help(true)
                .subcommand_required(true)
                .subcommand(
                    Command::new("create")
                        .about("create merge request")
                        .arg(arg!(-r --repository <REPOSITORY>))
                        .arg(arg!(-s --"source-branch" <SOURCE_BRANCH>))
                        .arg(arg!(-t --"target-branches" <TARGET_BRANCHES>)),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("merge-request", merge_request_matches)) => {
            match merge_request_matches.subcommand() {
                Some(("create", merge_request_create_matches)) => {
                    let args = merge_request_module::Args::parse(merge_request_create_matches);
                    match merge_request_module::create(&args) {
                        Ok(_) => {
                            println!("Done.")
                        }
                        Err(e) => {
                            panic!("error: {}", e)
                        }
                    }
                }
                _ => {}
            }
        }

        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
