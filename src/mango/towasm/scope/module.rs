use mango::towasm::Wasm;
use mango::util::encdec::ToCode;
use mango::util::encdec::ToText;
use std::fs::File;
use std::io;
use std::io::Write;

pub struct Module {}

impl Module {
    pub fn new() -> Self {
        Module {}
    }
}

impl ToText for Module {
    fn to_text(&self) -> String {
        " module ".to_owned()
        //        format!(" add ")
    }
}

impl ToCode for Module {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        file.write(b" module ")?;
        Ok(())
    }
}

impl Wasm for Module {}
