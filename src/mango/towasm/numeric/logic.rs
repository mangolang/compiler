use mango::towasm::collect::Type;
use mango::towasm::values::Expression;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;
use std::io::Write;

pub struct Gt {
    left: Box<Expression>,
    right: Box<Expression>,
}

impl Gt {
    pub fn new(left: Expression, right: Expression) -> Self {
        assert!(left.typ() == right.typ());
        Gt {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn typ(&self) -> &Type {
        &Type::Bool
    }
}

impl Wasm for Gt {
    fn as_wat(&self) -> String {
        format!(
            "{}\n{}\n{}.gt_s",
            self.left.as_wat(),
            self.right.as_wat(),
            self.typ().as_wat(),
        )
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" gt_s ")?;
        Ok(())
    }
}

pub struct Lt {
    left: Box<Expression>,
    right: Box<Expression>,
}

impl Lt {
    pub fn new(left: Expression, right: Expression) -> Self {
        assert!(left.typ() == right.typ());
        Lt {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn typ(&self) -> &Type {
        &Type::Bool
    }
}

impl Wasm for Lt {
    fn as_wat(&self) -> String {
        format!(
            "{}\n{}\n{}.lt_s",
            self.left.as_wat(),
            self.right.as_wat(),
            self.typ().as_wat(),
        )
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" lt+s ")?;
        Ok(())
    }
}
