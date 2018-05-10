use mango::towasm::collect::Type;
use mango::towasm::numeric::Add;
use mango::towasm::numeric::Mul;
use mango::towasm::values::localvar::GetLocal;
use mango::towasm::values::Const;
use mango::towasm::values::Local;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;

/// A (combination of) operations that has an output value
pub enum Expression {
    Const(Const),
    Local(GetLocal),
    Mul(Mul),
    Add(Add),
}

impl Expression {
    pub fn typ(&self) -> &Type {
        match self {
            Expression::Const(op) => &op.typ,
            Expression::Local(op) => &op.typ(),
            Expression::Mul(op) => &op.typ(),
            Expression::Add(op) => &op.typ(),
        }
    }
}

impl Wasm for Expression {
    fn as_wat(&self) -> String {
        match self {
            Expression::Const(op) => op.as_wat(),
            Expression::Local(op) => op.as_wat(),
            Expression::Mul(op) => op.as_wat(),
            Expression::Add(op) => op.as_wat(),
        }
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}
