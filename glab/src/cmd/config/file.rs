use std::{
    fs,
    io::{Read, Write},
    path::Path,
};

pub fn read_config_file(file_name: &str) -> String {
    check_config_file(file_name);
    let path = link_config_file(Some(file_name));

    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .unwrap();

    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf).unwrap();
    buf
}

fn check_config_file(file_name: &str) {
    let dir_path = link_config_file(None);
    let file_path = link_config_file(Some(file_name));

    if !Path::new(&dir_path).exists() {
        fs::create_dir(&dir_path).unwrap();
    }

    if !Path::new(&file_path).exists() {
        let mut file = fs::File::create(&file_path).unwrap();
        let buf = r#"
language: en_US
hostname: https://gitlab.com
"#;
        file.write_all(buf.as_bytes()).unwrap();
        fs::metadata(&file_path)
            .unwrap()
            .permissions()
            .set_readonly(true);
    }
}

fn link_config_file(path_file: Option<&str>) -> String {
    let mut var = String::new();

    if cfg!(target_os = "windows") {
        var = std::env::var("USERPROFILE").unwrap();
    } else if cfg!(target_os = "linux") {
        var = std::env::var("HOME").unwrap();
    }

    let mut config_path = std::path::PathBuf::new();

    config_path.push(var);
    config_path.push(".glab");

    if path_file.is_some() {
        config_path.push(path_file.unwrap());
    }

    config_path.to_string_lossy().to_string()
}
