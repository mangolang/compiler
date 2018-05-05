use mango::util::encdec::ToCode;
use std::fs::File;
use std::io::Write;
use std::io;
use mango::util::encdec::ToText;
use mango::towasm::IWasm;

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

impl IWasm for Block {}


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

impl IWasm for Branch {}


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

impl IWasm for BranchIf {}
