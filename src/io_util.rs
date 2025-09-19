use std::fs;

pub fn read_file(files: &Vec<String>) -> String {
    let mut all_text: String = String::new();
    let file_iter = files.iter();
    for val in file_iter {
        println!("{}", val);
        let read_text = fs::read_to_string(val).expect("Can't read file");
        all_text = all_text + &fs::read_to_string(val).expect("Can't read file");
    }
    all_text
}

pub fn get_file_contents(path: String) -> Result<String, String> {
    let read_text = fs::read_to_string(path).expect("hi")?;
    Ok(read_text);
    Err("".to_string())
}
