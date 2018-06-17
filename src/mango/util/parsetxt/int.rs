use regex::Regex;

/// This matches integer literals, either just numbers in base 10, or base 2-36 with prefix.
/// The syntax for -37 in base 16 is -16b25 and 2748 is 16bABC.
/// Incorrect values like 4b7 or 0b0 are not handled at the lexing stage.
pub fn int_pattern() -> &'static str {
    r"(?:\+|-*)(?:[1-9][0-9]*b(?:_?[0-9a-zA-Z])+|[0-9](?:_?[0-9])*)"
}

/// Convert a String that matches [int_pattern] to an i64 integer. Overflow is possible.
pub fn parse_int<S: Into<String>>(text: S) -> Option<i64> {
    let text = text.into();
    debug_assert!(
        Regex::new(&format!("^{}$", int_pattern()))
            .unwrap()
            .is_match(&text)
    );
    Some(0i64)
}

// TODO: possibly add a i32 version?
// TODO: Option<i64> to deal with overflow?

#[cfg(test)]
mod tests {
    use super::parse_int;

    #[test]
    fn test_parse_int() {
        assert_eq!(42, parse_int("42").unwrap())
    }
}
