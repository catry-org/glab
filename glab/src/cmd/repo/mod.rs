use clap::{Arg, SubCommand};

use super::{config::Config, Command};

pub struct Repo;

impl Command for Repo {
    fn info<'a, 'b>() -> clap::App<'a, 'b> {
        SubCommand::with_name("repo")
            .about("clone command")
            .subcommand(SubCmdClone::info())
            .subcommand(SubCmdSearch::info())
    }

    fn execute(_matches: &clap::ArgMatches<'_>, _: Config) {
        todo!()
    }
}

struct SubCmdClone;

impl Command for SubCmdClone {
    fn info<'a, 'b>() -> clap::App<'a, 'b> {
        SubCommand::with_name("clone").arg(Arg::with_name("repo").required(true).takes_value(true))
    }

    fn execute(_matches: &clap::ArgMatches<'_>, _: Config) {
        todo!()
    }
}

struct SubCmdSearch;

impl Command for SubCmdSearch {
    fn info<'a, 'b>() -> clap::App<'a, 'b> {
        SubCommand::with_name("search")
            .arg(Arg::with_name("repo_name").required(true).takes_value(true))
    }

    fn execute(_matches: &clap::ArgMatches<'_>, _: Config) {
        todo!()
    }
}
