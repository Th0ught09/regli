use std::{env, fs};

pub fn start_shell_search() -> Vec<String> {
    let mut files = Vec::new();
    let paths = fs::read_dir(env::current_dir().unwrap()).unwrap();
    for path in paths {
        files.push(path.unwrap().path().display().to_string());
    }
    files
}
