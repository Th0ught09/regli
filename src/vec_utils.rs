pub fn push_strs(strings: &Vec<String>) -> String {
    let mut output = String::new();
    for string in strings {
        if !string.is_empty() {
            output.push_str(string);
            output.push('\n');
        };
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_none() {
        assert_eq!(push_strs(&vec![]), "");
    }
    #[test]
    fn handles_one() {
        assert_eq!(push_strs(&vec![String::from("line1")]), "line1\n");
    }
}
