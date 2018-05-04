use mango::util::encdec::ToText;
use mango::towasm::WasmAdd;
use mango::towasm::Wasm;
use mango::util::encdec::ToCode;
use std::fs::File;
use std::io;

/// Numeric operations
// https://webassembly.github.io/spec/core/bikeshed/index.html#numeric-instructions%E2%91%A0
pub enum WasmNumeric {
    Add(WasmAdd),
//    Sub(WasmSub),
//    Mul(WasmMul),
//    Div(WasmDiv),
}

impl ToText for WasmNumeric {
    fn to_text(&self) -> String {
        match self {
            WasmNumeric::Add(op) => op.to_text(),
        }
    }
}

impl ToCode for WasmNumeric {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        match self {
            WasmNumeric::Add(op) => op.to_code(file),
        }
    }
}

impl Wasm for WasmNumeric {}
