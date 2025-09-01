use std::fs;

pub fn read_file() -> String {
    fs::read_to_string("./test.txt").expect("Can't read file")
}

