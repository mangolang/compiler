use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::lexing::lexer::Lexer;
use crate::lexing::reader::reader::{Reader, ReaderResult};
use crate::token::{EndBlockToken, StartBlockToken, Tokens};

lazy_static! {
    static ref NO_CODE_LINE_RE: Regex = Regex::new(r"^(#|\n)").unwrap();
    static ref INDENT_RE: Regex = Regex::new(r"^(\t| {4})").unwrap();
}

//TODO @mark: should not be called, or should be undone, after continuation (...)
//TODO @mark: should not be called, or should be undone, for empty lines

/// Process the indents at the start of a line, and add the tokens to the Lexer.
pub fn lex_indents(reader: &mut impl Reader, lexer: &mut impl Lexer) {

    // Skip if this is an empty or comment-only line.
    if let ReaderResult::Match(_) = reader.strip_peek(&*NO_CODE_LINE_RE) {
        return
    }

    // Determine the indent of the line.
    let mut line_indent = 0;
    while let ReaderResult::Match(_) = reader.direct_match(&*INDENT_RE) {
        line_indent += 1;
    }

    // Determine the tokens to create.
    let prev_indent = lexer.get_indent();
    for i in line_indent .. prev_indent {
        lexer.add(Tokens::EndBlock(EndBlockToken::new(true, false)));
    }
    for i in prev_indent .. line_indent {
        lexer.add(Tokens::StartBlock(StartBlockToken::new()));
    }

    lexer.set_indent(line_indent);
}

#[cfg(test)]
mod indents {
    use crate::io::source::SourceFile;
    use crate::lexing::reader::source_reader::SourceReader;
    use crate::lexing::lexer::{CodeLexer, Lexer};
    use crate::token::{StartBlockToken, Tokens, EndBlockToken};

    use super::lex_indents;
    use crate::lexing::tests::create_lexer;

    fn check(initial_indent: u32, input: &str, expected: &[Tokens]) {
        let (source, mut reader, mut lexer) = create_lexer(input);
        lexer.set_indent(initial_indent);
        lex_indents(&mut reader, &mut lexer);
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
