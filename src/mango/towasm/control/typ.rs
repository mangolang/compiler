use mango::util::encdec::ToText;
use mango::towasm::IWasm;
use mango::util::encdec::ToCode;
use std::fs::File;
use std::io;
use mango::towasm::Return;
use mango::towasm::Call;

/// Control flow operations
// https://webassembly.github.io/spec/core/bikeshed/index.html#control-instructions%E2%91%A0
pub enum Control {
    Call(Call),
    Return(Return),
}

impl ToText for Control {
    fn to_text(&self) -> String {
        match self {
            Control::Call(op) => op.to_text(),
            Control::Return(op) => op.to_text(),
        }
    }
}

impl ToCode for Control {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        match self {
            Control::Call(op) => op.to_code(file),
            Control::Return(op) => op.to_code(file),
        }
    }
}

impl IWasm for Control {}
