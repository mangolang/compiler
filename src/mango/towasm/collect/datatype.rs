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

pub enum Value {
    Int(i64),
    Float(f64),
}

impl Value {
    pub fn is_type(&self, typ: Type) -> bool {
        match self {
            Value::Int(_) => {
                match typ {
                    Type::Bool => false,
                    _ => true, // int value can be stored in int or float
                }
            }
            Value::Float(_) => match typ {
                Type::Float32 => true,
                Type::Float64 => true,
                _ => false,
            },
        }
    }
}

impl Wasm for Value {
    fn as_wat(&self) -> String {
        match self {
            Value::Int(val) => format!("{}", val),
            Value::Float(val) => format!("{}", val),
        }.to_owned()
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}
