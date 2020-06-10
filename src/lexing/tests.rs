
use super::lex;
use crate::io::source::SourceFile;
use crate::lexing::reader::source_reader::SourceReader;
use crate::lexing::lexer::CodeLexer;


/// Create a set of source, reader and lexer for testing purposes.
pub fn create_lexer(txt: &str) -> (SourceFile, SourceReader, CodeLexer) {
    let source = SourceFile::test(txt);
    let reader = SourceReader::new(&source);
    let lexer = CodeLexer::new(source.len());
    (source, reader, lexer)
}

#[test]
fn lex_01() {
    let input = "(x * x + y * y)";
    let src = SourceFile::test(input);
    let res = lex(&src);
    dbg!(res);  //TODO @mark: TEMPORARY! REMOVE THIS!
    todo!()
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
    dbg!(res);  //TODO @mark: TEMPORARY! REMOVE THIS!
    todo!()
}
