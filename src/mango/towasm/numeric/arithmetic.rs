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
    pub fn new(left: Box<Expression>, right: Box<Expression>) -> Box<Self> {
        assert!(left.typ() == right.typ());
        Box::new(Add { left: left, right: right })
    }

    pub fn typ(&self) -> &Type {
        self.left.typ()
    }
}

impl Wasm for Add {
    fn as_wat(&self) -> String {
        format!("{}\n{}\n{}.add", self.left.as_wat(), self.right.as_wat(), self.typ().as_wat(),)
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" add ")?;
        Ok(())
    }
}

impl Expression for Add {
    fn typ(&self) -> &Type {
        self.typ()
    }
}

pub struct Mul {
    left: Box<Expression>,
    right: Box<Expression>,
}

impl Mul {
    pub fn new(left: Box<Expression>, right: Box<Expression>) -> Box<Self> {
        assert!(left.typ() == right.typ());
        Box::new(Mul { left: left, right: right })
    }

    pub fn typ(&self) -> &Type {
        self.left.typ()
    }
}

impl Wasm for Mul {
    fn as_wat(&self) -> String {
        format!("{}\n{}\n{}.mul", self.left.as_wat(), self.right.as_wat(), self.typ().as_wat(),)
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" mul ")?;
        Ok(())
    }
}

impl Expression for Mul {
    fn typ(&self) -> &Type {
        self.typ()
    }
}
