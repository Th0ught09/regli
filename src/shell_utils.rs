use std::{env, fs, path::Path};

pub fn start_shell_search() -> Vec<String> {
    let mut files = Vec::new();
    let paths = fs::read_dir(env::current_dir().unwrap()).unwrap();
    for path in paths {
        files.push(path.unwrap().path().display().to_string());
    }
    files
}

pub fn get_dir_files() -> Vec<String> {
    let files = Vec::new();
    files
}

pub fn is_path_file(path: String) -> bool {
    Path::new(path.as_str()).is_file()
}

pub fn is_path_dir(path: String) -> bool {
    Path::new(path.as_str()).is_dir()
}
