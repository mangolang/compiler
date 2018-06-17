use regex::Regex;

/// This matches real literals (base 10), which look like this:
///   sign / int1 / period / int2 / e / sign / int
/// Here int is a series of 0-9 digits separated by at most one underscore.
/// Signs are optional, everything from 'e' is optional, and int1 OR int2 is optional.
pub fn real_pattern() -> &'static str {
    // TODO: do I want to allow numbers to start with a period?
    // TODO: for now, only base10 for reals (would 8b11e2 be 9*8^2 or 9*10^2?)
    // TODO: does not deal with NaN of infinity
    r"(?:\+|-*)(?:\d(?:_?\d)*\.\d(?:_?\d)*|\d(?:_?\d)*\.|\.\d(?:_?\d)*)(?:e(?:\+|-|)\d(?:_?\d)*)?"
}

/// Convert a String that matches [real_pattern] to an f64 real. Overflow and loss of precision is possible.
pub fn parse_real<S: Into<String>>(text: S) -> Option<f64> {
    let text = text.into();
    debug_assert!(
        Regex::new(&format!("^{}$", real_pattern()))
            .unwrap()
            .is_match(&text)
    );
    Some(0.0f64)
}

// TODO: possibly add a i32 version?
// TODO: Option<i64> to deal with overflow?

#[cfg(test)]
mod tests {
    use super::parse_real;

    #[test]
    fn test_parse_int() {
        assert_eq!(42., parse_real("42.").unwrap())
    }
}
