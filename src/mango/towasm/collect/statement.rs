use mango::towasm::values::Assign;
use mango::towasm::values::DeclareLocal;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;

pub enum Statement {
    Local(DeclareLocal),
    Assign(Assign),
}

impl Wasm for Statement {
    fn as_wat(&self) -> String {
        match self {
            Statement::Local(decl) => decl.as_wat(),
            Statement::Assign(assign) => assign.as_wat(),
        }
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}
