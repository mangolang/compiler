use mango::towasm::collect::Type;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;
use std::io::Write;

pub struct Add {
    typ: Type,
}

impl Add {
    pub fn new(typ: Type) -> Self {
        Add { typ }
    }
}

impl Wasm for Add {
    fn as_wat(&self) -> String {
        " add ".to_owned()
        //        format!(" add ")
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" add ")?;
        Ok(())
    }
}

pub struct Mul {
    typ: Type,
}

impl Mul {
    pub fn new(typ: Type) -> Self {
        Mul { typ }
    }
}

impl Wasm for Mul {
    fn as_wat(&self) -> String {
        " mul ".to_owned()
        //        format!(" add ")
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" mul ")?;
        Ok(())
    }
}
