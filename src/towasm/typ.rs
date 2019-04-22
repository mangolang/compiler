use std::fs::File;
use std::io;
use std::io::Write;

use crate::util::strtype::Name;

/// WASM type
pub trait Wasm {
    fn as_wat(&self) -> String;
    fn write_wasm(&self, file: &mut File) -> io::Result<()>;
}

impl Wasm for Name {
    fn as_wat(&self) -> String {
        self.value()
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        //TODO @mark: that's not actually valid wasm, is it?
        file.write_all(format!(" {} ", self.value()).as_bytes())?;
        Ok(())
    }
}
