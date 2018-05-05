use mango::towasm::Wasm;
use mango::util::encdec::ToCode;
use mango::util::encdec::ToText;
use std::fs::File;
use std::io;
use std::io::Write;

pub struct Block {}

impl Block {
    pub fn new() -> Self {
        Block {}
    }
}

impl ToText for Block {
    fn to_text(&self) -> String {
        " call ".to_owned()
        //        format!(" add ")
    }
}

impl ToCode for Block {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        file.write(b" call ")?;
        Ok(())
    }
}

impl Wasm for Block {}

pub struct Branch {}

impl Branch {
    pub fn new() -> Self {
        Branch {}
    }
}

impl ToText for Branch {
    fn to_text(&self) -> String {
        " br ".to_owned()
        //        format!(" add ")
    }
}

impl ToCode for Branch {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        file.write(b" call ")?;
        Ok(())
    }
}

impl Wasm for Branch {}

pub struct BranchIf {}

impl BranchIf {
    pub fn new() -> Self {
        BranchIf {}
    }
}

impl ToText for BranchIf {
    fn to_text(&self) -> String {
        " br_if ".to_owned()
        //        format!(" add ")
    }
}

impl ToCode for BranchIf {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        file.write(b" call ")?;
        Ok(())
    }
}

impl Wasm for BranchIf {}
