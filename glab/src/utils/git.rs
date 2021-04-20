use std::process::Command;

pub fn exec(args: Vec<&str>) -> String {
    let output = Command::new("git")
        .args(args)
        .output()
        .expect("failed to execute process");

    let output = if output.stdout.is_empty() {
        output.stderr
    } else {
        output.stdout
    };

    return String::from_utf8(output).unwrap();
}