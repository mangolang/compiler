use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::lexing::lexer::Lexer;
use crate::lexing::reader::typ::{Reader, ReaderResult};
use crate::lexeme::{ParenthesisCloseLexeme, ParenthesisOpenLexeme, Lexeme};
use crate::lexeme::collect::{bracket_close, bracket_open, parenthesis_close, parenthesis_open, unlexable};

lazy_static! {
    static ref GROUPING_RE: Regex = Regex::new(r"^[\(\)\[\]\{\}]").unwrap();
}

/// Lex any number of parentheses, braces and brackets, and add the lexemes to the Lexer.
pub fn lex_grouping(reader: &mut impl Reader, lexer: &mut impl Lexer) {
    while let ReaderResult::Match(sym) = reader.strip_match(&*GROUPING_RE) {
        lexer.add(match sym.as_str() {
            "(" => parenthesis_open(),
            ")" => parenthesis_close(),
            "[" => bracket_open(),
            "]" => bracket_close(),
            "{" => unlexable("{ not yet implemented"), //TODO @mark
            "}" => unlexable("} not yet implemented"), //TODO @mark
            _ => unreachable!(),
        });
    }
}

#[cfg(test)]
mod test_util {
    use crate::io::source::SourceFile;
    use crate::lexing::grouping::lex_grouping;
    use crate::lexing::lexer::{CodeLexer, Lexer};
    use crate::lexing::reader::source_reader::SourceReader;
    use crate::lexing::tests::create_lexer;
    use crate::lexeme::{EndBlockLexeme, StartBlockLexeme, Lexeme};
    use crate::lexeme::collect::{parenthesis_close, parenthesis_open};
    use crate::lexing::lexer::lexeme_collector::LexemeCollector;

    pub fn check(input: &str, expected: &[Lexeme]) {
        let expected: LexemeCollector = expected.into();
        let (source, mut reader, mut lexer) = create_lexer(input);
        lex_grouping(&mut reader, &mut lexer);
        assert_eq!(lexer.lexemes(), &expected);
    }
}

#[cfg(test)]
mod mismatch {
    use crate::io::source::SourceFile;
    use crate::lexing::lexer::{CodeLexer, Lexer};
    use crate::lexing::reader::source_reader::SourceReader;
    use crate::lexing::tests::create_lexer;
    use crate::lexeme::{EndBlockLexeme, StartBlockLexeme, Lexeme};
    use crate::lexeme::collect::{parenthesis_close, parenthesis_open};
    use crate::lexing::lexer::lexeme_collector::LexemeCollector;

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
    use crate::io::source::SourceFile;
    use crate::lexing::lexer::{CodeLexer, Lexer};
    use crate::lexing::reader::source_reader::SourceReader;
    use crate::lexing::tests::create_lexer;
    use crate::lexeme::{EndBlockLexeme, StartBlockLexeme, Lexeme};
    use crate::lexeme::collect::{parenthesis_close, parenthesis_open};
    use crate::lexing::lexer::lexeme_collector::LexemeCollector;

    use super::test_util::check;

    #[test]
    fn open() {
        check(" ( ", &[parenthesis_open()]);
    }

    #[test]
    fn close() {
        check(" ) ", &[parenthesis_close()]);
    }

    #[test]
    fn paired() {
        check(
            "(( ))",
            &[parenthesis_open(), parenthesis_open(), parenthesis_close(), parenthesis_close()],
        );
    }

    #[test]
    fn unbalanced() {
        check("(( )", &[parenthesis_open(), parenthesis_open(), parenthesis_close()]);
    }

    #[test]
    fn and_words() {
        check("(hello)", &[parenthesis_open()]);
    }
}

#[cfg(test)]
mod brackets {
    use crate::io::source::SourceFile;
    use crate::lexing::lexer::{CodeLexer, Lexer};
    use crate::lexing::reader::source_reader::SourceReader;
    use crate::lexing::tests::create_lexer;
    use crate::lexeme::{EndBlockLexeme, StartBlockLexeme, Lexeme};
    use crate::lexeme::collect::{bracket_close, bracket_open};
    use crate::lexing::lexer::lexeme_collector::LexemeCollector;

    use super::test_util::check;

    #[test]
    fn open() {
        check(" [ ", &[bracket_open()]);
    }

    #[test]
    fn close() {
        check(" ] ", &[bracket_close()]);
    }

    #[test]
    fn paired() {
        check("[[ ]]", &[bracket_open(), bracket_open(), bracket_close(), bracket_close()]);
    }

    #[test]
    fn unbalanced() {
        check("[[ ]", &[bracket_open(), bracket_open(), bracket_close()]);
    }

    #[test]
    fn and_words() {
        check("[hello]", &[bracket_open()]);
    }
}

#[cfg(test)]
mod mixed {
    use crate::io::source::SourceFile;
    use crate::lexing::lexer::{CodeLexer, Lexer};
    use crate::lexing::reader::source_reader::SourceReader;
    use crate::lexing::tests::create_lexer;
    use crate::lexeme::{EndBlockLexeme, StartBlockLexeme, Lexeme};
    use crate::lexeme::collect::{bracket_close, bracket_open, parenthesis_close, parenthesis_open};
    use crate::lexing::lexer::lexeme_collector::LexemeCollector;

    use super::test_util::check;

    #[test]
    fn parenthese_inside_brackets() {
        check(
            "[ ( ) ]",
            &[bracket_open(), parenthesis_open(), parenthesis_close(), bracket_close()],
        );
    }

    #[test]
    fn unbalanced_bracket_and_parenthese() {
        check("[)", &[bracket_open(), parenthesis_close()]);
    }
}
