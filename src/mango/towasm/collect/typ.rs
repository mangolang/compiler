use mango::util::encdec::ToText;
use mango::util::encdec::ToCode;
use mango::towasm::WasmNumeric;
use mango::towasm::WasmControl;
use std::fs::File;
use std::io;

/// WASM type
//pub trait WASM: PartialEq + Eq + Hash + Debug + ToText {}
pub trait Wasm: ToText + ToCode {}

/// WASM collection
pub enum WasmOperation {
    Numeric(WasmNumeric),
    Control(WasmControl),
}

impl ToText for WasmOperation {
    fn to_text(&self) -> String {
        match self {
            WasmOperation::Numeric(op) => op.to_text(),
            WasmOperation::Control(op) => op.to_text(),
        }
    }
}

impl ToCode for WasmOperation {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        match self {
            WasmOperation::Numeric(op) => op.to_code(file),
            WasmOperation::Control(op) => op.to_code(file),
        }
    }
}

impl Wasm for WasmOperation {}

