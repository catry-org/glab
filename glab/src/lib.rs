pub mod cmd;

pub fn toml_string(id: &str) -> String {
    let file = include_str!("../Cargo.toml");
    let config: toml::Value = toml::from_str(file).unwrap();
    let string = config["package"][id].to_string();

    return string.trim_matches(|x| x == '\"').to_string();
}