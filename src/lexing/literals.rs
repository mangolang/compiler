use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::lexing::lexer::Lexer;
use crate::lexing::reader::typ::{Reader, ReaderResult};
use crate::lexeme::{ParenthesisCloseLexeme, ParenthesisOpenLexeme, Lexemes};
use crate::lexeme::collect::{
    association, identifier, literal_bool, literal_int, literal_real, literal_text, operator, parenthesis_close, parenthesis_open,
    unlexable,
};
use crate::util::codeparts::operator::ASSOCIATION_RE;
use crate::util::codeparts::operator::SYMBOL_RE;
use crate::util::parsetxt::int::INT_RE;
use crate::util::parsetxt::int::parse_int;
use crate::util::parsetxt::real::parse_real;
use crate::util::parsetxt::real::REAL_RE;
use crate::util::parsetxt::text::parse_single_quote;
use crate::util::parsetxt::text::SINGLE_QUOTE_RE;
use crate::util::strtype::name::IDENTIFIER_RE;

lazy_static! {
    // TODO maybe these will be constants instead of keywords one day
    static ref CONSTANTS_RE: Regex = Regex::new(r"^(?:true|false|NaN|infinity)\b").unwrap();
}

/// Lex literals (text, int, real, boolean), not necessarily to exhaustion.
pub fn lex_literal(reader: &mut impl Reader, lexer: &mut impl Lexer) {
    // Using overall loop instead of per match is needed, because otherwise '1 1.5' is matches as 1, 1
    loop {
        // Constants.
        if let ReaderResult::Match(sym) = reader.strip_match(&*CONSTANTS_RE) {
            lexer.add(match sym.as_str() {
                "true" => literal_bool(true),
                "false" => literal_bool(false),
                //TODO: deal with NaN and infinity (how?)
                "NaN" => panic!("NaN is not currently supported"),
                "infinity" => panic!("infinity is not currently supported"),
                _ => unreachable!(),
            });
            continue;
        }

        // Real numbers.
        if let ReaderResult::Match(sym) = reader.strip_match(&*REAL_RE) {
            //TODO: get rid of this 'unwrap'
            lexer.add(literal_real(parse_real(sym.as_str()).unwrap()));
            continue;
        }

        // Integers.
        if let ReaderResult::Match(sym) = reader.strip_match(&*INT_RE) {
            //TODO: get rid of this 'unwrap'
            lexer.add(literal_int(parse_int(sym.as_str()).unwrap()));
            continue;
        }

        // Text (string literals).
        if let ReaderResult::Match(sym) = reader.strip_match(&*SINGLE_QUOTE_RE) {
            lexer.add(literal_text(parse_single_quote(sym.as_str())));
            continue;
        }

        break;
    }
}

#[cfg(test)]
mod test_util {
    use crate::lexing::lexer::Lexer;
    use crate::lexing::tests::create_lexer;
    use crate::lexeme::{IdentifierLexeme, Lexemes};
    use crate::lexeme::collect::{identifier, literal_bool, literal_int};
    use crate::lexing::lexer::lexeme_collector::LexemeCollector;
    use crate::lexeme::lexemes::OperatorLexeme;
    use crate::util::codeparts::Symbol;
    use crate::util::strtype::Name;
    use crate::util::strtype::typ::StrType;

    use super::lex_literal;

    pub fn check(input: &str, expected: &[Lexemes]) {
        let (source, mut reader, mut lexer) = create_lexer(input);
        lex_literal(&mut reader, &mut lexer);
        assert_eq!(lexer.lexemes(), &expected.into());
    }
}

#[cfg(test)]
mod constants {
    use crate::lexing::lexer::Lexer;
    use crate::lexing::tests::create_lexer;
    use crate::lexeme::{IdentifierLexeme, Lexemes};
    use crate::lexeme::collect::{identifier, literal_bool, literal_int};
    use crate::lexing::lexer::lexeme_collector::LexemeCollector;
    use crate::lexeme::lexemes::OperatorLexeme;
    use crate::util::codeparts::Symbol;
    use crate::util::strtype::Name;
    use crate::util::strtype::typ::StrType;

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
        check("true", &[literal_bool(true)]);
        check("false", &[literal_bool(false)]);
    }

    #[test]
    fn multiple() {
        check(
            "true false\ttrue false",
            &[literal_bool(true), literal_bool(false), literal_bool(true), literal_bool(false)],
        );
    }
}

#[cfg(test)]
mod int {
    use crate::lexing::lexer::Lexer;
    use crate::lexing::tests::create_lexer;
    use crate::lexeme::{IdentifierLexeme, Lexemes};
    use crate::lexeme::collect::{identifier, literal_bool, literal_int};
    use crate::lexing::lexer::lexeme_collector::LexemeCollector;
    use crate::lexeme::lexemes::OperatorLexeme;
    use crate::util::codeparts::Symbol;
    use crate::util::strtype::Name;
    use crate::util::strtype::typ::StrType;

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
        check("0", &[literal_int(0)]);
        check("0000000000000000000000000000000000", &[literal_int(0)]);
    }

    #[test]
    fn prefix() {
        check("+1", &[literal_int(1)]);
        check("-1", &[literal_int(-1)]);
    }

    //#[test]  //TODO: maybe support this one day
    fn double_minus() {
        check("--1", &[literal_int(1)]);
        check("---1", &[literal_int(1)]);
        check("-+-1", &[literal_int(1)]);
    }

    #[test]
    fn valid_underscores() {
        // No need for fancy cases, most parsing-testing should happen at `parse_int`
        check("1_2_3", &[literal_int(123)]);
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
        check(&big, &[literal_int(::std::i64::MAX)]);
        let small = format!("{}", ::std::i64::MIN);
        check(&small, &[literal_int(::std::i64::MIN)]);
    }

    #[test]
    fn multiple() {
        check(
            "1 2 3 1234567890",
            &[literal_int(1), literal_int(2), literal_int(3), literal_int(1234567890)],
        );
    }
}

#[cfg(test)]
mod real {
    use crate::lexing::lexer::Lexer;
    use crate::lexing::tests::create_lexer;
    use crate::lexeme::{IdentifierLexeme, Lexemes};
    use crate::lexeme::collect::{identifier, literal_bool, literal_int, literal_real};
    use crate::lexing::lexer::lexeme_collector::LexemeCollector;
    use crate::lexeme::lexemes::OperatorLexeme;
    use crate::util::codeparts::Symbol;
    use crate::util::strtype::Name;
    use crate::util::strtype::typ::StrType;

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
        check("0.0", &[literal_real(0.0)]);
        check("0.000000000000000000000000000000000", &[literal_real(0.0)]);
        check("000000000000000000000000000000000.0", &[literal_real(0.0)]);
    }

    #[test]
    fn prefix() {
        check("+1.0", &[literal_real(1.0)]);
        check("-1.0", &[literal_real(-1.0)]);
    }

    #[test]
    fn exponential() {
        check("1.0e1", &[literal_real(10.0)]);
        check("1.0e-1", &[literal_real(0.10)]);
        check("-1.0e1", &[literal_real(-10.0)]);
        check("-1.0e-1", &[literal_real(-0.10)]);
        check("+1.0e+1", &[literal_real(10.0)]);
    }

    #[test]
    fn multiple() {
        check(
            "1.1 2.2 3.3 0.1234567890",
            &[literal_real(1.1), literal_real(2.2), literal_real(3.3), literal_real(0.1234567890)],
        );
    }
}

#[cfg(test)]
mod text {
    use crate::lexing::lexer::Lexer;
    use crate::lexing::tests::create_lexer;
    use crate::lexeme::{IdentifierLexeme, Lexemes};
    use crate::lexeme::collect::{identifier, literal_bool, literal_int, literal_real, literal_text};
    use crate::lexing::lexer::lexeme_collector::LexemeCollector;
    use crate::lexeme::lexemes::OperatorLexeme;
    use crate::util::codeparts::Symbol;
    use crate::util::strtype::Name;
    use crate::util::strtype::typ::StrType;

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
        check("''", &[literal_text("")]);
    }

    #[test]
    fn simple() {
        check("'x'", &[literal_text("x")]);
        check("'hello world!'", &[literal_text("hello world!")]);
    }

    #[test]
    fn double_quotes() {
        check("'\"\"'", &[literal_text("\"\"")]);
    }

    //#[test]  //TODO @mark:
    fn unbalanced() {
        // This should match one empty string, leaving a single quote.
        // That single quote should be picked up by unlexable.
        check("'''", &[literal_text("")]);
    }

    #[test]
    fn escaped() {
        check("'\\''", &[literal_text("\\'")]);
    }

    //#[test]  //TODO @mark
    fn escape_escaped() {
        check("'\\\\'", &[literal_text("\\\\")]);
    }

    #[test]
    fn repeated() {
        check(
            "'' 'hello' 'world'",
            &[literal_text(""), literal_text("hello"), literal_text("world")],
        );
        check("'''' ''", &[literal_text(""), literal_text(""), literal_text("")]);
    }
}

#[cfg(test)]
mod exhaustion {
    use crate::lexing::lexer::Lexer;
    use crate::lexing::tests::create_lexer;
    use crate::lexeme::{IdentifierLexeme, Lexemes};
    use crate::lexeme::collect::{identifier, literal_bool, literal_int, literal_real, literal_text};
    use crate::lexing::lexer::lexeme_collector::LexemeCollector;
    use crate::lexeme::lexemes::OperatorLexeme;
    use crate::util::codeparts::Symbol;
    use crate::util::strtype::Name;
    use crate::util::strtype::typ::StrType;

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
            &[literal_bool(true), literal_bool(false), literal_bool(true), literal_bool(false)],
        );
    }

    #[test]
    fn number_before_bool() {
        check(
            "1 false true 1",
            &[literal_int(1), literal_bool(false), literal_bool(true), literal_int(1)],
        );
    }

    #[test]
    fn repeated_numbers() {
        check(
            "1.0e1 1.0e1 1 2 3",
            &[literal_real(10.), literal_real(10.), literal_int(1), literal_int(2), literal_int(3)],
        );
    }

    #[test]
    fn int_before_real() {
        check(
            "1 2 3 1.0e1 1.0e1",
            &[literal_int(1), literal_int(2), literal_int(3), literal_real(10.), literal_real(10.)],
        );
    }

    #[test]
    fn number_then_text() {
        check(
            "1.0e1 37 42 'hello' 'world'",
            &[
                literal_real(1.0e1),
                literal_int(37),
                literal_int(42),
                literal_text("hello"),
                literal_text("world"),
            ],
        );
    }

    #[test]
    fn text_before_number() {
        check(
            "'hello' 'world' 1.0e1",
            &[literal_text("hello"), literal_text("world"), literal_real(10.)],
        );
    }
}

//TODO @mark: mixed test
