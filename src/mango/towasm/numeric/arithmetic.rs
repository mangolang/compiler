use mango::towasm::collect::Type;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;
use std::io::Write;

#[derive(new)]
pub struct Add {
    pub typ: Type,
}

impl Wasm for Add {
    fn as_wat(&self) -> String {
        " add ".to_owned()
        //        format!(" add ")
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" add ")?;
        Ok(())
    }
}

#[derive(new)]
pub struct Mul {
    pub typ: Type,
}

impl Wasm for Mul {
    fn as_wat(&self) -> String {
        " mul ".to_owned()
        //        format!(" add ")
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" mul ")?;
        Ok(())
    }
}
