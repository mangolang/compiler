
use super::lex;
use crate::io::source::SourceFile;
use crate::lexing::reader::source_reader::SourceReader;
use crate::lexing::lexer::CodeLexer;
use crate::token::collect::*;
use crate::common::error::ErrMsg;

// use ::indoc::indoc;

/// Create a set of source, reader and lexer for testing purposes.
pub fn create_lexer(txt: &str) -> (SourceFile, SourceReader, CodeLexer) {
    let source = SourceFile::test(txt);
    let reader = SourceReader::new(&source);
    let lexer = CodeLexer::new(source.len());
    (source, reader, lexer)
}

#[test]
fn lex_01() -> Result<(), ErrMsg> {
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
fn lex_02() -> Result<(), ErrMsg> {
    // let input = indoc!("(
    //     x * x + ...
    //     y * y
    // )");
    let input = "(
    x * x + ...
    y * y
)";
    let src = SourceFile::test(input);
    let res = lex(&src);
    assert_eq!(res, vec![
        parenthesis_open(),
        newline(),
        start_block(),
        identifier("x")?,
        operator("*")?,
        identifier("x")?,
        operator("+")?,
        ellipsis(),
        newline(),
        identifier("y")?,
        operator("*")?,
        identifier("y")?,
        newline(),
        end_block(),
        parenthesis_close(),
    ]);
    Ok(())
}

#[test]
fn lex_03() -> Result<(), ErrMsg> {
    let input = "(3*3 + 5.0 * 5.0)";
    let src = SourceFile::test(input);
    let res = lex(&src);
    assert_eq!(res, vec![
        parenthesis_open(),
        literal_int(3),
        operator("*")?,
        literal_int(3),
        operator("+")?,
        literal_real(5.0),
        operator("*")?,
        literal_real(5.0),
        parenthesis_close(),
    ]);
    Ok(())
}

#[test]
fn lex_04() -> Result<(), ErrMsg> {
    let input = "((3*3 + 5.0 * 5.0) == 25.0) == true";
    let src = SourceFile::test(input);
    let res = lex(&src);
    assert_eq!(res, vec![
        parenthesis_open(),
        parenthesis_open(),
        literal_int(3),
        operator("*")?,
        literal_int(3),
        operator("+")?,
        literal_real(5.0),
        operator("*")?,
        literal_real(5.0),
        parenthesis_close(),
        operator("==")?,
        literal_real(25.0),
        parenthesis_close(),
        operator("==")?,
        literal_bool(true),
    ]);
    Ok(())
}

#[test]
fn lex_05() -> Result<(), ErrMsg> {
    let input = "let mut x = [3, 5]\nprint(sqrt(x[0] * x[0] + x[1] * x[1]))";
    let src = SourceFile::test(input);
    let res = lex(&src);
    assert_eq!(res, vec![
        keyword("let")?,
        keyword("mut")?,
        identifier("x")?,
        association("=")?,
        bracket_open(),
        literal_int(3),
        comma(),
        literal_int(5),
        bracket_close(),
        newline(),
        identifier("print")?,
        parenthesis_open(),
        identifier("sqrt")?,
        parenthesis_open(),
        identifier("x")?,
        bracket_open(),
        literal_int(0),
        bracket_close(),
        operator("*")?,
        identifier("x")?,
        bracket_open(),
        literal_int(0),
        bracket_close(),
        operator("+")?,
        identifier("x")?,
        bracket_open(),
        literal_int(1),
        bracket_close(),
        operator("*")?,
        identifier("x")?,
        bracket_open(),
        literal_int(1),
        bracket_close(),
        parenthesis_close(),
        parenthesis_close(),
    ]);
    Ok(())
}

//TODO @mark:
// #[test]
fn lex_10() -> Result<(), ErrMsg> {
    let input = "
let mut seq = [1, 4, 5, 2, 3]
let mut changed = true
while changed:
    changed = false
    for i in range(len(seq) - 1):
        if seq[i] > seq[i+1]:
            seq[i], seq[i+1] = seq[i+1], seq[i]
            changed = true
assert seq == [1, 2, 3, 4, 5]";
    let src = SourceFile::test(input);
    let res = lex(&src);
    assert_eq!(res, vec![
        parenthesis_open(),
        literal_int(3),
        operator("*")?,
        literal_int(3),
        operator("+")?,
        literal_real(5.0),
        operator("*")?,
        literal_real(5.0),
        parenthesis_close(),
    ]);
    Ok(())
}
