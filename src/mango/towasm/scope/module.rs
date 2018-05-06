use mango::towasm::scope::Function;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;
use std::io::Write;

pub struct Module {
    functions: Vec<Function>,
}

impl Module {
    pub fn new(functions: Vec<Function>) -> Self {
        Module { functions }
    }
}

impl Wasm for Module {
    fn as_wat(&self) -> String {
        " module ".to_owned()
        //        format!(" add ")
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" module ")?;
        Ok(())
    }
}
