use regex::Regex;

pub fn update_matches(
    message: &String,
    matches: &mut Vec<String>,
    non_matches: &mut Vec<String>,
    messages: Vec<String>,
) {
    let re = Regex::new(message.as_str()).unwrap();
    for message in messages {
        if re.is_match(message.as_str()) {
            matches.push(message.clone())
        } else {
            non_matches.push(message.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pushes_match() {
        let message = "h".to_string();
        let mut matches: Vec<String> = Vec::new();
        let mut non_matches: Vec<String> = Vec::new();
        let messages = vec![String::from("hi"), String::from("gq")];
        updated_matches(&message, &mut matches, &mut non_matches, messages);
    }
}
