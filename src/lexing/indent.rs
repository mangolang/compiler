use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::io::slice::SourceSlice;
use crate::lexeme::{EndBlockLexeme, Lexeme, StartBlockLexeme};
use crate::lexing::lexer::Lexer;
use crate::lexing::reader::typ::{Reader, ReaderResult};

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
    let mut source: SourceSlice = reader.source_at_current();
    while let ReaderResult::Match(more_src) = reader.direct_match(&*INDENT_RE) {
        line_indent += 1;
        //TODO @mark: need test coverage for this source slice thing
        source = source.join(more_src).unwrap()
    }

    // Determine the lexemes to create.
    let prev_indent = lexer.get_indent();
    for _ in line_indent..prev_indent {
        lexer.add(Lexeme::EndBlock(EndBlockLexeme::new(true, false, source.clone())));
    }
    for _ in prev_indent..line_indent {
        lexer.add(Lexeme::StartBlock(StartBlockLexeme::new(source.clone())));
    }

    lexer.set_at_indentable(false);
    lexer.set_indent(line_indent);
}

#[cfg(test)]
mod indents {
    use crate::io::slice::SourceSlice;
    use crate::lexeme::{EndBlockLexeme, Lexeme};
    use crate::lexeme::collect::for_test::*;
    use crate::lexing::lexer::Lexer;
    use crate::lexing::lexer::lexeme_collector::LexemeCollector;
    use crate::lexing::tests::create_lexer;

    use super::lex_indents;

    fn check(initial_indent: u32, input: &str, expected: &[Lexeme]) {
        let expected: LexemeCollector = expected.into();
        let (_source, mut reader, mut lexer) = create_lexer(input);
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
                start_block(),
                start_block(),
            ],
        );
    }

    #[test]
    fn decrease_to_two() {
        check(3, "    \thello", &[Lexeme::EndBlock(EndBlockLexeme::new(true, false, SourceSlice::mock()))]);
    }

    #[test]
    fn decrease_to_zero() {
        check(
            2,
            "hello",
            &[
                Lexeme::EndBlock(EndBlockLexeme::new(true, false, SourceSlice::mock())),
                Lexeme::EndBlock(EndBlockLexeme::new(true, false, SourceSlice::mock())),
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
