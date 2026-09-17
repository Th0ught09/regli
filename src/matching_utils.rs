use crate::ProcessedLine;
use regex::Regex;

// pub fn update_matches(
//     message: &str,
//     matches: &mut Vec<ProcessedLine<String>>,
//     non_matches: &mut Vec<ProcessedLine<String>>,
//     messages: &Vec<ProcessedLine<String>>,
// ) {
//     let re = Regex::new(message).unwrap();
//     for message in messages {
//         if re.is_match(message.item.as_str()) {
//             message.matches()
//         } else {
//             message.missed()
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
}
