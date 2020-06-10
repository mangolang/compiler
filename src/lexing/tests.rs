
use super::lex;
use crate::io::source::SourceFile;

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
