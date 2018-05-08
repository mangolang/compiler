use mango::towasm::collect::datatype::Value;
use mango::towasm::collect::Type;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;
use std::io::Write;

pub struct Const {
    typ: Type,
    value: Value,
}

impl Const {
    pub fn new(typ: Type, value: Value) -> Self {
        assert!(value.is_type(&typ));
        Const { typ, value }
    }
}

impl Wasm for Const {
    fn as_wat(&self) -> String {
        format!("({}.const {})", self.typ.as_wat(), self.value.as_wat())
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}