use clap::{Arg, SubCommand};

use super::Command;

pub struct Repo;

impl Command for Repo {
    fn info<'a, 'b>() -> clap::App<'a, 'b> {
        SubCommand::with_name("repo")
            .about("clone command")
            .subcommand(SubCommand::with_name("clone").arg(Arg::with_name("repo")
                .required(true).takes_value(true))
            )
            .subcommand(SubCommand::with_name("search").arg(Arg::with_name("repo_name")
                .required(true).takes_value(true))
            )
    }

    fn execute(_matches: &clap::ArgMatches<'_>) {
        todo!()
    }
}