use std::fs;

pub fn read_file(files: &Vec<String>) -> String {
    let mut all_text: String = String::new();
    let file_iter = files.iter();
    for val in file_iter {
        let read_text = get_file_contents(val);
        all_text = all_text + &read_text;
    }
    all_text
}

pub fn get_file_contents(path: &String) -> String {
    fs::read_to_string(path).expect("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_text() {
        let path = String::from("src/tests/resources/test.txt");
        let result = get_file_contents(&path);
        assert_eq!(result, "file1\n");
    }
    #[test]
    fn reads_file() {
        let path = String::from("src/tests/resources/test.txt");
        let result = read_file(&vec![path]);
        assert_eq!(result, "file1\n");
    }
    #[test]
    fn reads_multiple_files() {
        let paths = vec![
            String::from("src/tests/resources/test.txt"),
            String::from("src/tests/resources/test_mult.txt"),
        ];
        let result = read_file(&paths);
        assert_eq!(result, "file1\nfile2\n");
    }
}
