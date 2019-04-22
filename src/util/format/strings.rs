/// Convert a normal string to a double-quoted one that would result in the original
/// string when parsed by a typical language.
pub fn to_double_quoted_str(txt: &str) -> String {
    // todo: performance? mostly I'd like to add the quotes as part of the stream, but it seems difficult
    let esc: String = txt
        .chars()
        .map(|c| match c {
            '\\' => r"\\".to_string(),
            '\"' => "\\\"".to_string(),
            '\n' => "\\n".to_string(),
            '\0' => panic!("Found null byte in to_double_quoted_str"),
            c => c.to_string(),
        })
        .collect();
    "\"".to_string() + &esc + "\""
}

#[cfg(test)]
mod tests {
    use super::to_double_quoted_str;

    #[test]
    fn test_to_double_quoted_str() {
        assert_eq!("\"hello world\"", to_double_quoted_str("hello world"));
        assert_eq!("\"hello world\"", to_double_quoted_str("hello world"));
        assert_eq!("\"hello\\nworld\"", to_double_quoted_str("hello\nworld"));
        assert_eq!("\"hello\\\\ world\"", to_double_quoted_str("hello\\ world"));
        assert_eq!("\"hello\\\"world\"", to_double_quoted_str("hello\"world"));
        assert_eq!(
            "\"\\\"\\\"\\\"\\n\\\\\"",
            to_double_quoted_str("\"\"\"\n\\")
        );
        assert_eq!("\"\\\\n\"", to_double_quoted_str("\\n"));
        assert_eq!("\"\\\\\\n\"", to_double_quoted_str("\\\n"));
    }
}
