// The unsigned integers etc are not included.
// They're also not data types in wasm, the operation can choose signed/unsigned.
// The reason they're not included though, is that I don't expect to use them.
// If I do use them, I should probably add them here and break wasm 1-1.

use std::fs::File;
use std::io;

use crate::towasm::Wasm;

#[derive(PartialEq, Eq, Clone, Copy)]
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
        }
        .to_owned()
    }

    fn write_wasm(&self, _file: &mut File) -> io::Result<()> {
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
            Value::Int(_) => !matches!(typ, Type::Bool),
            Value::Float(_) => matches!(typ, Type::Float32 | Type::Float64),
        }
    }
}

impl Wasm for Value {
    fn as_wat(&self) -> String {
        match self {
            Value::Int(val) => format!("{}", val),
            Value::Float(val) => format!("{}", val),
        }
    }

    fn write_wasm(&self, _file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}
