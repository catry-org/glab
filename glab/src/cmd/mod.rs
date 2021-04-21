use clap::{App, ArgMatches};
use auth::Auth;
use config::Config;
use issue::Issue;
use pr::PR;
use repo::Repo;
use wiki::Wiki;
use crate::BANNER;
use crate::utils::{localization, colors};

use self::config::file;
pub mod config;
pub mod repo;
pub mod wiki;
pub mod issue;
pub mod auth;
pub mod pr;

// Commands Init
pub fn commands<'a, 'b>() -> Vec<App<'a, 'b>> {
    vec![
            Config::info(),
            Auth::info(),
            Repo::info(),
            Issue::info(),
            PR::info(),
            Wiki::info(),
        ]
}

/// Commands Handler
pub fn cmd_matches(matches: ArgMatches<'_>) {
    let config_file = file::read_config_file("config.yaml");
    let config: Config = serde_yaml::from_str(&config_file).unwrap();

    // SubCommands
    if matches.is_present("config") {
        Config::execute(&matches.subcommand_matches("config").unwrap());
    } else if matches.is_present("auth") {
        Auth::execute(&matches.subcommand_matches("auth").unwrap());
    } else if matches.is_present("repo") {
        Repo::execute(&matches.subcommand_matches("repo").unwrap());
    } else if matches.is_present("pr") {
        PR::execute(&matches.subcommand_matches("pr").unwrap());
    } else if matches.is_present("wiki") {
        Wiki::execute(&matches.subcommand_matches("wiki").unwrap());
    } else {
        println!("{}", BANNER);
        println!("{}", colors::Colors::from(&localization::get_text(&config.lang, "emptyCommand"), colors::Colors::BrightRed));
    }
}

trait Command {
    fn info<'a, 'b>() -> App<'a, 'b>;
    fn execute(matches: &ArgMatches<'_>);
}