use mango::towasm::collect::Type;
use mango::towasm::values::Expression;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;
use std::io::Write;

pub struct Add {
    left: Box<Expression>,
    right: Box<Expression>,
}

impl Add {
    pub fn new(left: Expression, right: Expression) -> Self {
        assert!(left.typ() == right.typ());
        Add {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn typ(&self) -> &Type {
        self.left.typ()
    }
}

impl Wasm for Add {
    fn as_wat(&self) -> String {
        format!(
            "{}\n{}\n{}.add",
            self.left.as_wat(),
            self.right.as_wat(),
            self.typ().as_wat(),
        )
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" add ")?;
        Ok(())
    }
}

pub struct Mul {
    left: Box<Expression>,
    right: Box<Expression>,
}

impl Mul {
    pub fn new(left: Expression, right: Expression) -> Self {
        assert!(left.typ() == right.typ());
        Mul {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn typ(&self) -> &Type {
        self.left.typ()
    }
}

impl Wasm for Mul {
    fn as_wat(&self) -> String {
        format!(
            "{}\n{}\n{}.mul",
            self.left.as_wat(),
            self.right.as_wat(),
            self.typ().as_wat(),
        )
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" mul ")?;
        Ok(())
    }
}
