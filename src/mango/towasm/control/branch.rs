use mango::towasm::collect::Statement;
use mango::towasm::util::Name;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;
use std::io::Write;

pub struct Branch {}

impl Branch {
    pub fn new() -> Self {
        Branch {}
    }
}

impl Wasm for Branch {
    fn as_wat(&self) -> String {
        " br ".to_owned()
        //        format!(" add ")
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" call ")?;
        Ok(())
    }
}

pub struct BranchIf {}

impl BranchIf {
    pub fn new() -> Self {
        BranchIf {}
    }
}

impl Wasm for BranchIf {
    fn as_wat(&self) -> String {
        " br_if ".to_owned()
        //        format!(" add ")
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" call ")?;
        Ok(())
    }
}