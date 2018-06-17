use mango::util::strslice::char_ops::char_drop;
use regex::Regex;

#[derive(Debug)]
pub enum IntParseFailReason {
    Invalid,
    Overflow,
    Underflow,
}

/// This matches integer literals, either just numbers in base 10, or base 2-36 with prefix.
/// The syntax for -37 in base 16 is -16b25 and 2748 is 16bABC.
/// Incorrect values like 4b7 or 0b0 are not handled at the lexing stage.
pub fn int_pattern() -> &'static str {
    r"(?P<sign>\+|-?)(?:(?P<base>[1-9][0-9]*)b(?P<reb_val>(?:_?[0-9a-zA-Z])+)|(?P<b10_val>[0-9](?:_?[0-9])*))"
}

/// Convert a String that matches [int_pattern] to an i64 integer. Overflow is possible.
pub fn parse_int<S: Into<String>>(text: S) -> Result<i64, IntParseFailReason> {
    let text = text.into();
    match Regex::new(&format!("^{}$", int_pattern()))
        .unwrap()
        .captures(&text)
    {
        None => return Err(IntParseFailReason::Invalid),
        Some(captures) => {
            // Sign
            let sign_str = captures.name("sign").unwrap().as_str();
            let sign = if sign_str == "+" || sign_str == "" {
                1 // positive
            } else {
                -1 // negative
            };
            // Check if base10 or special
            match captures.name("b10_val") {
                None => {
                    // There is a base provided.
                    if let Some(base) = captures.name("base") {
                        if let Some(value) = captures.name("reb_val") {
                            // TODO: implement
                            panic!(format!(
                                "Do not yet know how to deal with {} in base {}",
                                char_drop(value.as_str(), &'_'),
                                base.as_str()
                            ))
                        } else {
                            panic!("Expected 'reb_val' match in regex")
                        }
                    } else {
                        panic!("Expected 'base' match in regex")
                    }
                }
                Some(value) => {
                    // This is a 'normal' (base10) value.
                    // TODO: check for over/underflow
                    return Ok(char_drop(value.as_str(), &'_').parse::<i64>().unwrap());
                }
            }
        }
    }
}

// TODO: possibly add a i32 version?
// TODO: Option<i64> to deal with overflow?

#[cfg(test)]
mod tests {
    use super::parse_int;

    #[test]
    fn test_parse_int() {
        assert_eq!(42, parse_int("42").unwrap());
        assert_eq!(42, parse_int("4_2").unwrap());
        // assert_eq!(42, parse_int("10b4_2").unwrap());
    }
}
