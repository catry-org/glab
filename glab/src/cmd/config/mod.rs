use clap::SubCommand;
use super::Command;
pub mod file;

pub struct Config;

impl Command for Config {
    fn info<'a, 'b>() -> clap::App<'a, 'b> {
        SubCommand::with_name("config")
            .about("config command")
            .subcommand(SubCommand::with_name("list"))
            .subcommand(SubCommand::with_name("get"))
    }

    fn execute(_matches: &clap::ArgMatches<'_>) {
        todo!()
    }
}