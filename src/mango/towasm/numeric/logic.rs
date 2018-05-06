use mango::towasm::collect::Type;
use mango::towasm::Wasm;
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

impl Wasm for Lt {
    fn as_wat(&self) -> String {
        " lt ".to_owned()
        //        format!(" add ")
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" lt ")?;
        Ok(())
    }
}

pub struct Gt {
    typ: Type,
}

impl Gt {
    pub fn new(typ: Type) -> Self {
        Gt { typ }
    }
}

impl Wasm for Gt {
    fn as_wat(&self) -> String {
        " gt ".to_owned()
        //        format!(" add ")
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" gt ")?;
        Ok(())
    }
}
