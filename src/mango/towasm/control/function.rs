use mango::towasm::Wasm;
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

impl Wasm for Call {
    fn as_wat(&self) -> String {
        " call ".to_owned()
        //        format!(" add ")
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" call ")?;
        Ok(())
    }
}

pub struct Return {}

impl Return {
    pub fn new() -> Self {
        Return {}
    }
}

impl Wasm for Return {
    fn as_wat(&self) -> String {
        " call ".to_owned()
        //        format!(" add ")
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" call ")?;
        Ok(())
    }
}
