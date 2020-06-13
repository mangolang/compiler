use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::lexing::lexer::Lexer;
use crate::lexing::reader::reader::{Reader, ReaderResult};
use crate::token::{ParenthesisCloseToken, ParenthesisOpenToken, Tokens};
use crate::token::collect::{association, identifier, operator, parenthesis_close, parenthesis_open, unlexable, literal_int, literal_bool, literal_real, literal_text};
use crate::util::codeparts::operator::ASSOCIATION_RE;
use crate::util::codeparts::operator::SYMBOL_RE;
use crate::util::strtype::name::IDENTIFIER_RE;

lazy_static! {
    // TODO maybe these will be constants instead of keywords one day
    static ref CONSTANTS_RE: Regex = Regex::new(r"^(?:true|false|NaN|infinity)\b").unwrap();

    // This matches integer literals, either just numbers in base 10, or base 2-36 with prefix.
    // The syntax for -37 in base 16 is -16b25 and 2748 is 16bABC.
    // Incorrect values like 4b7 or 0b0 are not handled at the lexing stage.
    static ref INT_RE: Regex = Regex::new(r"^(?:\+|-*)(?:[1-9][0-9]*b(?:_?[0-9a-zA-Z])+|[0-9](?:_?[0-9])*)").unwrap();

    // This matches real literals (base 10), which look like this:
    //   sign / int1 / period / int2 / e / sign / int
    // Here int is a series of 0-9 digits separated by at most one underscore.
    // Signs are optional, everything from 'e' is optional, and int1 OR int2 is optional.
    //TODO: is starting with a period allowed?
    static ref REAL_RE: Regex = Regex::new(r"^(?:\+|-*)(?:\d(?:_?\d)*\.\d(?:_?\d)*|\d(?:_?\d)*\.|\.\d(?:_?\d)*)(?:e(?:\+|-?)\d(?:_?\d)*)?").unwrap();

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
        lexer.add(literal_real(sym.as_str().parse::<f64>().unwrap().into()));
    }

    // Integers.
    while let ReaderResult::Match(sym) = reader.strip_match(&*INT_RE) {
        lexer.add(literal_int(sym.as_str().parse().unwrap()));
    }

    // Text (string literals).
    while let ReaderResult::Match(sym) = reader.strip_match(&*TEXT_RE) {
        let unquoted = &sym.as_str()[1 .. sym.as_str().len() - 1];
        lexer.add(literal_text(unquoted));
    }
}

// #[cfg(test)]
// mod int {
//     use crate::lexing::lexer::Lexer;
//     use crate::lexing::tests::create_lexer;
//     use crate::token::{IdentifierToken, Tokens};
//     use crate::token::collect::identifier;
//     use crate::token::tokens::OperatorToken;
//     use crate::util::codeparts::Symbol;
//     use crate::util::strtype::Name;
//     use crate::util::strtype::typ::StrType;
//
//     use super::lex_identifier;
//     use std::borrow::Cow;
//     use crate::token::collect::token_list::TokenList;
//
//     fn check(input: &str, expected_names: &[&str]) {
//         let (source, mut reader, mut lexer) = create_lexer(input);
//         lex_identifier(&mut reader, &mut lexer);
//         let expected: TokenList = expected_names.iter()
//             .map(|n| Tokens::Identifier(IdentifierToken::from_name(Name::new(*n).unwrap())))
//             .collect();
//         assert_eq!(lexer.tokens(), &expected);
//     }
//
//     #[test]
//     fn empty() {
//         check("", &vec![]);
//     }
//
//     #[test]
//     fn number_prefix() {
//         check("1abc", &vec![]);
//         check("0o", &vec![]);
//     }
//
//     #[test]
//     fn symbol() {
//         check("*", &vec![]);
//         check("+", &vec![]);
//         check(".", &vec![]);
//     }
//
//     #[test]
//     fn single() {
//         check("x", &vec!["x"]);
//         check("abc", &vec!["abc"]);
//     }
//
//     #[test]
//     fn with_numbers() {
//         check("x0", &vec!["x0"]);
//         check("a1b2c3", &vec!["a1b2c3"]);
//     }
//
//     #[test]
//     fn underscores() {
//         check("_", &vec!["_"]);
//         check("_abc", &vec!["_abc"]);
//         check("_a_", &vec!["_a_"]);
//     }
//
//     #[test]
//     fn number_underscore() {
//         check("_9", &vec![]);
//     }
//
//     #[test]
//     fn with_postfix() {
//         check("hi?", &vec!["hi"]);
//         check("hi!", &vec!["hi"]);
//         check("x.y", &vec!["x"]);
//         check("a,b", &vec!["a"]);
//     }
//
//     #[test]
//     fn multiple() {
//         check("mangoes are tasty fruits", &vec!["mangoes", "are", "tasty", "fruits"]);
//     }
// }
