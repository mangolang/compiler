use mango::towasm::collect::Type;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;
use std::io::Write;

pub struct Local {
    name: Name,
    typ: Type,
}

impl Local {
    pub fn new(typ: Type) -> Self {
        Local { typ }
    }
}

impl Wasm for Local {
    fn as_wat(&self) -> String {
        " add ".to_owned()
        //        format!(" add ")
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" add ")?;
        Ok(())
    }
}
