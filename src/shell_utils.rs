use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn start_shell_search(dir: PathBuf) -> Vec<String> {
    let paths = fs::read_dir(dir).unwrap();
    get_dir_files(paths)
}

pub fn get_dir_files(paths: fs::ReadDir) -> Vec<String> {
    let mut files = Vec::new();
    for path in paths {
        files.push(path.unwrap().path().display().to_string());
    }
    files
}

pub fn is_path_file(path: String) -> bool {
    Path::new(path.as_str()).is_file()
}

pub fn is_path_dir(path: String) -> bool {
    Path::new(path.as_str()).is_dir()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn test_gets_dir_file() {
        assert_eq!(
            get_dir_files(fs::read_dir(PathBuf::from("./src/tests/resources")).unwrap()),
            vec![
                String::from("lines.txt"),
                String::from("test.txt"),
                String::from("test_mult.txt"),
            ],
        )
    }
}
