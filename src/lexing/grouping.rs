use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::lexeme::collect::short::bracket_close;
use crate::lexeme::collect::short::bracket_open;
use crate::lexeme::collect::short::parenthesis_close;
use crate::lexeme::collect::short::parenthesis_open;
use crate::lexeme::collect::short::unlexable;
use crate::lexing::lexer::Lexer;
use crate::lexing::reader::typ::{Reader, ReaderResult};

lazy_static! {
    static ref GROUPING_RE: Regex = Regex::new(r"^[\(\)\[\]\{\}]").unwrap();
}

/// Lex any number of parentheses, braces and brackets, and add the lexemes to the Lexer.
pub fn lex_grouping(reader: &mut impl Reader, lexer: &mut impl Lexer) {
    while let ReaderResult::Match(source) = reader.strip_match(&*GROUPING_RE) {
        lexer.add(match source.as_str() {
            "(" => parenthesis_open(source),
            ")" => parenthesis_close(source),
            "[" => bracket_open(source),
            "]" => bracket_close(source),
            "{" => unlexable(source), //TODO @mark
            "}" => unlexable(source), //TODO @mark
            _ => unreachable!(),
        });
    }
}

#[cfg(test)]
mod test_util {
    use crate::lexeme::Lexeme;
    use crate::lexing::grouping::lex_grouping;
    use crate::lexing::lexer::lexeme_collector::LexemeCollector;
    use crate::lexing::lexer::Lexer;
    use crate::lexing::tests::create_lexer;

    pub fn check(input: &str, expected: &[Lexeme]) {
        let expected: LexemeCollector = expected.into();
        let (_source, mut reader, mut lexer) = create_lexer(input);
        lex_grouping(&mut reader, &mut lexer);
        assert_eq!(lexer.lexemes(), &expected);
    }
}

#[cfg(test)]
mod mismatch {
    use super::test_util::check;

    #[test]
    fn empty() {
        check("", &[]);
        check("    \t", &[]);
    }

    #[test]
    fn after_mismatch() {
        check("abc *", &[]);
        check("0 *", &[]);
    }

    #[test]
    fn mismatch() {
        check("*", &[]);
        check(".", &[]);
        check("0", &[]);
        check("a", &[]);
    }
}

#[cfg(test)]
mod parenthese {
    use crate::lexeme::collect::for_test::builder;

    use super::test_util::check;

    #[test]
    fn open() {
        check(" ( ", &builder().parenthesis_open().build());
    }

    #[test]
    fn close() {
        check(" ) ", &builder().parenthesis_close().build());
    }

    #[test]
    fn paired() {
        check(
            "(( ))",
            &builder()
                .parenthesis_open()
                .parenthesis_open()
                .parenthesis_close()
                .parenthesis_close()
                .build(),
        );
    }

    #[test]
    fn unbalanced() {
        check("(( )", &builder().parenthesis_open().parenthesis_open().parenthesis_close().build());
    }

    #[test]
    fn and_words() {
        check("(hello)", &builder().parenthesis_open().build());
    }
}

#[cfg(test)]
mod brackets {
    use crate::lexeme::collect::for_test::builder;

    use super::test_util::check;

    #[test]
    fn open() {
        check(" [ ", &builder().bracket_open().build());
    }

    #[test]
    fn close() {
        check(" ] ", &builder().bracket_close().build());
    }

    #[test]
    fn paired() {
        check(
            "[[ ]]",
            &builder().bracket_open().bracket_open().bracket_close().bracket_close().build(),
        );
    }

    #[test]
    fn unbalanced() {
        check("[[ ]", &builder().bracket_open().bracket_open().bracket_close().build());
    }

    #[test]
    fn and_words() {
        check("[hello]", &builder().bracket_open().build());
    }
}

#[cfg(test)]
mod mixed {
    use crate::lexeme::collect::for_test::builder;

    use super::test_util::check;

    #[test]
    fn parenthese_inside_brackets() {
        check(
            "[ ( ) ]",
            &builder()
                .bracket_open()
                .parenthesis_open()
                .parenthesis_close()
                .bracket_close()
                .build(),
        )
    }

    #[test]
    fn unbalanced_bracket_and_parenthese() {
        check("[)", &builder().bracket_open().parenthesis_close().build())
    }
}
