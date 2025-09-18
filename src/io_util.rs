use std::fs;

pub fn read_file(files: Vec<String>) -> String {
    let mut all_text: String;
    let file_iter = files.iter();
    for val in file_iter {
        all_text = all_text + &fs::read_to_string(val).expect("Can't read file");
    }
}

