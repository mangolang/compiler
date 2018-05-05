use mango::util::encdec::ToText;
use mango::util::encdec::ToCode;
use mango::towasm::Numeric;
use mango::towasm::Control;
use std::fs::File;
use std::io;
use mango::towasm::Scope;

/// WASM type
//pub trait WASM: PartialEq + Eq + Hash + Debug + ToText {}
pub trait IWasm: ToText + ToCode {}

/// WASM collection
pub enum Wasm {
    Numeric(Numeric),
    Control(Control),
    Scope(Scope),
}

impl ToText for Wasm {
    fn to_text(&self) -> String {
        match self {
            Wasm::Numeric(op) => op.to_text(),
            Wasm::Control(op) => op.to_text(),
            Wasm::Scope(op) => op.to_text(),
        }
    }
}

impl ToCode for Wasm {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        match self {
            Wasm::Numeric(op) => op.to_code(file),
            Wasm::Control(op) => op.to_code(file),
            Wasm::Scope(op) => op.to_code(file),
        }
    }
}

impl IWasm for Wasm {}
