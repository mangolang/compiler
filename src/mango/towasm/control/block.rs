use mango::util::encdec::ToCode;
use std::fs::File;
use std::io::Write;
use std::io;
use mango::util::encdec::ToText;

pub struct WasmBlock {}

impl WasmBlock {
    pub fn new() -> Self {
        WasmBlock {}
    }
}

impl ToText for WasmBlock {
    fn to_text(&self) -> String {
        " call ".to_owned()
//        format!(" add ")
    }
}

impl ToCode for WasmBlock {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        file.write(b" call ")?;
        Ok(())
    }
}
