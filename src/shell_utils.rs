use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn start_shell_search(dir: PathBuf, valid_extensions: Vec<String>) -> Vec<String> {
    let paths = fs::read_dir(dir).unwrap();
    get_dir_files(paths, valid_extensions)
}

pub fn get_dir_files(paths: fs::ReadDir, valid_extensions: Vec<String>) -> Vec<String> {
    let mut files = Vec::new();
    for path in paths {
        let path = path.unwrap().path();
        if has_correct_extension(&path, &valid_extensions) {
            files.push(path.display().to_string());
        }
    }
    files
}

pub fn has_correct_extension(path: &Path, valid_extensions: &[String]) -> bool {
    valid_extensions.contains(&path.extension().unwrap().to_str().unwrap().to_string())
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
            get_dir_files(
                fs::read_dir(PathBuf::from("./src/tests/resources")).unwrap(),
                vec![String::from("")]
            ),
            vec![
                String::from("lines.txt"),
                String::from("test.txt"),
                String::from("test_mult.txt"),
            ],
        )
    }
    #[test]
    fn test_filters_extension_ignores() {
        assert!(
            !(has_correct_extension(
                &PathBuf::from("./src/tests/shell_resources/executable.exe"),
                &[String::from("")]
            ))
        )
    }
    #[test]
    fn test_filters_extension_finds() {
        assert!(has_correct_extension(
            &PathBuf::from("./src/tests/shell_resources/file.txt"),
            &[String::from("txt")]
        ))
    }
}
