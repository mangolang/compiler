use std::fs::File;
use std::io;

/// WASM type
pub trait Wasm {
    fn as_wat(&self) -> String;
    fn write_wasm(&self, file: &mut File) -> io::Result<()>;
}
