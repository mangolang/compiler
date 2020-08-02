use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::lexing::lexer::Lexer;
use crate::lexing::reader::typ::{Reader, ReaderResult};
use crate::lexeme::{EndBlockLexeme, StartBlockLexeme, Lexeme};

lazy_static! {
    static ref NO_CODE_LINE_RE: Regex = Regex::new(r"^(#|\n)").unwrap();
    static ref INDENT_RE: Regex = Regex::new(r"^(\t| {4})").unwrap();
}

//TODO @mark: should not be called, or should be undone, after continuation (...)
//TODO @mark: should not be called, or should be undone, for empty lines

/// Process the indents at the start of a line, and add the lexemes to the Lexer.
pub fn lex_indents(reader: &mut impl Reader, lexer: &mut impl Lexer) {
    // Only try to lex indents if at the start of a line.
    if !lexer.is_at_indentable() {
        return;
    }

    // Skip if this is an empty or comment-only line.
    if let ReaderResult::Match(_) = reader.strip_peek(&*NO_CODE_LINE_RE) {
        return;
    }

    // Determine the indent of the line.
    let mut line_indent = 0;
    while let ReaderResult::Match(_) = reader.direct_match(&*INDENT_RE) {
        line_indent += 1;
    }

    // Determine the lexemes to create.
    let prev_indent = lexer.get_indent();
    for i in line_indent..prev_indent {
        lexer.add(Lexeme::EndBlock(EndBlockLexeme::new(true, false, )));
    }
    for i in prev_indent..line_indent {
        lexer.add(Lexeme::StartBlock(StartBlockLexeme::new()));
    }

    lexer.set_at_indentable(false);
    lexer.set_indent(line_indent);
}

#[cfg(test)]
mod indents {
    use crate::io::source::SourceFile;
    use crate::lexing::lexer::{CodeLexer, Lexer};
    use crate::lexing::reader::source_reader::SourceReader;
    use crate::lexing::tests::create_lexer;
    use crate::lexeme::{EndBlockLexeme, StartBlockLexeme, Lexeme};
    use crate::lexing::lexer::lexeme_collector::LexemeCollector;

    use super::lex_indents;

    fn check(initial_indent: u32, input: &str, expected: &[Lexeme]) {
        let expected: LexemeCollector = expected.into();
        let (source, mut reader, mut lexer) = create_lexer(input);
        lexer.set_indent(initial_indent);
        lex_indents(&mut reader, &mut lexer);
        assert_eq!(lexer.lexemes(), &expected);
    }

    #[test]
    fn increase() {
        check(
            0,
            "\t    hello",
            &[
                Lexeme::StartBlock(StartBlockLexeme::new()),
                Lexeme::StartBlock(StartBlockLexeme::new()),
            ],
        );
    }

    #[test]
    fn decrease_to_two() {
        check(3, "    \thello", &[Lexeme::EndBlock(EndBlockLexeme::new(true, false))]);
    }

    #[test]
    fn decrease_to_zero() {
        check(
            2,
            "hello",
            &[
                Lexeme::EndBlock(EndBlockLexeme::new(true, false)),
                Lexeme::EndBlock(EndBlockLexeme::new(true, false)),
            ],
        );
    }

    #[test]
    fn constant_two() {
        check(2, "\t    hello", &[]);
    }

    #[test]
    fn constant_zero() {
        check(0, "hello", &[]);
    }

    #[test]
    fn direct_comment() {
        check(0, "#hello", &[]);
    }

    #[test]
    fn indented_comment() {
        check(0, "    \t#hello", &[]);
    }

    #[test]
    fn empty_line() {
        check(0, "\n", &[]);
    }

    #[test]
    fn whitespace_line() {
        check(0, "\t    \n", &[]);
    }

    #[test]
    fn after_mismatch() {
        check(0, "word\t    \n", &[]);
    }

    //TODO @mark: check that the correct characters are stripped, especially for comments
}
