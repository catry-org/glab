use clap::{App, Arg, ArgMatches, SubCommand};

// Commands Init
pub fn commands<'a, 'b>() -> Vec<App<'a, 'b>> {
    vec![
            // config Command
            SubCommand::with_name("config")
                .about("config command")
                .arg(Arg::with_name("config_name").index(1)),
            // update Command
            SubCommand::with_name("update")
                .about("update command"),
            // auth Command
            SubCommand::with_name("auth")
                .about("auth command"),
            // repo Command
            SubCommand::with_name("repo")
                .about("clone command"),
            // issue Command
            SubCommand::with_name("issue")
                .about("issue command"),
            // Pull Request (PR) Command
            SubCommand::with_name("pr")
                .about("Pull Request command"),
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
