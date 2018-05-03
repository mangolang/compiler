use mango::ast_full::AST;
use mango::util::encdec::ToText;
use mango::util::format::to_double_quoted_str;
use mango::util::numtype::f64eq;
use mango::util::encdec::ToCode;
use mango::towasm::Wasm;
use std::fs::File;
use std::io::Write;
use std::io;

pub struct WasmAdd {
}

impl WasmAdd {
    pub fn new() -> Self {
        WasmAdd {}
    }
}

impl ToText for WasmAdd {
    fn to_text(&self) -> String {
        " add ".to_owned()
//        format!(" add ")
    }
}

impl ToCode for WasmAdd {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        file.write(b" add ")?;
        Ok(())
    }
}

impl Wasm for WasmAdd {}
