use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::lexing::lexer::Lexer;
use crate::lexing::reader::reader::{Reader, ReaderResult};
use crate::token::{ParenthesisCloseToken, ParenthesisOpenToken, Tokens};
use crate::token::collect::{association, identifier, operator, parenthesis_close, parenthesis_open, unlexable};
use crate::util::codeparts::operator::ASSOCIATION_RE;
use crate::util::codeparts::operator::SYMBOL_RE;
use crate::util::strtype::name::IDENTIFIER_RE;
use crate::util::strtype::typ::StrType;

/// Lex an identifier.
pub fn lex_identifier(reader: &mut impl Reader, lexer: &mut impl Lexer) {
    while let ReaderResult::Match(sym) = reader.strip_match(&*IDENTIFIER_RE) {
        lexer.add(identifier(sym.as_str()).unwrap());
    }
}

#[cfg(test)]
mod identifiers {
    use crate::lexing::lexer::Lexer;
    use crate::lexing::tests::create_lexer;
    use crate::token::{IdentifierToken, Tokens};
    use crate::token::collect::identifier;
    use crate::token::tokens::OperatorToken;
    use crate::util::codeparts::Symbol;
    use crate::util::strtype::Name;

    use super::lex_identifier;

    fn check(input: &str, expected: &[Tokens]) {
        let (source, mut reader, mut lexer) = create_lexer(input);
        lex_identifier(&mut reader, &mut lexer);
        assert_eq!(lexer.tokens(), expected);
    }

    #[test]
    fn empty() {
        check("", &vec![]);
    }

    #[test]
    fn number_prefix() {
        check("1abc", &vec![]);
        check("0o", &vec![]);
    }

    #[test]
    fn symbol() {
        check("*", &vec![]);
        check("+", &vec![]);
        check(".", &vec![]);
    }

    #[test]
    fn single() {
        check("x", &vec![Tokens::Identifier(IdentifierToken::from_name(Name::new("")?))]);
        check("abc", &vec![]);
    }

    #[test]
    fn with_numbers() {
        check("x0", &vec![Tokens::Identifier(IdentifierToken::from_name(Name::new("")?))]);
        check("a1b2c3", &vec![]);
    }

    #[test]
    fn underscores() {
        check("_", &vec![]);
        check("_abc", &vec![]);
        check("_a_", &vec![]);
    }


    #[test]
    fn number_underscore() {
        check("_9", &vec![]);
    }
}
