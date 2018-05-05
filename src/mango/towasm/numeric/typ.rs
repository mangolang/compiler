use mango::util::encdec::ToText;
use mango::towasm::Add;
use mango::towasm::Mul;
use mango::towasm::IWasm;
use mango::util::encdec::ToCode;
use std::fs::File;
use std::io;

/// Numeric operations
// https://webassembly.github.io/spec/core/bikeshed/index.html#numeric-instructions%E2%91%A0
pub enum Numeric {
    Add(Add),
    Mul(Mul),
//    Sub(WasmSub),
//    Mul(WasmMul),
//    Div(WasmDiv),
}

impl ToText for Numeric {
    fn to_text(&self) -> String {
        match self {
            Numeric::Add(op) => op.to_text(),
            Numeric::Mul(op) => op.to_text(),
        }
    }
}

impl ToCode for Numeric {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        match self {
            Numeric::Add(op) => op.to_code(file),
            Numeric::Mul(op) => op.to_code(file),
        }
    }
}

impl IWasm for Numeric {}
