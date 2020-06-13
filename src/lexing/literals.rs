use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::lexing::lexer::Lexer;
use crate::lexing::reader::reader::{Reader, ReaderResult};
use crate::token::{ParenthesisCloseToken, ParenthesisOpenToken, Tokens};
use crate::token::collect::{association, identifier, literal_bool, literal_int, literal_real, literal_text, operator, parenthesis_close, parenthesis_open, unlexable};
use crate::util::codeparts::operator::ASSOCIATION_RE;
use crate::util::codeparts::operator::SYMBOL_RE;
use crate::util::strtype::name::IDENTIFIER_RE;

lazy_static! {
    // TODO maybe these will be constants instead of keywords one day
    static ref CONSTANTS_RE: Regex = Regex::new(r"^(?:true|false|NaN|infinity)\b").unwrap();

    // This matches integer literals, either just numbers in base 10, or base 2-36 with prefix.
    // The syntax for -37 in base 16 is -16b25 and 2748 is 16bABC.
    // Incorrect values like 4b7 or 0b0 are not handled at the lexing stage.
    static ref INT_RE: Regex = Regex::new(r"^(?:\+|-*)(?:[1-9][0-9]*b(?:_?[0-9a-zA-Z])+|[0-9](?:_?[0-9])*)\b").unwrap();

    // This matches real literals (base 10), which look like this:
    //   sign / int1 / period / int2 / e / sign / int
    // Here int is a series of 0-9 digits separated by at most one underscore.
    // Signs are optional, everything from 'e' is optional, and int1 OR int2 is optional.
    //TODO: is starting with a period allowed?
    static ref REAL_RE: Regex = Regex::new(r"^(?:\+|-*)(?:\d(?:_?\d)*\.\d(?:_?\d)*|\d(?:_?\d)*\.|\.\d(?:_?\d)*)(?:e(?:\+|-?)\d(?:_?\d)*)?\b").unwrap();

    // From a single quote until the first non-escaped single-quote on the same line.
    // TODO: Only single-quoted, single-line strings for now; double-quoted strings may become templates?
    static ref TEXT_RE: Regex = Regex::new(r"^(?:'[^\n\r]*?[^\\]'|'')").unwrap();
}

/// Lex a single literal (text, int, real, boolean).
pub fn lex_literal(reader: &mut impl Reader, lexer: &mut impl Lexer) {

    // Constants.
    while let ReaderResult::Match(sym) = reader.strip_match(&*CONSTANTS_RE) {
        lexer.add(match sym.as_str() {
            "true" => literal_bool(true),
            "false" => literal_bool(false),
            //TODO: deal with NaN and infinity (how?)
            "NaN" => panic!("NaN is not currently supported"),
            "infinity" => panic!("infinity is not currently supported"),
            _ => unreachable!(),
        });
    }

    // Real numbers.
    while let ReaderResult::Match(sym) = reader.strip_match(&*REAL_RE) {
        lexer.add(literal_real(sym.as_str()
            .parse::<f64>().unwrap().into()));
    }

    // Integers.
    while let ReaderResult::Match(sym) = reader.strip_match(&*INT_RE) {
        lexer.add(literal_int(sym.as_str()
            .replace("_", "")
            .parse().unwrap()));
    }

    // Text (string literals).
    while let ReaderResult::Match(sym) = reader.strip_match(&*TEXT_RE) {
        let unquoted = &sym.as_str()[1 .. sym.as_str().len() - 1];
        lexer.add(literal_text(unquoted));
    }
}

//TODO @mark: after mismatch test

#[cfg(test)]
mod int {
    use crate::lexing::lexer::Lexer;
    use crate::lexing::tests::create_lexer;
    use crate::token::{IdentifierToken, Tokens};
    use crate::token::collect::{identifier, literal_int, literal_bool};
    use crate::token::collect::token_list::TokenList;
    use crate::token::tokens::OperatorToken;
    use crate::util::codeparts::Symbol;
    use crate::util::strtype::Name;
    use crate::util::strtype::typ::StrType;

    use super::lex_literal;

    fn check(input: &str, expected: &[Tokens]) {
        let (source, mut reader, mut lexer) = create_lexer(input);
        lex_literal(&mut reader, &mut lexer);
        assert_eq!(lexer.tokens(), &expected.into());
    }

    #[test]
    fn empty() {
        check("", &vec![]);
    }

    #[test]
    fn after_mismatch() {
        check("a 1", &vec![]);
        check("a1", &vec![]);
    }

    #[test]
    fn zero() {
        check("0", &vec![literal_int(0)]);
        check("0000000000000000000000000000000000", &vec![literal_int(0)]);
    }

    #[test]
    fn prefix() {
        check("+1", &vec![literal_int(1)]);
        check("-1", &vec![literal_int(-1)]);
    }

    //#[test]  //TODO: maybe support this one day
    fn double_minus() {
        check("--1", &vec![literal_int(1)]);
        check("---1", &vec![literal_int(1)]);
        check("-+-1", &vec![literal_int(1)]);
    }

    #[test]
    fn valid_underscores() {
        check("1_2_3", &vec![literal_int(123)]);
    }

    #[test]
    fn invalid_underscores() {
        check("1__2_3", &vec![]);
        check("_1_2_3", &vec![]);
        check("123_", &vec![]);
    }

    #[test]
    fn long() {
        let big = format!("{}", ::std::i64::MAX);
        check(&big, &vec![literal_int(::std::i64::MAX)]);
        let small = format!("{}", ::std::i64::MIN);
        check(&small, &vec![literal_int(::std::i64::MIN)]);
    }

    #[test]
    fn multiple() {
        check("1 2 3 1234567890", &vec![
            literal_int(1),
            literal_int(2),
            literal_int(3),
            literal_int(1234567890),
        ]);
    }
}


//TODO @mark: mixed test