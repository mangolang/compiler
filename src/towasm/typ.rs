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
        unimplemented!()
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(format!(" {} ", self.value()).as_bytes())?;
        Ok(())
    }
}
