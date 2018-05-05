use mango::util::encdec::ToCode;
use std::io::Write;
use std::io;
use mango::util::encdec::ToText;
use mango::towasm::IWasm;
use std::fs::File;

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

impl IWasm for Module {}
