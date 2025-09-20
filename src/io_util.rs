use std::fs;

pub fn read_file(files: &Vec<String>) -> String {
    let mut all_text: String = String::new();
    let file_iter = files.iter();
    for val in file_iter {
        println!("{}", val);
        let read_text = get_file_contents(val);
        all_text = all_text + &read_text;
    }
    all_text
}

pub fn get_file_contents(path: &String) -> String {
    fs::read_to_string(path).expect("")
}
