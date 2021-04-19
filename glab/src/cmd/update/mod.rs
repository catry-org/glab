use clap::SubCommand;

use super::Command;

pub struct Update;

impl Command for Update {
    fn info<'a, 'b>() -> clap::App<'a, 'b> {
        SubCommand::with_name("update")
            .about("update command")
    }

    fn execute(_matches: &clap::ArgMatches<'_>) {
        todo!()
    }
}