use clap::SubCommand;

use super::{config::Config, Command};

pub struct Wiki;

impl Command for Wiki {
    fn info<'a, 'b>() -> clap::App<'a, 'b> {
        SubCommand::with_name("wiki").about("wiki command")
    }

    fn execute(_matches: &clap::ArgMatches<'_>, _: Config) {
        todo!()
    }
}
