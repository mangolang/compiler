use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::lexeme::collect::short::literal_bool;
use crate::lexeme::collect::short::literal_int;
use crate::lexeme::collect::short::literal_real;
use crate::lexeme::collect::short::literal_text;
use crate::lexing::lexer::Lexer;
use crate::lexing::reader::typ::{Reader, ReaderResult};
use crate::lexing::util::int::{parse_int, INT_RE};
use crate::lexing::util::real::{parse_real, REAL_RE};
use crate::lexing::util::text::{parse_single_quote, SINGLE_QUOTE_RE};

lazy_static! {
    // TODO maybe these will be constants instead of keywords one day
    static ref CONSTANTS_RE: Regex = Regex::new(r"^(?:true|false|NaN|infinity)\b").unwrap();
}

/// Lex literals (text, int, real, boolean), not necessarily to exhaustion.
pub fn lex_literal(reader: &mut impl Reader, lexer: &mut impl Lexer) {
    // Using overall loop instead of per match is needed, because otherwise '1 1.5' is matches as 1, 1
    loop {
        // Constants.
        if let ReaderResult::Match(source) = reader.strip_match(&*CONSTANTS_RE) {
            lexer.add(match source.as_str() {
                "true" => literal_bool(true, source),
                "false" => literal_bool(false, source),
                //TODO: deal with NaN and infinity (how?)
                "NaN" => panic!("NaN is not currently supported"),
                "infinity" => panic!("infinity is not currently supported"),
                _ => unreachable!(),
            });
            continue;
        }

        // Real numbers.
        if let ReaderResult::Match(source) = reader.strip_match(&*REAL_RE) {
            //TODO: get rid of this 'unwrap'
            lexer.add(literal_real(parse_real(source.as_str()).unwrap(), source));
            continue;
        }

        // Integers.
        if let ReaderResult::Match(source) = reader.strip_match(&*INT_RE) {
            //TODO: get rid of this 'unwrap'
            lexer.add(literal_int(parse_int(source.as_str()).unwrap(), source));
            continue;
        }

        // Text (string literals).
        if let ReaderResult::Match(source) = reader.strip_match(&*SINGLE_QUOTE_RE) {
            lexer.add(literal_text(parse_single_quote(source.as_str()), source));
            continue;
        }

        break;
    }
}

#[cfg(test)]
mod test_util {
    use crate::lexeme::Lexeme;
    use crate::lexing::lexer::Lexer;
    use crate::lexing::tests::create_lexer;

    use super::lex_literal;

    pub fn check(input: &str, expected: &[Lexeme]) {
        let (_source, mut reader, mut lexer) = create_lexer(input);
        lex_literal(&mut reader, &mut lexer);
        assert_eq!(lexer.lexemes(), &expected.into());
    }
}

#[cfg(test)]
mod constants {
    use crate::lexeme::collect::for_test::*;

    use super::test_util::check;

    #[test]
    fn empty() {
        check("", &[]);
    }

    #[test]
    fn after_mismatch() {
        check("== true", &[]);
        check("a true", &[]);
        check("a NaN", &[]);
    }

    #[test]
    fn too_long() {
        check("trueq", &[]);
        check("falseq", &[]);
        check("NaNq", &[]);
        check("infinityq", &[]);
    }

    #[test]
    fn bool() {
        check("true", &[literal_bool(true).into()]);
        check("false", &[literal_bool(false).into()]);
    }

    #[test]
    fn multiple() {
        check(
            "true false\ttrue false",
            &[
                literal_bool(true).into(),
                literal_bool(false).into(),
                literal_bool(true).into(),
                literal_bool(false).into(),
            ],
        );
    }
}

#[cfg(test)]
mod int {
    use crate::lexeme::collect::for_test::*;

    use super::test_util::check;

    #[test]
    fn empty() {
        check("", &[]);
    }

    #[test]
    fn mismatch() {
        check("!", &[]);
        check("a", &[]);
    }

    #[test]
    fn after_mismatch() {
        check("a 1", &[]);
        check("a1", &[]);
    }

    #[test]
    fn zero() {
        check("0", &[literal_int(0).into()]);
        check("0000000000000000000000000000000000", &[literal_int(0).into()]);
    }

    #[test]
    fn prefix() {
        check("+1", &[literal_int(1).into()]);
        check("-1", &[literal_int(-1).into()]);
    }

    //#[test]  //TODO: maybe support this one day
    fn double_minus() {
        check("--1", &[literal_int(1).into()]);
        check("---1", &[literal_int(1).into()]);
        check("-+-1", &[literal_int(1).into()]);
    }

    #[test]
    fn valid_underscores() {
        // No need for fancy cases, most parsing-testing should happen at `parse_int`
        check("1_2_3", &[literal_int(123).into()]);
    }

    #[test]
    fn invalid_underscores() {
        // No need for fancy cases, most parsing-testing should happen at `parse_int`
        check("1__2_3", &[]);
        check("_1_2_3", &[]);
        check("123_", &[]);
    }

    #[test]
    fn long() {
        let big = format!("{}", ::std::i64::MAX);
        check(&big, &[literal_int(::std::i64::MAX).into()]);
        let small = format!("{}", ::std::i64::MIN);
        check(&small, &[literal_int(::std::i64::MIN).into()]);
    }

    #[test]
    fn multiple() {
        check(
            "1 2 3 1234567890",
            &[
                literal_int(1).into(),
                literal_int(2).into(),
                literal_int(3).into(),
                literal_int(1234567890).into(),
            ],
        );
    }
}

#[cfg(test)]
mod real {
    use crate::lexeme::collect::for_test::*;

    use super::test_util::check;

    #[test]
    fn empty() {
        check("", &[]);
    }

    #[test]
    fn mismatch() {
        check("!", &[]);
        check("a", &[]);
    }

    #[test]
    fn after_mismatch() {
        check("a 1.0", &[]);
        check("a1.0", &[]);
    }

    #[test]
    fn zero() {
        check("0.0", &[literal_real(0.0).into()]);
        check("0.000000000000000000000000000000000", &[literal_real(0.0).into()]);
        check("000000000000000000000000000000000.0", &[literal_real(0.0).into()]);
    }

    #[test]
    fn prefix() {
        check("+1.0", &[literal_real(1.0).into()]);
        check("-1.0", &[literal_real(-1.0).into()]);
    }

    #[test]
    fn exponential() {
        check("1.0e1", &[literal_real(10.0).into()]);
        check("1.0e-1", &[literal_real(0.10).into()]);
        check("-1.0e1", &[literal_real(-10.0).into()]);
        check("-1.0e-1", &[literal_real(-0.10).into()]);
        check("+1.0e+1", &[literal_real(10.0).into()]);
    }

    #[test]
    fn multiple() {
        check(
            "1.1 2.2 3.3 0.1234567890",
            &[
                literal_real(1.1).into(),
                literal_real(2.2).into(),
                literal_real(3.3).into(),
                literal_real(0.1234567890).into(),
            ],
        );
    }
}

#[cfg(test)]
mod text {
    use crate::lexeme::collect::for_test::*;

    use super::test_util::check;

    #[test]
    fn empty() {
        check("", &[]);
    }

    #[test]
    fn mismatch() {
        check("!", &[]);
        check("a", &[]);
    }

    #[test]
    fn after_mismatch() {
        check("a 'a'", &[]);
        check("a'a'", &[]);
    }

    #[test]
    fn no_content() {
        check("''", &[literal_text("").into()]);
    }

    #[test]
    fn simple() {
        check("'x'", &[literal_text("x").into()]);
        check("'hello world!'", &[literal_text("hello world!").into()]);
    }

    #[test]
    fn double_quotes() {
        check("'\"\"'", &[literal_text("\"\"").into()]);
    }

    //#[test]  //TODO @mark:
    fn unbalanced() {
        // This should match one empty string, leaving a single quote.
        // That single quote should be picked up by unlexable.
        check("'''", &[literal_text("").into()]);
    }

    #[test]
    fn escaped() {
        check("'\\''", &[literal_text("\\'").into()]);
    }

    //#[test]  //TODO @mark
    fn escape_escaped() {
        check("'\\\\'", &[literal_text("\\\\").into()]);
    }

    #[test]
    fn repeated() {
        check(
            "'' 'hello' 'world'",
            &[literal_text("").into(), literal_text("hello").into(), literal_text("world").into()],
        );
        check(
            "'''' ''",
            &[literal_text("").into(), literal_text("").into(), literal_text("").into()],
        );
    }
}

#[cfg(test)]
mod exhaustion {
    use crate::lexeme::collect::for_test::*;

    use super::test_util::check;

    //TODO @mark: TEMPORARY! REMOVE THIS!
    // // Constants.
    // while let ReaderResult::Match(sym) = reader.strip_match( & * CONSTANTS_RE) {
    // lexer.add( match sym.as_str() {
    // "true" => literal_bool(true),
    // "false" => literal_bool(false),
    // "NaN" => panic ! ("NaN is not currently supported"),
    // "infinity" => panic ! ("infinity is not currently supported"),
    // _ => unreachable ! (),
    // });
    // }
    //
    // // Real numbers.
    // while let ReaderResult::Match(sym) = reader.strip_match( & * REAL_RE) {
    // lexer.add(literal_real(parse_real(sym.as_str()).unwrap()));
    // }
    //
    // // Integers.
    // while let ReaderResult::Match(sym) = reader.strip_match( & * INT_RE) {
    // lexer.add(literal_int(parse_int(sym.as_str()).unwrap()));
    // }
    //
    // // Text (string literals).
    // while let ReaderResult::Match(sym) = reader.strip_match( & * SINGLE_QUOTE_RE) {
    // lexer.add(literal_text(parse_single_quote(sym.as_str())));
    // }

    #[test]
    fn repeated_booleans_type() {
        check(
            "true false true false",
            &[
                literal_bool(true).into(),
                literal_bool(false).into(),
                literal_bool(true).into(),
                literal_bool(false).into(),
            ],
        );
    }

    #[test]
    fn number_before_bool() {
        check(
            "1 false true 1",
            &[
                literal_int(1).into(),
                literal_bool(false).into(),
                literal_bool(true).into(),
                literal_int(1).into(),
            ],
        );
    }

    #[test]
    fn repeated_numbers() {
        check(
            "1.0e1 1.0e1 1 2 3",
            &[
                literal_real(10.).into(),
                literal_real(10.).into(),
                literal_int(1).into(),
                literal_int(2).into(),
                literal_int(3).into(),
            ],
        );
    }

    #[test]
    fn int_before_real() {
        check(
            "1 2 3 1.0e1 1.0e1",
            &[
                literal_int(1).into(),
                literal_int(2).into(),
                literal_int(3).into(),
                literal_real(10.).into(),
                literal_real(10.).into(),
            ],
        );
    }

    #[test]
    fn number_then_text() {
        check(
            "1.0e1 37 42 'hello' 'world'",
            &[
                literal_real(1.0e1).into(),
                literal_int(37).into(),
                literal_int(42).into(),
                literal_text("hello").into(),
                literal_text("world").into(),
            ],
        );
    }

    #[test]
    fn text_before_number() {
        check(
            "'hello' 'world' 1.0e1",
            &[literal_text("hello").into(), literal_text("world").into(), literal_real(10.).into()],
        );
    }
}

//TODO @mark: mixed test
