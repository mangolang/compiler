use mango::util::encdec::ToText;
use mango::towasm::instructions::WasmAdd;
use mango::towasm::Wasm;
use mango::util::encdec::ToCode;
use std::fs::File;
use std::io;

/// The numeric instructions
// https://webassembly.github.io/spec/core/bikeshed/index.html#numeric-instructions%E2%91%A0
pub enum WasmNumericInstruction {
    Add(WasmAdd),
//    Sub(WasmSub),
//    Mul(WasmMul),
//    Div(WasmDiv),
}

impl ToText for WasmNumericInstruction {
    fn to_text(&self) -> String {
        match self {
            WasmNumericInstruction::Add(op) => op.to_text(),
        }
    }
}

impl ToCode for WasmNumericInstruction {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        match self {
            WasmNumericInstruction::Add(op) => op.to_code(file),
        }
    }
}

impl Wasm for WasmNumericInstruction {}
