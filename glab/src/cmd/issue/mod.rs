use clap::SubCommand;

use super::Command;

pub struct Issue;

impl Command for Issue {
    fn info<'a, 'b>() -> clap::App<'a, 'b> {
        SubCommand::with_name("issue")
            .about("issue command")
            .subcommand(SubCommand::with_name("checkout"))
    }

    fn execute(_matches: &clap::ArgMatches<'_>) {
        todo!()
    }
}