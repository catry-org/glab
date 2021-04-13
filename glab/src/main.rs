use glab::{cmd, toml_string};
use clap::App;

fn main() {
    let matches = App::new(toml_string("name"))
        .version(toml_string("version").as_str())
        .about(toml_string("description").as_str())
        .subcommands(cmd::commands())
        .get_matches();

    cmd::cmd_matches(matches);
}