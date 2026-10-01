use std::collections::HashSet;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::processed_line::ProcessedLine;

pub fn start_shell_search(
    dir: PathBuf,
    valid_extensions: Vec<String>,
) -> HashSet<ProcessedLine<String>> {
    let paths = fs::read_dir(dir).unwrap();
    get_dir_files(paths, valid_extensions)
}

pub fn get_dir_files(
    paths: fs::ReadDir,
    valid_extensions: Vec<String>,
) -> HashSet<ProcessedLine<String>> {
    let mut files = HashSet::new();
    for path in paths {
        let path = path.unwrap().path();
        if has_correct_extension(&path, &valid_extensions) {
            files.insert(ProcessedLine::with_item(path.display().to_string()));
        }
    }
    files
}

pub fn has_correct_extension(path: &Path, valid_extensions: &[String]) -> bool {
    if let Some(extension) = path.extension() {
        if !(valid_extensions.is_empty()) {
            valid_extensions.contains(&extension.to_str().unwrap().to_string())
        } else {
            true
        }
    } else {
        true
    }
}

pub fn is_path_file(path: String) -> bool {
    Path::new(path.as_str()).is_file()
}

pub fn is_path_dir(path: &String) -> bool {
    Path::new(path.as_str()).is_dir()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gets_dir_file() {
        let mut test_set = HashSet::new();
        test_set.insert(ProcessedLine::with_item(String::from(
            "./src/tests/resources/test_dir",
        )));
        test_set.insert(ProcessedLine::with_item(String::from(
            "./src/tests/resources/test_mult.txt",
        )));
        test_set.insert(ProcessedLine::with_item(String::from(
            "./src/tests/resources/lines.txt",
        )));
        test_set.insert(ProcessedLine::with_item(String::from(
            "./src/tests/resources/unused.exe",
        )));
        test_set.insert(ProcessedLine::with_item(String::from(
            "./src/tests/resources/test.txt",
        )));
        assert_eq!(
            get_dir_files(
                fs::read_dir(PathBuf::from("./src/tests/resources")).unwrap(),
                Vec::new()
            ),
            test_set
        )
    }
    #[test]
    fn test_filters_extension_ignores() {
        assert!(
            !(has_correct_extension(
                &PathBuf::from("./src/tests/shell_resources/executable.exe"),
                &[String::from("txt")]
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
