// The unsigned integers etc are not included.
// They're also not data types in wasm, the operation can choose signed/unsigned.
// The reason they're not included though, is that I don't expect to use them.
// If I do use them, I should probably add them here and break wasm 1-1.

use mango::towasm::Wasm;
use std::fs::File;
use std::io;

pub enum Type {
    Int32,
    Int64,
    Float32,
    Float64,
    Bool,
}

impl Wasm for Type {
    fn as_wat(&self) -> String {
        match self {
            Type::Int32 => "i32",
            Type::Int64 => "i64",
            Type::Float32 => "f32",
            Type::Float64 => "f64",
            Type::Bool => "i32",
        }.to_owned()
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}
