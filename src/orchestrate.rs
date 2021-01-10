use ::std::path::Path;

use crate::common::error::MangoResult;
use crate::io::disk::read;
use crate::io::source::SourceFile;
use crate::ir::IR;
use crate::parsing::parse::parse;
use crate::lexing::lex;

fn source_file_to_ir(source: SourceFile) -> MangoResult<IR> {
    let lexemes = lex(&source);
    let _parselets = parse(lexemes);
    unimplemented!()
}

pub fn mango_file_to_ir(pth: &Path) -> MangoResult<IR> {
    let source = read(pth)?;
    source_file_to_ir(source)
}

pub fn mango_str_to_ir(name: impl AsRef<str>, source: &str) -> MangoResult<IR> {
    let source = SourceFile::new(name, source);
    source_file_to_ir(source)
}

#[cfg(test)]
mod e2e {
    use super::*;

    #[test]
    fn from_str() {
        let ir = mango_str_to_ir("e2e_from_str", "\
let x = 3
let y = 4
let z = x * x + y * y
print(z)
").unwrap();
        assert_eq!(IR::Tmp, ir);
    }
}
