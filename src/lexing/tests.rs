use crate::common::error::ErrMsg;
use crate::io::source::SourceFile;
use crate::lexing::lexer::CodeLexer;
use crate::lexing::reader::source_reader::SourceReader;
use crate::token::collect::*;

use super::lex;

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
    assert_eq!(
        res,
        vec![
            parenthesis_open(),
            identifier("x")?,
            operator("*")?,
            identifier("x")?,
            operator("+")?,
            identifier("y")?,
            operator("*")?,
            identifier("y")?,
            parenthesis_close(),
        ]
    );
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
    assert_eq!(
        res,
        vec![
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
        ]
    );
    Ok(())
}

#[test]
fn lex_03() -> Result<(), ErrMsg> {
    let input = "(3*3 + 5.0 * 5.0)";
    let src = SourceFile::test(input);
    let res = lex(&src);
    assert_eq!(
        res,
        vec![
            parenthesis_open(),
            literal_int(3),
            operator("*")?,
            literal_int(3),
            operator("+")?,
            literal_real(5.0),
            operator("*")?,
            literal_real(5.0),
            parenthesis_close(),
        ]
    );
    Ok(())
}

#[test]
fn lex_04() -> Result<(), ErrMsg> {
    let input = "((3*3 + 5.0 * 5.0) == 25.0) == true";
    let src = SourceFile::test(input);
    let res = lex(&src);
    assert_eq!(
        res,
        vec![
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
        ]
    );
    Ok(())
}

#[test]
fn lex_05() -> Result<(), ErrMsg> {
    let input = "let mut x = [3, 5]\nprint(sqrt(x[0] * x[0] + x[1] * x[1]))";
    let src = SourceFile::test(input);
    let res = lex(&src);
    assert_eq!(
        res,
        vec![
            keyword_supported("let")?,
            keyword_supported("mut")?,
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
        ]
    );
    Ok(())
}

#[test]
fn lex_06() -> Result<(), ErrMsg> {
    let input = "
let mut seq = [1, 4, 5, 2, 3,]
let mut changed = true
while changed:
    changed = false
    for i in seq.indices().skip_last():
        if seq[i] > seq[i+1]:
            seq[i], seq[i+1] = seq[i+1], seq[i]
            changed = true
assert seq == [1, 2, 3, 4, 5]
";
    let src = SourceFile::test(input);
    let res = lex(&src);
    assert_eq!(
        res,
        vec![
            newline(),
            // let mut seq = [1, 4, 5, 2, 3,]
            keyword_supported("let")?,
            keyword_supported("mut")?,
            identifier("seq")?,
            association("=")?,
            bracket_open(),
            literal_int(1),
            comma(),
            literal_int(4),
            comma(),
            literal_int(5),
            comma(),
            literal_int(2),
            comma(),
            literal_int(3),
            comma(),
            bracket_close(),
            newline(),
            // let mut changed = true
            keyword_supported("let")?,
            keyword_supported("mut")?,
            identifier("changed")?,
            association("=")?,
            literal_bool(true),
            newline(),
            // while changed:
            keyword_supported("while")?,
            identifier("changed")?,
            colon(),
            newline(),
            // changed = false
            start_block(),
            identifier("changed")?,
            association("=")?,
            literal_bool(false),
            newline(),
            // for i in seq.indices().skip_last():
            keyword_supported("for")?,
            identifier("i")?,
            keyword_supported("in")?,
            identifier("seq")?,
            period(),
            identifier("indices")?,
            parenthesis_open(),
            parenthesis_close(),
            period(),
            identifier("skip_last")?,
            parenthesis_open(),
            parenthesis_close(),
            colon(),
            newline(),
            // if seq[i] > seq[i+1]:
            start_block(),
            keyword_supported("if")?,
            identifier("seq")?,
            bracket_open(),
            identifier("i")?,
            bracket_close(),
            operator(">")?,
            identifier("seq")?,
            bracket_open(),
            identifier("i")?,
            operator("+")?,
            literal_int(1),
            bracket_close(),
            colon(),
            newline(),
            // seq[i], seq[i+1] = seq[i+1], seq[i]
            start_block(),
            identifier("seq")?,
            bracket_open(),
            identifier("i")?,
            bracket_close(),
            comma(),
            identifier("seq")?,
            bracket_open(),
            identifier("i")?,
            operator("+")?,
            literal_int(1),
            bracket_close(),
            association("=")?,
            identifier("seq")?,
            bracket_open(),
            identifier("i")?,
            operator("+")?,
            literal_int(1),
            bracket_close(),
            comma(),
            identifier("seq")?,
            bracket_open(),
            identifier("i")?,
            bracket_close(),
            newline(),
            // changed = true
            identifier("changed")?,
            association("=")?,
            literal_bool(true),
            newline(),
            // assert seq == [1, 2, 3, 4, 5]
            end_block(),
            end_block(),
            end_block(),
            keyword_supported("assert")?,
            identifier("seq")?,
            operator("==")?,
            bracket_open(),
            literal_int(1),
            comma(),
            literal_int(2),
            comma(),
            literal_int(3),
            comma(),
            literal_int(4),
            comma(),
            literal_int(5),
            bracket_close(),
            newline(),
        ]
    );
    Ok(())
}
