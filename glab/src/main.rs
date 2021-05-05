use clap::App;
use glab::{cmd, toml_string, utils::git, BANNER};

fn main() {
    let matches = App::new(toml_string("name"))
        .version(toml_string("version").as_str())
        .about(toml_string("description").as_str())
        .before_help(BANNER)
        .long_version(format!("{}\nGit: {}", toml_string("version"), git::version()).as_str())
        .subcommands(cmd::commands())
        .get_matches();

    cmd::cmd_matches(matches);
}
