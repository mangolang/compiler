use ::std::str::FromStr;

use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::lexing::lexer::Lexer;
use crate::lexing::reader::reader::{Reader, ReaderResult};
use crate::token::{ParenthesisCloseToken, ParenthesisOpenToken, Tokens};
use crate::token::collect::{association, identifier, keyword, operator, parenthesis_close, parenthesis_open, unlexable};
use crate::util::codeparts::Keyword;
use crate::util::codeparts::operator::ASSOCIATION_RE;
use crate::util::codeparts::operator::SYMBOL_RE;
use crate::util::strtype::name::IDENTIFIER_RE;

/// Lex an identifier or keyword.
pub fn lex_keyword_identifier(reader: &mut impl Reader, lexer: &mut impl Lexer) {
    while let ReaderResult::Match(sym) = reader.strip_match(&*IDENTIFIER_RE) {
        let word = sym.as_str();
        lexer.add(match keyword(word) {
            Ok(kw) => kw,
            Err(err) => identifier(word).unwrap(),
        });
    }
}

#[cfg(test)]
mod identifiers {
    use std::borrow::Cow;

    use crate::lexing::lexer::Lexer;
    use crate::lexing::tests::create_lexer;
    use crate::token::{IdentifierToken, Tokens};
    use crate::token::collect::identifier;
    use crate::token::collect::token_list::TokenList;
    use crate::token::tokens::OperatorToken;
    use crate::util::codeparts::Symbol;
    use crate::util::strtype::Name;
    use crate::util::strtype::typ::StrType;

    use super::lex_keyword_identifier;

    fn check(input: &str, expected_names: &[&str]) {
        let (source, mut reader, mut lexer) = create_lexer(input);
        lex_keyword_identifier(&mut reader, &mut lexer);
        let expected: TokenList = expected_names.iter()
            .map(|n| Tokens::Identifier(IdentifierToken::from_name(Name::new(*n).unwrap())))
            .collect();
        assert_eq!(lexer.tokens(), &expected);
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
    fn after_mismatch() {
        check("* abc", &vec![]);
        check(". x", &vec![]);
    }

    #[test]
    fn symbol() {
        check("*", &vec![]);
        check("+", &vec![]);
        check(".", &vec![]);
    }

    #[test]
    fn single() {
        check("x", &vec!["x"]);
        check("abc", &vec!["abc"]);
    }

    #[test]
    fn with_numbers() {
        check("x0", &vec!["x0"]);
        check("a1b2c3", &vec!["a1b2c3"]);
    }

    #[test]
    fn underscores() {
        check("_", &vec!["_"]);
        check("_abc", &vec!["_abc"]);
        check("_a_", &vec!["_a_"]);
    }

    #[test]
    fn number_underscore() {
        check("_9", &vec![]);
    }

    #[test]
    fn with_postfix() {
        check("hi?", &vec!["hi"]);
        check("hi!", &vec!["hi"]);
        check("x.y", &vec!["x"]);
        check("a,b", &vec!["a"]);
    }

    #[test]
    fn multiple() {
        check("mangoes are tasty fruits", &vec!["mangoes", "are", "tasty", "fruits"]);
    }
}

#[cfg(test)]
mod keywords {
    use ::std::str::FromStr;

    use crate::lexing::lexer::Lexer;
    use crate::lexing::tests::create_lexer;
    use crate::token::{IdentifierToken, KeywordToken, Tokens};
    use crate::token::collect::{identifier, keyword};
    use crate::token::collect::token_list::TokenList;
    use crate::token::tokens::OperatorToken;
    use crate::util::codeparts::{Keyword, Symbol};
    use crate::util::codeparts::keyword::KEYWORDS;
    use crate::util::strtype::Name;
    use crate::util::strtype::typ::StrType;

    use super::lex_keyword_identifier;
    use super::mixed::check;

    #[test]
    fn all_keywords() {
        for (name, token) in KEYWORDS.iter() {
            check(name, &[Tokens::Keyword(KeywordToken::from_keyword(token.clone()))]);
        }
    }

    #[test]
    fn multiple() {
        check("let mut mango", &[
            Tokens::Keyword(KeywordToken::from_keyword(Keyword::Let)),
            Tokens::Keyword(KeywordToken::from_keyword(Keyword::Mut)),
            Tokens::Keyword(KeywordToken::from_keyword(Keyword::Reserved("mango".to_owned()))),
        ]);
    }
}

#[cfg(test)]
mod mixed {
    use ::std::str::FromStr;

    use crate::lexing::lexer::Lexer;
    use crate::lexing::tests::create_lexer;
    use crate::token::{IdentifierToken, KeywordToken, Tokens};
    use crate::token::collect::{identifier, keyword};
    use crate::token::collect::token_list::TokenList;
    use crate::token::tokens::OperatorToken;
    use crate::util::codeparts::{Keyword, Symbol};
    use crate::util::codeparts::keyword::KEYWORDS;
    use crate::util::strtype::Name;
    use crate::util::strtype::typ::StrType;

    use super::lex_keyword_identifier;

    pub fn check(input: &str, expected_keywords: &[Tokens]) {
        let (source, mut reader, mut lexer) = create_lexer(input);
        lex_keyword_identifier(&mut reader, &mut lexer);
        assert_eq!(lexer.tokens(), &expected_keywords.into());
    }

    #[test]
    fn multiple() {
        check("let mut python mango", &[
            Tokens::Keyword(KeywordToken::from_keyword(Keyword::Let)),
            Tokens::Keyword(KeywordToken::from_keyword(Keyword::Mut)),
            Tokens::Identifier(IdentifierToken::from_name(Name::new("python").unwrap())),
            Tokens::Keyword(KeywordToken::from_keyword(Keyword::Reserved("mango".to_owned()))),
        ]);
    }
}