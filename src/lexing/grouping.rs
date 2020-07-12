use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::lexing::lexer::Lexer;
use crate::lexing::reader::reader::{Reader, ReaderResult};
use crate::token::collect::{bracket_close, bracket_open, parenthesis_close, parenthesis_open, unlexable};
use crate::token::{ParenthesisCloseToken, ParenthesisOpenToken, Tokens};

lazy_static! {
    static ref GROUPING_RE: Regex = Regex::new(r"^[\(\)\[\]\{\}]").unwrap();
}

/// Lex any number of parentheses, braces and brackets, and add the tokens to the Lexer.
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
    use crate::token::collect::token_list::TokenList;
    use crate::token::collect::{parenthesis_close, parenthesis_open};
    use crate::token::{EndBlockToken, StartBlockToken, Tokens};

    pub fn check(input: &str, expected: &[Tokens]) {
        let expected: TokenList = expected.into();
        let (source, mut reader, mut lexer) = create_lexer(input);
        lex_grouping(&mut reader, &mut lexer);
        assert_eq!(lexer.tokens(), &expected);
    }
}

#[cfg(test)]
mod mismatch {
    use crate::io::source::SourceFile;
    use crate::lexing::lexer::{CodeLexer, Lexer};
    use crate::lexing::reader::source_reader::SourceReader;
    use crate::lexing::tests::create_lexer;
    use crate::token::collect::token_list::TokenList;
    use crate::token::collect::{parenthesis_close, parenthesis_open};
    use crate::token::{EndBlockToken, StartBlockToken, Tokens};

    use super::test_util::check;

    #[test]
    fn empty() {
        check("", &vec![]);
        check("    \t", &vec![]);
    }

    #[test]
    fn after_mismatch() {
        check("abc *", &vec![]);
        check("0 *", &vec![]);
    }

    #[test]
    fn mismatch() {
        check("*", &vec![]);
        check(".", &vec![]);
        check("0", &vec![]);
        check("a", &vec![]);
    }
}

#[cfg(test)]
mod parenthese {
    use crate::io::source::SourceFile;
    use crate::lexing::lexer::{CodeLexer, Lexer};
    use crate::lexing::reader::source_reader::SourceReader;
    use crate::lexing::tests::create_lexer;
    use crate::token::collect::token_list::TokenList;
    use crate::token::collect::{parenthesis_close, parenthesis_open};
    use crate::token::{EndBlockToken, StartBlockToken, Tokens};

    use super::test_util::check;

    #[test]
    fn open() {
        check(" ( ", &vec![parenthesis_open()]);
    }

    #[test]
    fn close() {
        check(" ) ", &vec![parenthesis_close()]);
    }

    #[test]
    fn paired() {
        check(
            "(( ))",
            &vec![parenthesis_open(), parenthesis_open(), parenthesis_close(), parenthesis_close()],
        );
    }

    #[test]
    fn unbalanced() {
        check("(( )", &vec![parenthesis_open(), parenthesis_open(), parenthesis_close()]);
    }

    #[test]
    fn and_words() {
        check("(hello)", &vec![parenthesis_open()]);
    }
}

#[cfg(test)]
mod brackets {
    use crate::io::source::SourceFile;
    use crate::lexing::lexer::{CodeLexer, Lexer};
    use crate::lexing::reader::source_reader::SourceReader;
    use crate::lexing::tests::create_lexer;
    use crate::token::collect::token_list::TokenList;
    use crate::token::collect::{bracket_close, bracket_open};
    use crate::token::{EndBlockToken, StartBlockToken, Tokens};

    use super::test_util::check;

    #[test]
    fn open() {
        check(" [ ", &vec![bracket_open()]);
    }

    #[test]
    fn close() {
        check(" ] ", &vec![bracket_close()]);
    }

    #[test]
    fn paired() {
        check("[[ ]]", &vec![bracket_open(), bracket_open(), bracket_close(), bracket_close()]);
    }

    #[test]
    fn unbalanced() {
        check("[[ ]", &vec![bracket_open(), bracket_open(), bracket_close()]);
    }

    #[test]
    fn and_words() {
        check("[hello]", &vec![bracket_open()]);
    }
}

#[cfg(test)]
mod mixed {
    use crate::io::source::SourceFile;
    use crate::lexing::lexer::{CodeLexer, Lexer};
    use crate::lexing::reader::source_reader::SourceReader;
    use crate::lexing::tests::create_lexer;
    use crate::token::collect::token_list::TokenList;
    use crate::token::collect::{bracket_close, bracket_open, parenthesis_close, parenthesis_open};
    use crate::token::{EndBlockToken, StartBlockToken, Tokens};

    use super::test_util::check;

    #[test]
    fn parenthese_inside_brackets() {
        check(
            "[ ( ) ]",
            &vec![bracket_open(), parenthesis_open(), parenthesis_close(), bracket_close()],
        );
    }

    #[test]
    fn unbalanced_bracket_and_parenthese() {
        check("[)", &vec![bracket_open(), parenthesis_close()]);
    }
}
