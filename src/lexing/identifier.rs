use ::std::str::FromStr;

use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::lexing::lexer::Lexer;
use crate::lexing::reader::typ::{Reader, ReaderResult};
use crate::lexeme::{ParenthesisCloseLexeme, ParenthesisOpenLexeme, Lexeme};
use crate::lexeme::collect::{association, identifier, keyword_or_reserved, operator, parenthesis_close, parenthesis_open, unlexable};
use crate::util::codeparts::Keyword;
use crate::util::codeparts::operator::ASSOCIATION_RE;
use crate::util::codeparts::operator::SYMBOL_RE;
use crate::util::strtype::name::IDENTIFIER_RE;

/// Lex an identifier or keyword.
pub fn lex_keyword_identifier(reader: &mut impl Reader, lexer: &mut impl Lexer) {
    while let ReaderResult::Match(source) = reader.strip_match(&*IDENTIFIER_RE) {
        let word = source.as_str();
        lexer.add(match keyword_or_reserved(word, source.clone()) {
            Ok(kw) => kw,
            Err(err) => identifier(word, source.clone()).unwrap(),
        });
    }
}

#[cfg(test)]
mod identifiers {
    use std::borrow::Cow;

    use crate::lexing::lexer::Lexer;
    use crate::lexing::tests::create_lexer;
    use crate::lexeme::{IdentifierLexeme, Lexeme};
    use crate::lexeme::collect::identifier;
    use crate::lexing::lexer::lexeme_collector::LexemeCollector;
    use crate::lexeme::lexemes::OperatorLexeme;
    use crate::util::codeparts::Symbol;
    use crate::util::strtype::Name;
    use crate::util::strtype::typ::StrType;

    use super::lex_keyword_identifier;

    fn check(input: &str, expected_names: &[&str]) {
        let (source, mut reader, mut lexer) = create_lexer(input);
        lex_keyword_identifier(&mut reader, &mut lexer);
        let expected: LexemeCollector = expected_names
            .iter()
            .map(|n| Lexeme::Identifier(IdentifierLexeme::from_name(Name::new(*n).unwrap())))
            .collect();
        assert_eq!(lexer.lexemes(), &expected);
    }

    #[test]
    fn empty() {
        check("", &[]);
    }

    #[test]
    fn number_prefix() {
        check("1abc", &[]);
        check("0o", &[]);
    }

    #[test]
    fn after_mismatch() {
        check("* abc", &[]);
        check(". x", &[]);
    }

    #[test]
    fn symbol() {
        check("*", &[]);
        check("+", &[]);
        check(".", &[]);
    }

    #[test]
    fn single() {
        check("x", &["x"]);
        check("abc", &["abc"]);
    }

    #[test]
    fn with_numbers() {
        check("x0", &["x0"]);
        check("a1b2c3", &["a1b2c3"]);
    }

    #[test]
    fn underscores() {
        check("_", &["_"]);
        check("_abc", &["_abc"]);
        check("_a_", &["_a_"]);
    }

    #[test]
    fn number_underscore() {
        check("_9", &[]);
    }

    #[test]
    fn with_postfix() {
        check("hi?", &["hi"]);
        check("hi!", &["hi"]);
        check("x.y", &["x"]);
        check("a,b", &["a"]);
    }

    #[test]
    fn multiple() {
        check("mangoes are tasty fruits", &["mangoes", "are", "tasty", "fruits"]);
    }
}

#[cfg(test)]
mod keywords {
    use ::std::str::FromStr;

    use crate::lexing::lexer::Lexer;
    use crate::lexing::tests::create_lexer;
    use crate::lexeme::{IdentifierLexeme, KeywordLexeme, Lexeme};
    use crate::lexeme::collect::{identifier, keyword_or_reserved};
    use crate::lexing::lexer::lexeme_collector::LexemeCollector;
    use crate::lexeme::lexemes::OperatorLexeme;
    use crate::util::codeparts::{Keyword, Symbol};
    use crate::util::codeparts::keyword::KEYWORDS;
    use crate::util::strtype::Name;
    use crate::util::strtype::typ::StrType;

    use super::lex_keyword_identifier;
    use super::mixed::check;

    #[test]
    fn all_keywords() {
        for (name, lexeme) in KEYWORDS.iter() {
            check(name, &[Lexeme::Keyword(KeywordLexeme::from_keyword(lexeme.clone()))]);
        }
    }

    #[test]
    fn multiple() {
        check(
            "let mut mango",
            &[
                Lexeme::Keyword(KeywordLexeme::from_keyword(Keyword::Let)),
                Lexeme::Keyword(KeywordLexeme::from_keyword(Keyword::Mut)),
                Lexeme::Keyword(KeywordLexeme::from_keyword(Keyword::Reserved("mango".to_owned()))),
            ],
        );
    }
}

#[cfg(test)]
mod mixed {
    use ::std::str::FromStr;

    use crate::lexing::lexer::Lexer;
    use crate::lexing::tests::create_lexer;
    use crate::lexeme::{IdentifierLexeme, KeywordLexeme, Lexeme};
    use crate::lexeme::collect::{identifier, keyword_or_reserved};
    use crate::lexing::lexer::lexeme_collector::LexemeCollector;
    use crate::lexeme::lexemes::OperatorLexeme;
    use crate::util::codeparts::{Keyword, Symbol};
    use crate::util::codeparts::keyword::KEYWORDS;
    use crate::util::strtype::Name;
    use crate::util::strtype::typ::StrType;

    use super::lex_keyword_identifier;

    pub fn check(input: &str, expected_keywords: &[Lexeme]) {
        let (source, mut reader, mut lexer) = create_lexer(input);
        lex_keyword_identifier(&mut reader, &mut lexer);
        assert_eq!(lexer.lexemes(), &expected_keywords.into());
    }

    #[test]
    fn multiple() {
        check(
            "let mut python mango",
            &[
                Lexeme::Keyword(KeywordLexeme::from_keyword(Keyword::Let)),
                Lexeme::Keyword(KeywordLexeme::from_keyword(Keyword::Mut)),
                Lexeme::Identifier(IdentifierLexeme::from_name(Name::new("python").unwrap())),
                Lexeme::Keyword(KeywordLexeme::from_keyword(Keyword::Reserved("mango".to_owned()))),
            ],
        );
    }
}
