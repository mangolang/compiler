use mango::ast_full::AST;
use mango::util::encdec::ToText;
use mango::util::format::to_double_quoted_str;
use mango::util::numtype::f64eq;
use mango::util::encdec::ToCode;
use mango::towasm::Wasm;
use std::fs::File;
use std::io::Write;
use std::io;

// todo: call_indirect

pub struct WasmCall {}

impl WasmCall {
    pub fn new() -> Self {
        WasmCall {}
    }
}

impl ToText for WasmCall {
    fn to_text(&self) -> String {
        " call ".to_owned()
//        format!(" add ")
    }
}

impl ToCode for WasmCall {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        file.write(b" call ")?;
        Ok(())
    }
}

impl Wasm for WasmCall {}


pub struct WasmReturn {}

impl WasmReturn {
    pub fn new() -> Self {
        WasmReturn {}
    }
}

impl ToText for WasmReturn {
    fn to_text(&self) -> String {
        " call ".to_owned()
//        format!(" add ")
    }
}

impl ToCode for WasmReturn {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        file.write(b" call ")?;
        Ok(())
    }
}

impl Wasm for WasmReturn {}
