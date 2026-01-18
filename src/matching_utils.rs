use regex::Regex;

pub fn update_matches(
    message: &str,
    matches: &mut Vec<String>,
    non_matches: &mut Vec<String>,
    messages: Vec<String>,
) {
    let re = Regex::new(message).unwrap();
    for message in messages {
        if re.is_match(message.as_str()) {
            matches.push(message.clone())
        } else {
            non_matches.push(message.clone())
        }
    }
}

#[allow(dead_code)]
fn test_matches() -> (Vec<String>, Vec<String>) {
    let message = "h".to_string();
    let mut matches: Vec<String> = Vec::new();
    let mut non_matches: Vec<String> = Vec::new();
    let messages = vec![String::from("hi"), String::from("gq")];
    update_matches(&message, &mut matches, &mut non_matches, messages);
    (matches, non_matches)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pushes_match() {
        let (matches, non_matches) = test_matches();
        assert_eq!(matches, vec!["hi"]);
        assert_eq!(non_matches, vec!["gq"]);
    }
}
