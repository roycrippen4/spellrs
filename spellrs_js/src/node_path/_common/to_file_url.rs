use std::collections::HashMap;

use regex::Regex;

pub fn encode_whitespace(input: &str) -> String {
    let whitespace_encodings: HashMap<char, &str> = [
        ('\u{0009}', "%09"), // Tab
        ('\u{000A}', "%0A"), // Line Feed
        ('\u{000B}', "%0B"), // Vertical Tab
        ('\u{000C}', "%0C"), // Form Feed
        ('\u{000D}', "%0D"), // Carriage Return
        ('\u{0020}', "%20"), // Space
    ]
    .iter()
    .cloned()
    .collect();

    let re = Regex::new(r"[\x09\x0A\x0B\x0C\x0D\x20]").unwrap();

    re.replace_all(input, |caps: &regex::Captures| {
        let ch = caps[0].chars().next().unwrap(); // Extract the matched character
        whitespace_encodings
            .get(&ch)
            .unwrap_or(&&caps[0])
            .to_string()
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_whitespace() {
        assert_eq!(encode_whitespace("foo"), "foo");
        assert_eq!(encode_whitespace("foo\tbar"), "foo%09bar");
        assert_eq!(encode_whitespace("foo\nbar"), "foo%0Abar");
        assert_eq!(encode_whitespace("foo\u{000B}bar"), "foo%0Bbar");
        assert_eq!(encode_whitespace("foo\u{000C}bar"), "foo%0Cbar");
        assert_eq!(encode_whitespace("foo\rbar"), "foo%0Dbar");
        assert_eq!(encode_whitespace("foo bar"), "foo%20bar");
        assert_eq!(encode_whitespace("foo\u{0009}bar"), "foo%09bar");
        assert_eq!(encode_whitespace("foo\u{000A}bar"), "foo%0Abar");
        assert_eq!(encode_whitespace("foo\u{000B}bar"), "foo%0Bbar");
        assert_eq!(encode_whitespace("foo\u{000C}bar"), "foo%0Cbar");
        assert_eq!(encode_whitespace("foo\u{000D}bar"), "foo%0Dbar");
        assert_eq!(encode_whitespace("foo\u{0020}bar"), "foo%20bar");
        assert_eq!(encode_whitespace("foo\u{FEFF}bar"), "foo\u{FEFF}bar");
    }
}
