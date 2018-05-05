use mango::util::encdec::ToText;
use mango::util::encdec::ToCode;
use std::fs::File;
use std::io::Write;
use std::io;
use mango::towasm::IWasm;

pub struct Add {
}

impl Add {
    pub fn new() -> Self {
        Add {}
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

impl IWasm for Add {}


pub struct Mul {
}

impl Mul {
    pub fn new() -> Self {
        Mul {}
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

impl IWasm for Mul {}
