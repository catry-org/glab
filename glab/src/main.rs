use glab::{cmd, toml_string, BANNER, utils::git};
use clap::App;

fn main() {

    let version_message = format!("{}\nGit: {}", toml_string("version"), git::exec(vec!["version"])).to_string();

    let matches = App::new(toml_string("name"))
        .version(toml_string("version").as_str())
        .about(toml_string("description").as_str())
        .before_help(BANNER)
        .long_version(version_message.as_ref())
        .subcommands(cmd::commands())
        .get_matches();

    cmd::cmd_matches(matches);
}