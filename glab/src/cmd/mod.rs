use clap::{App, ArgMatches};
use auth::Auth;
use config::Config;
use issue::Issue;
use pr::PR;
use repo::Repo;
use update::Update;
use wiki::Wiki;
use crate::BANNER;
use crate::utils::{localization, colors};
pub mod config;
pub mod update;
pub mod repo;
pub mod wiki;
pub mod issue;
pub mod auth;
pub mod pr;

// Commands Init
pub fn commands<'a, 'b>() -> Vec<App<'a, 'b>> {
    vec![
            Config::info(),
            Update::info(),
            Auth::info(),
            Repo::info(),
            Issue::info(),
            PR::info(),
            Wiki::info(),
        ]
}

/// Commands Handler
pub fn cmd_matches(matches: ArgMatches<'_>) {
    // SubCommands
    if matches.is_present("config") {
        Config::execute(&matches);
    } else if matches.is_present("update") {
        Update::execute(&matches);
    } else if matches.is_present("auth") {
        Auth::execute(&matches);
    } else if matches.is_present("repo") {
        Repo::execute(&matches);
    } else if matches.is_present("pr") {
        PR::execute(&matches);
    } else if matches.is_present("wiki") {
        Wiki::execute(&matches);
    } else {
        println!("{}", BANNER);
        println!("{}", colors::Colors::from(&localization::get_text("en_US", "emptyCommand"), colors::Colors::BrightRed));
    }
}

trait Command {
    fn info<'a, 'b>() -> App<'a, 'b>;
    fn execute(matches: &ArgMatches<'_>);
}