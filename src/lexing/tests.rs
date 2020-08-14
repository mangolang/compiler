use crate::io::source::SourceFile;
use crate::lexeme::collect::for_test::*;
use crate::lexing::lexer::CodeLexer;
use crate::lexing::reader::source_reader::SourceReader;

use super::lex;

// use ::indoc::indoc;

/// Create a set of source, reader and lexer for testing purposes.
pub fn create_lexer(txt: &str) -> (SourceFile, SourceReader, CodeLexer) {
    let source = SourceFile::mock(txt);
    let reader = SourceReader::new(&source);
    let lexer = CodeLexer::new(source.len());
    (source, reader, lexer)
}

#[test]
fn lex_01() {
    let input = "(x * x + y * y)";
    let src = SourceFile::mock(input);
    let res = lex(&src);
    assert_eq!(
        res,
        vec![
            parenthesis_open(),
            identifier("x").into(),
            operator("*").into(),
            identifier("x").into(),
            operator("+").into(),
            identifier("y").into(),
            operator("*").into(),
            identifier("y").into(),
            parenthesis_close(),
        ].into()
    );
}

#[test]
fn lex_02() {
    // let input = indoc!("(
    //     x * x + ...
    //     y * y
    // )");
    let input = "(
    x * x + ...
    y * y
)";
    let src = SourceFile::mock(input);
    let res = lex(&src);
    assert_eq!(
        res,
        vec![
            parenthesis_open(),
            newline(),
            start_block(),
            identifier("x").into(),
            operator("*").into(),
            identifier("x").into(),
            operator("+").into(),
            ellipsis(),
            newline(),
            identifier("y").into(),
            operator("*").into(),
            identifier("y").into(),
            newline(),
            end_block(),
            parenthesis_close(),
        ].into()
    );
}

#[test]
fn lex_03() {
    let input = "(3*3 + 5.0 * 5.0)";
    let src = SourceFile::mock(input);
    let res = lex(&src);
    assert_eq!(
        res,
        vec![
            parenthesis_open(),
            literal_int(3).into(),
            operator("*").into(),
            literal_int(3).into(),
            operator("+").into(),
            literal_real(5.0).into(),
            operator("*").into(),
            literal_real(5.0).into(),
            parenthesis_close(),
        ].into()
    );
}

#[test]
fn lex_04() {
    let input = "((3*3 + 5.0 * 5.0) == 25.0) == true";
    let src = SourceFile::mock(input);
    let res = lex(&src);
    assert_eq!(
        res,
        vec![
            parenthesis_open(),
            parenthesis_open(),
            literal_int(3).into(),
            operator("*").into(),
            literal_int(3).into(),
            operator("+").into(),
            literal_real(5.0).into(),
            operator("*").into(),
            literal_real(5.0).into(),
            parenthesis_close(),
            operator("==").into(),
            literal_real(25.0).into(),
            parenthesis_close(),
            operator("==").into(),
            literal_bool(true).into(),
        ].into()
    );
}

#[test]
fn lex_05() {
    let input = "let mut x = [3, 5]\nprint(sqrt(x[0] * x[0] + x[1] * x[1]))";
    let src = SourceFile::mock(input);
    let res = lex(&src);
    assert_eq!(
        res,
        vec![
            keyword_supported("let"),
            keyword_supported("mut"),
            identifier("x").into(),
            association("=").into(),
            bracket_open(),
            literal_int(3).into(),
            comma(),
            literal_int(5).into(),
            bracket_close(),
            newline(),
            identifier("print").into(),
            parenthesis_open(),
            identifier("sqrt").into(),
            parenthesis_open(),
            identifier("x").into(),
            bracket_open(),
            literal_int(0).into(),
            bracket_close(),
            operator("*").into(),
            identifier("x").into(),
            bracket_open(),
            literal_int(0).into(),
            bracket_close(),
            operator("+").into(),
            identifier("x").into(),
            bracket_open(),
            literal_int(1).into(),
            bracket_close(),
            operator("*").into(),
            identifier("x").into(),
            bracket_open(),
            literal_int(1).into(),
            bracket_close(),
            parenthesis_close(),
            parenthesis_close(),
        ].into()
    );
}

#[test]
fn lex_06() {
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
    let src = SourceFile::mock(input);
    let res = lex(&src);
    assert_eq!(
        res,
        vec![
            newline(),
            // let mut seq = [1, 4, 5, 2, 3,]
            keyword_supported("let"),
            keyword_supported("mut"),
            identifier("seq").into(),
            association("=").into(),
            bracket_open(),
            literal_int(1).into(),
            comma(),
            literal_int(4).into(),
            comma(),
            literal_int(5).into(),
            comma(),
            literal_int(2).into(),
            comma(),
            literal_int(3).into(),
            comma(),
            bracket_close(),
            newline(),
            // let mut changed = true
            keyword_supported("let"),
            keyword_supported("mut"),
            identifier("changed").into(),
            association("=").into(),
            literal_bool(true).into(),
            newline(),
            // while changed:
            keyword_supported("while"),
            identifier("changed").into(),
            colon(),
            newline(),
            // changed = false
            start_block(),
            identifier("changed").into(),
            association("=").into(),
            literal_bool(false).into(),
            newline(),
            // for i in seq.indices().skip_last():
            keyword_supported("for"),
            identifier("i").into(),
            keyword_supported("in"),
            identifier("seq").into(),
            period(),
            identifier("indices").into(),
            parenthesis_open(),
            parenthesis_close(),
            period(),
            identifier("skip_last").into(),
            parenthesis_open(),
            parenthesis_close(),
            colon(),
            newline(),
            // if seq[i] > seq[i+1]:
            start_block(),
            keyword_supported("if"),
            identifier("seq").into(),
            bracket_open(),
            identifier("i").into(),
            bracket_close(),
            operator(">").into(),
            identifier("seq").into(),
            bracket_open(),
            identifier("i").into(),
            operator("+").into(),
            literal_int(1).into(),
            bracket_close(),
            colon(),
            newline(),
            // seq[i], seq[i+1] = seq[i+1], seq[i]
            start_block(),
            identifier("seq").into(),
            bracket_open(),
            identifier("i").into(),
            bracket_close(),
            comma(),
            identifier("seq").into(),
            bracket_open(),
            identifier("i").into(),
            operator("+").into(),
            literal_int(1).into(),
            bracket_close(),
            association("=").into(),
            identifier("seq").into(),
            bracket_open(),
            identifier("i").into(),
            operator("+").into(),
            literal_int(1).into(),
            bracket_close(),
            comma(),
            identifier("seq").into(),
            bracket_open(),
            identifier("i").into(),
            bracket_close(),
            newline(),
            // changed = true
            identifier("changed").into(),
            association("=").into(),
            literal_bool(true).into(),
            newline(),
            // assert seq == [1, 2, 3, 4, 5]
            end_block(),
            end_block(),
            end_block(),
            keyword_supported("assert"),
            identifier("seq").into(),
            operator("==").into(),
            bracket_open(),
            literal_int(1).into(),
            comma(),
            literal_int(2).into(),
            comma(),
            literal_int(3).into(),
            comma(),
            literal_int(4).into(),
            comma(),
            literal_int(5).into(),
            bracket_close(),
            newline(),
        ].into()
    );
}
