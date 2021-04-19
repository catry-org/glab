use clap::SubCommand;

use super::Command;

pub struct PR;

impl Command for PR {
    fn info<'a, 'b>() -> clap::App<'a, 'b> {
        SubCommand::with_name("pr")
            .about("Pull Request command")
            .subcommand(SubCommand::with_name("checkout"))
    }

    fn execute(_matches: &clap::ArgMatches<'_>) {
        todo!()
    }
}