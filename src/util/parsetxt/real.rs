use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::util::strslice::char_ops::CharOps;

#[derive(Debug)]
pub enum RealParseFailReason {
    Invalid,
    Overflow,
    Underflow,
    PrecisionLoss(f64),
}

lazy_static! {
    /// This matches real literals (base 10), which look like this:
    ///   sign / int1 / period / int2 / e / sign / int
    /// Here int is a series of 0-9 digits separated by at most one underscore.
    /// Signs are optional, everything from 'e' is optional, and int1 OR int2 is optional.
    //TODO: is starting with a period allowed?
    pub static ref REAL_RE: Regex = Regex::new(r"^(?P<multiplier>(?:\+|-?)(?:\d(?:_?\d)*\.\d(?:_?\d)*|\d(?:_?\d)*\.|\.\d(?:_?\d)*))(?:e(?P<exponent>(?:\+|-?)\d(?:_?\d)*))?").unwrap();
}

/// Convert a String that matches [real_pattern] to an f64 real. Overflow and loss of precision is possible.
pub fn parse_real(text: &str) -> Result<f64, RealParseFailReason> {
    //TODO @mark: make sure no leftover chars (no $ at the end)
    match REAL_RE.captures(text) {
        None => Err(RealParseFailReason::Invalid),
        Some(captures) => {
            let multiplier = captures
                .name("multiplier")
                .unwrap()
                .as_str()
                .without_char('_')
                .parse::<f64>()
                .unwrap();
            match captures.name("exponent") {
                None => {
                    // This is a 'normal' real, no exponential notation
                    Ok(multiplier)
                }
                Some(exponent_match) => {
                    // This real is in exponential notation
                    let exponent = exponent_match.as_str().without_char('_').parse::<f64>().unwrap();
                    // TODO: is there a numerically smarter way to do this?
                    Ok(10f64.powf(exponent) * multiplier)
                }
            }
        }
    }
}

// TODO: possibly add a f32 version?

#[cfg(test)]
mod tests {
    use super::parse_real;

    fn close(x: f64, y: f64) -> bool {
        (x - y).abs() < 1e-8
    }

    #[test]
    fn test_parse_nonexp_real() {
        assert!(close(42., parse_real("42.0").unwrap()));
        assert!(close(-0.1, parse_real("-.1").unwrap()));
        assert!(close(-1., parse_real("-1.").unwrap()));
        assert!(close(12345.6789, parse_real("1_2_3_4_5.6_7_8_9").unwrap()));
    }

    #[test]
    fn test_parse_exp_real() {
        assert!(close(42., parse_real("42.0e0").unwrap()));
        assert!(close(-0.1, parse_real("-.1e0").unwrap()));
        assert!(close(-1., parse_real("-1.e0").unwrap()));
        assert!(close(42., parse_real("42.0e+0").unwrap()));
        assert!(close(12345.6789, parse_real("1_2_3_4_5.6_7_8_9e0").unwrap()));
        assert!(close(0.42, parse_real("42.0e-2").unwrap()));
        assert!(close(-0.001, parse_real("-.1e-2").unwrap()));
        assert!(close(-0.01, parse_real("-1.e-2").unwrap()));
        assert!(close(123.456_789, parse_real("1_2_3_4_5.6_7_8_9e-2").unwrap()));
        assert!(close(42.0, parse_real("42.0e-0_0_0").unwrap()));
    }

    #[test]
    fn test_invalid_real() {
        assert!(parse_real("+_42.0").is_err());
        assert!(parse_real("-_42.0").is_err());
        assert!(parse_real("_42.0").is_err());
        assert!(parse_real("42_.0").is_err());
        assert!(parse_real("42._0").is_err());
        assert!(parse_real("42.0_").is_err());
        assert!(parse_real("42.0e_0").is_err());
        assert!(parse_real("42.0e0_").is_err());
        assert!(parse_real("42.0e0b0").is_err());
    }

    // TODO: over/underflow
    // TODO: loss of precision
}
