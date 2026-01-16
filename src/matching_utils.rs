use regex::Regex;

pub fn updated_matches(
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
        let matches: Vec<&mut String> = Vec::new();
        let non_matches: Vec<&mut String> = Vec::new();
        let messages = vec!["hi", "gq"];
        updated_matches(&message, matches, non_matches, messages);
    }
}
