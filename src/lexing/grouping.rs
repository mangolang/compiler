use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::lexing::lexer::Lexer;
use crate::lexing::reader::reader::{Reader, ReaderResult};
use crate::token::{ParenthesisCloseToken, ParenthesisOpenToken, Tokens};
use crate::token::collect::{parenthesis_close, parenthesis_open, unlexable};

lazy_static! {
    static ref GROUPING_RE: Regex = Regex::new(r"^[\(\)\[\]\{\}]").unwrap();
}

/// Lex any number of parentheses, braces and brackets, and add the tokens to the Lexer.
pub fn lex_grouping(reader: &mut impl Reader, lexer: &mut impl Lexer) {

    while let ReaderResult::Match(sym) = reader.strip_match(&*GROUPING_RE) {
        lexer.add(match sym.as_str() {
            "(" => parenthesis_open(),
            ")" => parenthesis_close(),
            "[" => unlexable("[ not yet implemented"),  //TODO @mark
            "]" => unlexable("] not yet implemented"),  //TODO @mark
            "{" => unlexable("{ not yet implemented"),  //TODO @mark
            "}" => unlexable("} not yet implemented"),  //TODO @mark
            _ => unreachable!(),
        });
    }
}

#[cfg(test)]
mod grouping {
    use crate::io::source::SourceFile;
    use crate::lexing::lexer::{CodeLexer, Lexer};
    use crate::lexing::reader::source_reader::SourceReader;
    use crate::lexing::tests::create_lexer;
    use crate::token::{EndBlockToken, StartBlockToken, Tokens};

    use super::lex_grouping;
    use crate::token::collect::{parenthesis_open, parenthesis_close};
    use crate::token::collect::token_list::TokenList;

    fn check(input: &str, expected: &[Tokens]) {
        let expected: TokenList = expected.into();
        let (source, mut reader, mut lexer) = create_lexer(input);
        lex_grouping(&mut reader, &mut lexer);
        assert_eq!(lexer.tokens(), &expected);
    }

    #[test]
    fn empty() {
        check("", &vec![]);
        check("    \t", &vec![]);
    }

    #[test]
    fn mismatch() {
        check("*", &vec![]);
        check(".", &vec![]);
        check("0", &vec![]);
        check("a", &vec![]);
    }

    #[test]
    fn parenthese_open() {
        check(" ( ", &vec![
            parenthesis_open(),
        ]);
    }

    #[test]
    fn parenthese_close() {
        check(" ) ", &vec![
            parenthesis_close(),
        ]);
    }

    #[test]
    fn paired_parenthese() {
        check("(( ))", &vec![
            parenthesis_open(),
            parenthesis_open(),
            parenthesis_close(),
            parenthesis_close(),
        ]);
    }

    #[test]
    fn parenthese_and_words() {
        check("(hello)", &vec![
            parenthesis_open(),
        ]);
    }
}
