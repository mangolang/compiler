use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::lexing::lexer::Lexer;
use crate::lexing::reader::reader::{Reader, ReaderResult};
use crate::token::{ParenthesisCloseToken, ParenthesisOpenToken, Tokens};
use crate::token::collect::{parenthesis_close, parenthesis_open, unlexable};

lazy_static! {
    static ref GROUPING_RE: Regex = Regex::new(r"^\(\)\[\]\{\}").unwrap();
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

    fn check(initial_indent: u32, input: &str, expected: &[Tokens]) {
        let (source, mut reader, mut lexer) = create_lexer(input);
        lexer.set_indent(initial_indent);
        lex_grouping(&mut reader, &mut lexer);
        assert_eq!(lexer.tokens(), expected);
    }

    #[test]
    fn increase() {
        check(0,
              "\t    hello",
              &vec![
                  Tokens::StartBlock(StartBlockToken::new()),
                  Tokens::StartBlock(StartBlockToken::new()),
              ]);
    }

    #[test]
    fn decrease_to_two() {
        check(3,
              "    \thello",
              &vec![
                  Tokens::EndBlock(EndBlockToken::new(true, false)),
              ]);
    }

    #[test]
    fn decrease_to_zero() {
        check(2,
              "hello",
              &vec![
                  Tokens::EndBlock(EndBlockToken::new(true, false)),
                  Tokens::EndBlock(EndBlockToken::new(true, false)),
              ]);
    }

    #[test]
    fn constant_two() {
        check(2,
              "\t    hello",
              &vec![]
        );
    }

    #[test]
    fn constant_zero() {
        check(0,
              "hello",
              &vec![]
        );
    }

    #[test]
    fn direct_comment() {
        check(0,
              "#hello",
              &vec![]
        );
    }

    #[test]
    fn indented_comment() {
        check(0,
              "    \t#hello",
              &vec![]
        );
    }

    #[test]
    fn empty_line() {
        check(0,
              "\n",
              &vec![]
        );
    }

    #[test]
    fn whitespace_line() {
        check(0,
              "\t    \n",
              &vec![]
        );
    }

    //TODO @mark: check that the correct characters are stripped, especially for comments
}
