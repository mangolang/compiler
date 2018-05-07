use mango::towasm::collect::Type;
use mango::towasm::util::Name;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;
use std::io::Write;

pub struct Local {
    name: Name,
    typ: Type,
}

impl Local {
    pub fn new(name: Name, typ: Type) -> Self {
        Local { name, typ }
    }
}

impl Wasm for Local {
    fn as_wat(&self) -> String {
        format!("(local {} {})", self.name.as_wat(), self.typ.as_wat())
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}
