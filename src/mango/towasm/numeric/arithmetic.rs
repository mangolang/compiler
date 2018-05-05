use mango::towasm::collect::Type;
use mango::towasm::Wasm;
use mango::util::encdec::ToCode;
use mango::util::encdec::ToText;
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

impl ToText for Add {
    fn to_text(&self) -> String {
        " add ".to_owned()
        //        format!(" add ")
    }
}

impl ToCode for Add {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        file.write(b" add ")?;
        Ok(())
    }
}

impl Wasm for Add {}

pub struct Mul {
    typ: Type,
}

impl Mul {
    pub fn new(typ: Type) -> Self {
        Mul { typ }
    }
}

impl ToText for Mul {
    fn to_text(&self) -> String {
        " mul ".to_owned()
        //        format!(" add ")
    }
}

impl ToCode for Mul {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        file.write(b" mul ")?;
        Ok(())
    }
}

impl Wasm for Mul {}
