use std::process::Command;

pub fn version() -> String {
    exec(vec!["version"])
}

fn exec(args: Vec<&str>) -> String {
    let output = Command::new("git")
        .args(args)
        .output()
        .expect("failed to execute process");

    if output.stdout.is_empty() {
        panic!("{}", String::from_utf8(output.stderr).unwrap());
    } else {
        return String::from_utf8(output.stdout).unwrap();
    }
}
