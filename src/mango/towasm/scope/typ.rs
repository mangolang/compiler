use mango::util::encdec::ToText;
use mango::towasm::IWasm;
use mango::util::encdec::ToCode;
use std::fs::File;
use std::io;
use mango::towasm::Return;
use mango::towasm::Call;
use mango::towasm::Module;

/// Scope and visibility (module)
pub enum Scope {
    Module(Module),
}

impl ToText for Scope {
    fn to_text(&self) -> String {
        match self {
            Scope::Module(op) => op.to_text(),
        }
    }
}

impl ToCode for Scope {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        match self {
            Scope::Module(op) => op.to_code(file),
        }
    }
}

impl IWasm for Scope {}
