use regex::Regex;

pub fn updated_matches(message: &String,  matches:&mut Vec<String>,  non_matches:&mut Vec<String>, messages: Vec<String>) {
    let re = Regex::new(message.as_str()).unwrap();
    for message in messages {
        if re.is_match(message.as_str()) {
            matches.push(message.clone())
        } else {
            non_matches.push(message.clone())
        }
    }
}
