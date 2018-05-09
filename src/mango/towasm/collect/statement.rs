use mango::towasm::control::Block;
use mango::towasm::control::Loop;
use mango::towasm::values::Assign;
use mango::towasm::values::DeclareLocal;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;

pub enum Statement {
    Local(DeclareLocal),
    Assign(Assign),
    Block(Block),
    Loop(Loop),
}

impl Wasm for Statement {
    fn as_wat(&self) -> String {
        match self {
            Statement::Local(op) => op.as_wat(),
            Statement::Assign(op) => op.as_wat(),
            Statement::Block(op) => op.as_wat(),
            Statement::Loop(op) => op.as_wat(),
        }
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}
