use mango::towasm::control::Branch;
use mango::towasm::control::Group;
use mango::towasm::numeric::Add;
use mango::towasm::numeric::Gt;
use mango::towasm::numeric::Lt;
use mango::towasm::numeric::Mul;
use mango::towasm::scope::Module;
use std::fs::File;
use std::io;

/// WASM type
pub trait Wasm {
    fn as_wat(&self) -> String;
    fn write_wasm(&self, file: &mut File) -> io::Result<()>;
}
