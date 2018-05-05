use mango::towasm::collect::Type;
use mango::towasm::Wasm;
use mango::util::encdec::ToCode;
use mango::util::encdec::ToText;
use std::fs::File;
use std::io;
use std::io::Write;

pub struct Lt {
    typ: Type,
}

impl Lt {
    pub fn new(typ: Type) -> Self {
        Lt { typ }
    }
}

impl ToText for Lt {
    fn to_text(&self) -> String {
        " lt ".to_owned()
        //        format!(" add ")
    }
}

impl ToCode for Lt {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        file.write(b" lt ")?;
        Ok(())
    }
}

impl Wasm for Lt {}

pub struct Gt {
    typ: Type,
}

impl Gt {
    pub fn new(typ: Type) -> Self {
        Gt { typ }
    }
}

impl ToText for Gt {
    fn to_text(&self) -> String {
        " gt ".to_owned()
        //        format!(" add ")
    }
}

impl ToCode for Gt {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        file.write(b" gt ")?;
        Ok(())
    }
}

impl Wasm for Gt {}
