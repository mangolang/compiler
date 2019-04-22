use crate::towasm::collect::datatype::Value;
use crate::towasm::collect::Type;
use crate::towasm::values::Expression;
use crate::towasm::Wasm;
use std::fs::File;
use std::io;

pub struct Const {
    pub typ: Type,
    value: Value,
}

impl Const {
    pub fn new(typ: Type, value: Value) -> Box<Self> {
        assert!(value.is_type(&typ));
        Box::new(Const { typ, value })
    }
}

impl Wasm for Const {
    fn as_wat(&self) -> String {
        format!("{}.const {}", self.typ.as_wat(), self.value.as_wat())
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}

impl Expression for Const {
    fn typ(&self) -> &Type {
        &self.typ
    }
}
