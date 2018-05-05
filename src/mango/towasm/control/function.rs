use mango::towasm::Wasm;
use mango::util::encdec::ToCode;
use mango::util::encdec::ToText;
use std::fs::File;
use std::io;
use std::io::Write;

// todo: call_indirect

pub struct Call {}

impl Call {
    pub fn new() -> Self {
        Call {}
    }
}

impl ToText for Call {
    fn to_text(&self) -> String {
        " call ".to_owned()
        //        format!(" add ")
    }
}

impl ToCode for Call {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        file.write(b" call ")?;
        Ok(())
    }
}

impl Wasm for Call {}

pub struct Return {}

impl Return {
    pub fn new() -> Self {
        Return {}
    }
}

impl ToText for Return {
    fn to_text(&self) -> String {
        " call ".to_owned()
        //        format!(" add ")
    }
}

impl ToCode for Return {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        file.write(b" call ")?;
        Ok(())
    }
}

impl Wasm for Return {}
