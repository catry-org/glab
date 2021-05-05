use super::Command;
use clap::{Arg, SubCommand};
use serde::{Deserialize, Serialize};
pub mod file;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    private_token: Option<String>,
    pub language: String,
    pub hostname: String,
}

impl Command for Config {
    fn info<'a, 'b>() -> clap::App<'a, 'b> {
        SubCommand::with_name("config")
            .about("config command")
            .subcommand(SubCmdList::info())
            .subcommand(SubCmdGet::info())
    }

    fn execute(matches: &clap::ArgMatches<'_>, config: Config) {
        if matches.is_present("list") {
            SubCmdList::execute(matches, config);
        } else if matches.is_present("get") {
            SubCmdGet::execute(matches, config);
        }
    }
}

struct SubCmdList;

impl Command for SubCmdList {
    fn info<'a, 'b>() -> clap::App<'a, 'b> {
        SubCommand::with_name("list")
    }

    fn execute(_matches: &clap::ArgMatches<'_>, config: Config) {
        println!(
            "hostname: {}\nlanguage: {}\nprivate_token: {}",
            config.hostname,
            config.language,
            config.private_token.unwrap_or("None".to_owned())
        );
    }
}

struct SubCmdGet;

impl Command for SubCmdGet {
    fn info<'a, 'b>() -> clap::App<'a, 'b> {
        SubCommand::with_name("get").arg(
            Arg::with_name("config_data")
                .takes_value(true)
                .required(true),
        )
    }

    fn execute(matches: &clap::ArgMatches<'_>, config: Config) {
        let get_command = matches.subcommand_matches("get");

        if get_command.is_some() {
            let value = get_command.unwrap().value_of("config_data");

            if value.is_some() {
                if value.unwrap() == "hostname" {
                    println!("{}", config.hostname);
                } else if value.unwrap() == "language" {
                    println!("{}", config.language);
                } else if value.unwrap() == "private_token" {
                    println!("{}", config.private_token.unwrap_or("None".to_owned()));
                }
            }
        }
    }
}
