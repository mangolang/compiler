use mango::util::strslice::char_ops::CharOps;
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
    r"(?:(?P<base>(?:\+|-?)[1-9][0-9]*)b(?P<reb_val>(?:_?[0-9a-zA-Z])+)|(?P<b10_val>(?:\+|-?)[0-9](?:_?[0-9])*))"
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
            //            // Sign
            //            let sign_str = captures.name("sign").unwrap().as_str();
            //            let sign = if sign_str == "+" || sign_str == "" {
            //                1 // positive
            //            } else {
            //                -1 // negative
            //            };
            // Check if base10 or special
            match captures.name("b10_val") {
                None => {
                    // There is a base provided.
                    if let Some(base) = captures.name("base") {
                        if let Some(value) = captures.name("reb_val") {
                            // TODO: implement
                            panic!(format!(
                                "Do not yet know how to deal with {} in base {}",
                                value.as_str().without_char(&'_'),
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
                    return Ok(value.as_str().without_char(&'_').parse::<i64>().unwrap());
                }
            }
        }
    }
}

// TODO: possibly add a i32 version?

#[cfg(test)]
mod tests {
    use super::parse_int;

    #[test]
    fn test_parse_b10_int() {
        assert_eq!(42, parse_int("42").unwrap());
        assert_eq!(42, parse_int("4_2").unwrap());
        assert_eq!(123456789, parse_int("+1_2_3_4_5_6_7_8_9").unwrap());
        assert_eq!(-123456789, parse_int("-123456789").unwrap());
        assert_eq!(0, parse_int("-0").unwrap());
        assert_eq!(-1, parse_int("-1").unwrap());
        // Weird bases with 0 prefix are not supported.
        assert_eq!(9, parse_int("09").unwrap());
    }

    fn test_invalid_b10_ints() {
        assert!(parse_int("0x9").is_err());
        assert!(parse_int("A").is_err());
        assert!(parse_int("_0").is_err());
        assert!(parse_int("0_").is_err());
        // TODO: over/underflow
    }

    fn test_parse_based_ints() {
        // TODO: not implemented yet
    }
}
