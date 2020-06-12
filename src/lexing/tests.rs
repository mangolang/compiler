
use super::lex;
use crate::io::source::SourceFile;
use crate::lexing::reader::source_reader::SourceReader;
use crate::lexing::lexer::CodeLexer;
use crate::token::collect::*;
use crate::common::error::ErrMsg;


/// Create a set of source, reader and lexer for testing purposes.
pub fn create_lexer(txt: &str) -> (SourceFile, SourceReader, CodeLexer) {
    let source = SourceFile::test(txt);
    let reader = SourceReader::new(&source);
    let lexer = CodeLexer::new(source.len());
    (source, reader, lexer)
}

#[test]
fn lex_01() -> Result<(), ErrMsg>{
    let input = "(x * x + y * y)";
    let src = SourceFile::test(input);
    let res = lex(&src);
    assert_eq!(res, vec![
        parenthesis_open(),
        identifier("x")?,
        operator("*")?,
        identifier("x")?,
        operator("+")?,
        identifier("y")?,
        operator("*")?,
        identifier("y")?,
        parenthesis_close(),
    ]);
    Ok(())
}

#[test]
fn lex_02() {
    let input = "
    (
        x * x + ...
        y * y
    )
    ";
    let src = SourceFile::test(input);
    let res = lex(&src);
    //TODO @mark
}

#[test]
fn lex_03() {
    let input = "(3*3 + 5 * 5)";
    let src = SourceFile::test(input);
    let res = lex(&src);
    //TODO @mark
}