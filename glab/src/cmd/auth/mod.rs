use clap::SubCommand;

use super::Command;

pub struct Auth;

impl Command for Auth {
    fn info<'a, 'b>() -> clap::App<'a, 'b> {
        SubCommand::with_name("auth")
            .about("auth command")
            .subcommand(SubCommand::with_name("login"))
            .subcommand(SubCommand::with_name("logout"))
            .subcommand(SubCommand::with_name("check"))
    }

    fn execute(_matches: &clap::ArgMatches<'_>) {
        todo!()
    }
}