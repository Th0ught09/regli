pub fn push_strs(strings: &Vec<String>) -> String {
    let mut output = String::new();
    for string in strings {
        output.push_str(string);
        output.push('\n');
    }
    output
}
