use clap::SubCommand;

use super::Command;

pub struct Repo;

impl Command for Repo {
    fn info<'a, 'b>() -> clap::App<'a, 'b> {
        SubCommand::with_name("repo")
            .about("clone command")
            .subcommand(SubCommand::with_name("clone"))
            .subcommand(SubCommand::with_name("search"))
    }

    fn execute(_matches: &clap::ArgMatches<'_>) {
        todo!()
    }
}