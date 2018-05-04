use mango::util::encdec::ToCode;
use std::fs::File;
use std::io::Write;
use std::io;
use mango::util::encdec::ToText;
use mango::towasm::Wasm;

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

impl Wasm for WasmBlock {}


pub struct WasmBranch {}

impl WasmBranch {
    pub fn new() -> Self {
        WasmBranch {}
    }
}

impl ToText for WasmBranch {
    fn to_text(&self) -> String {
        " br ".to_owned()
//        format!(" add ")
    }
}

impl ToCode for WasmBranch {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        file.write(b" call ")?;
        Ok(())
    }
}

impl Wasm for WasmBranch {}


pub struct WasmBranchIf {}

impl WasmBranchIf {
    pub fn new() -> Self {
        WasmBranchIf {}
    }
}

impl ToText for WasmBranchIf {
    fn to_text(&self) -> String {
        " br_if ".to_owned()
//        format!(" add ")
    }
}

impl ToCode for WasmBranchIf {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        file.write(b" call ")?;
        Ok(())
    }
}

impl Wasm for WasmBranchIf {}
