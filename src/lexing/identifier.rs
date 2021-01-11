use crate::common::codeparts::name::IDENTIFIER_RE;
use crate::lexeme::collect::short::identifier;
use crate::lexeme::collect::short::keyword_or_reserved;
use crate::lexing::lexer::Lexer;
use crate::lexing::reader::typ::{Reader, ReaderResult};

/// Lex an identifier or keyword.
///
/// Note that fully-qualified names are created at parse time (identifier + period + identifier).
pub fn lex_keyword_identifier(reader: &mut impl Reader, lexer: &mut impl Lexer) {
    while let ReaderResult::Match(source) = reader.strip_match(&*IDENTIFIER_RE) {
        let word = source.as_str();
        lexer.add(match keyword_or_reserved(word, source.clone()) {
            Ok(kw) => kw,
            Err(_err) => identifier(word, source.clone()).unwrap(),
        });
    }
}

#[cfg(test)]
mod identifiers {
    use crate::common::codeparts::name::Name;
    use crate::io::slice::SourceSlice;
    use crate::lexeme::{IdentifierLexeme, Lexeme};
    use crate::lexing::lexer::lexeme_collector::LexemeCollector;
    use crate::lexing::lexer::Lexer;
    use crate::lexing::tests::create_lexer;

    use super::lex_keyword_identifier;

    fn check(input: &str, expected_names: &[&str]) {
        let (_source, mut reader, mut lexer) = create_lexer(input);
        lex_keyword_identifier(&mut reader, &mut lexer);
        let expected: LexemeCollector = expected_names
            .iter()
            .map(|n| Lexeme::Identifier(IdentifierLexeme::from_name(Name::new(*n).unwrap(), SourceSlice::mock())))
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
    use crate::common::codeparts::Keyword;
    use crate::common::codeparts::keyword::KEYWORDS;
    use crate::io::slice::SourceSlice;
    use crate::lexeme::{KeywordLexeme, Lexeme};
    use crate::lexeme::collect::for_test::keyword_or_reserved;

    use super::mixed::check;

    #[test]
    fn all_keywords() {
        for (name, lexeme) in KEYWORDS.iter() {
            check(
                name,
                &[Lexeme::Keyword(KeywordLexeme::from_keyword(lexeme.clone(), SourceSlice::mock()))],
            );
        }
    }

    #[test]
    fn multiple() {
        check(
            "let mut mango",
            &[
                keyword_or_reserved(Keyword::Let),
                keyword_or_reserved(Keyword::Mut),
                keyword_or_reserved(Keyword::Reserved("mango".to_owned())),
            ],
        );
    }
}

#[cfg(test)]
mod mixed {
    use crate::common::codeparts::Keyword;
    use crate::common::codeparts::name::Name;
    use crate::io::slice::SourceSlice;
    use crate::lexeme::{IdentifierLexeme, KeywordLexeme, Lexeme};
    use crate::lexing::lexer::Lexer;
    use crate::lexing::tests::create_lexer;

    use super::lex_keyword_identifier;

    pub fn check(input: &str, expected_keywords: &[Lexeme]) {
        let (_source, mut reader, mut lexer) = create_lexer(input);
        lex_keyword_identifier(&mut reader, &mut lexer);
        assert_eq!(lexer.lexemes(), &expected_keywords.into());
    }

    #[test]
    fn multiple() {
        check(
            "let mut python mango",
            &[
                Lexeme::Keyword(KeywordLexeme::from_keyword(Keyword::Let, SourceSlice::mock())),
                Lexeme::Keyword(KeywordLexeme::from_keyword(Keyword::Mut, SourceSlice::mock())),
                Lexeme::Identifier(IdentifierLexeme::from_name(Name::new("python").unwrap(), SourceSlice::mock())),
                Lexeme::Keyword(KeywordLexeme::from_keyword(
                    Keyword::Reserved("mango".to_owned()),
                    SourceSlice::mock(),
                )),
            ],
        );
    }
}
