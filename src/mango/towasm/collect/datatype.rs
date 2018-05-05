
// The unsigned integers etc are not included.
// They're also not data types in wasm, the operation can choose signed/unsigned.
// The reason they're not included though, is that I don't expect to use them.
// If I do use them, I should probably add them here and break wasm 1-1.

use mango::util::encdec::ToText;
use mango::util::encdec::ToCode;
use std::io;
use std::fs::File;

pub enum Type {
    Int32,
    Int64,
    Float32,
    Float64,
    Bool,
}

impl ToText for Type {
    fn to_text(&self) -> String {
        match self {
            Type::Int32 => "i32",
            Type::Int64 => "i64",
            Type::Float32 => "f32",
            Type::Float64 => "f64",
            Type::Bool => "i32",
        }.to_owned()
    }
}

impl ToCode for Type {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}