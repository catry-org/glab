use clap::{Arg, SubCommand};
use serde::{Serialize, Deserialize};
use super::Command;
pub mod file;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    private_token: Option<String>,
    pub lang: String,
    pub hostname: String
}

impl Command for Config {
    fn info<'a, 'b>() -> clap::App<'a, 'b> {
        SubCommand::with_name("config")
            .about("config command")
            .subcommand(SubCommand::with_name("list"))
            .subcommand(SubCommand::with_name("get").arg(
                Arg::with_name("config_data").takes_value(true)
            )
        )
    }

    fn execute(matches: &clap::ArgMatches<'_>) {
        let config = file::read_config_file(None);
        let yaml: Config = serde_yaml::from_str(&config).unwrap();

        if matches.is_present("list") {
           println!("hostname: {}\nlang: {}\nprivate_token: {}",
                yaml.hostname,
                yaml.lang,
                yaml.private_token.unwrap_or("None".to_owned())
            );
        } else if matches.is_present("get") {
            let get_command = matches.subcommand_matches("get");
            if get_command.is_some() {
                let value = get_command.unwrap().value_of("config_data");

                if value.is_some() {
                    if value.unwrap() == "hostname" {
                        println!("{}", yaml.hostname);
                    } else if value.unwrap() == "lang" {
                        println!("{}", yaml.lang);
                    } else if value.unwrap() == "private_token" {
                        println!("{}", yaml.private_token.unwrap_or("None".to_owned()));
                    }
                }

            }
        }
    }
}