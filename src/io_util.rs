use std::fs;

pub fn read_file(files: &[String]) -> Vec<String> {
    let mut all_text: Vec<String> = Vec::new();
    let file_iter = files.iter();
    for val in file_iter {
        let read_text = get_file_contents(val);
        for text in read_text {
            all_text.push(text);
        }
    }
    all_text
}

pub fn get_file_contents(path: &String) -> Vec<String> {
    let lines = fs::read_to_string(path).expect("");
    let mut result = Vec::new();
    let mut word = String::new();
    for char in lines.chars() {
        if char == '\n' {
            result.push(word);
            word = String::from("");
        } else {
            word.push(char);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_text() {
        let path = String::from("src/tests/resources/test.txt");
        let result = get_file_contents(&path);
        assert_eq!(result, vec!["file1"]);
    }
    #[test]
    fn reads_file() {
        let path = String::from("src/tests/resources/test.txt");
        let result = read_file(&vec![path]);
        assert_eq!(result, vec!["file1"]);
    }
    #[test]
    fn reads_lines_separate() {
        let path = String::from("src/tests/resources/lines.txt");
        let result = get_file_contents(&path);
        assert_eq!(result, vec!["line1", "line2"]);
    }
    #[test]
    fn reads_multiple_files() {
        let paths = vec![
            String::from("src/tests/resources/test.txt"),
            String::from("src/tests/resources/test_mult.txt"),
        ];
        let result = read_file(&paths);
        assert_eq!(result, vec!["file1", "file2"]);
    }
}
