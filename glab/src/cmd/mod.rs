use clap::{App, ArgMatches, SubCommand};

// Commands Init
pub fn commands<'a, 'b>() -> Vec<App<'a, 'b>> {
    vec![
            // config Command
            SubCommand::with_name("config")
                .about("config command")
                .subcommand(SubCommand::with_name("list"))
                .subcommand(SubCommand::with_name("get")),
            // update Command
            SubCommand::with_name("update")
                .about("update command"),
            // auth Command
            SubCommand::with_name("auth")
                .about("auth command")
                .subcommand(SubCommand::with_name("login"))
                .subcommand(SubCommand::with_name("logout"))
                .subcommand(SubCommand::with_name("check")),
            // repo Command
            SubCommand::with_name("repo")
                .about("clone command")
                .subcommand(SubCommand::with_name("clone"))
                .subcommand(SubCommand::with_name("search")),
            // issue Command
            SubCommand::with_name("issue")
                .about("issue command")
                .subcommand(SubCommand::with_name("checkout")),
            // Pull Request (PR) Command
            SubCommand::with_name("pr")
                .about("Pull Request command")
                .subcommand(SubCommand::with_name("checkout")),
            // wiki Command
            SubCommand::with_name("wiki")
                .about("wiki command"),
        ]
}

/// Commands Handler
pub fn cmd_matches(matches: ArgMatches<'_>) {
    if matches.is_present("config") {
        println!("{}", i18n("en_US", "test"))
    }
}

// i18n handler
fn i18n(lang: &str, id: &str) -> String {
    let ctx = json_gettext::static_json_gettext_build!(
        "ko_KR",
        "ko_KR", "i18n/ko_KR.json",
        "en_US", "i18n/en_US.json"
    ).unwrap();

    return json_gettext::get_text!(ctx, lang, id).unwrap().to_string();
}
