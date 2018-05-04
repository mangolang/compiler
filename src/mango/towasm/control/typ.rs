use mango::util::encdec::ToText;
use mango::towasm::Wasm;
use mango::util::encdec::ToCode;
use std::fs::File;
use std::io;
use mango::towasm::WasmReturn;
use mango::towasm::WasmCall;

/// Control flow operations
// https://webassembly.github.io/spec/core/bikeshed/index.html#control-instructions%E2%91%A0
pub enum WasmControl {
    Call(WasmCall),
    Return(WasmReturn),
}

impl ToText for WasmControl {
    fn to_text(&self) -> String {
        match self {
            WasmControl::Call(op) => op.to_text(),
            WasmControl::Return(op) => op.to_text(),
        }
    }
}

impl ToCode for WasmControl {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        match self {
            WasmControl::Call(op) => op.to_code(file),
            WasmControl::Return(op) => op.to_code(file),
        }
    }
}

impl Wasm for WasmControl {}
