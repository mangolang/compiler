use std::fs::File;
use std::io;
use std::io::Write;

use crate::towasm::collect::Type;
use crate::towasm::values::Expression;
use crate::towasm::Wasm;

pub struct Gt {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Gt {
    pub fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Box<Self> {
        assert!(left.typ() == right.typ());
        Box::new(Gt { left, right })
    }
}

impl Wasm for Gt {
    fn as_wat(&self) -> String {
        format!("{}\n{}\n{}.gt_s", self.left.as_wat(), self.right.as_wat(), self.typ().as_wat(),)
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write_all(b" gt_s ")?;
        Ok(())
    }
}

impl Expression for Gt {
    fn typ(&self) -> &Type {
        &Type::Bool
    }
}

pub struct Lt {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Lt {
    pub fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Box<Self> {
        assert!(left.typ() == right.typ());
        Box::new(Lt { left, right })
    }

    pub fn typ(&self) -> &Type {
        &Type::Bool
    }
}

impl Wasm for Lt {
    fn as_wat(&self) -> String {
        format!("{}\n{}\n{}.lt_s", self.left.as_wat(), self.right.as_wat(), self.typ().as_wat(),)
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write_all(b" lt+s ")?;
        Ok(())
    }
}

impl Expression for Lt {
    fn typ(&self) -> &Type {
        &Type::Bool
    }
}
