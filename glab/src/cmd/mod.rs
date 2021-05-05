use crate::utils::{colors, localization};
use crate::BANNER;
use auth::Auth;
use clap::{App, ArgMatches};
use config::Config;
use issue::Issue;
use pr::PR;
use repo::Repo;
use wiki::Wiki;

pub mod auth;
pub mod config;
pub mod issue;
pub mod pr;
pub mod repo;
pub mod wiki;

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
    let config_file = config::file::read_config_file("config.yaml");
    let config: Config = serde_yaml::from_str(&config_file).unwrap();

    // SubCommands
    if matches.is_present("config") {
        Config::execute(&matches.subcommand_matches("config").unwrap(), config);
    } else if matches.is_present("auth") {
        Auth::execute(&matches.subcommand_matches("auth").unwrap(), config);
    } else if matches.is_present("repo") {
        Repo::execute(&matches.subcommand_matches("repo").unwrap(), config);
    } else if matches.is_present("pr") {
        PR::execute(&matches.subcommand_matches("pr").unwrap(), config);
    } else if matches.is_present("wiki") {
        Wiki::execute(&matches.subcommand_matches("wiki").unwrap(), config);
    } else {
        println!("{}", BANNER);
        println!(
            "{}",
            colors::Colors::from(
                &localization::get_text(&config.language, "emptyCommand"),
                colors::Colors::BrightRed
            )
        );
    }
}

trait Command {
    fn info<'a, 'b>() -> App<'a, 'b>;
    fn execute(matches: &ArgMatches<'_>, config: Config);
}
