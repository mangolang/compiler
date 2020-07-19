use derive_new::new;

use crate::parselet::AST;
use crate::util::encdec::ToText;
use crate::util::format::to_double_quoted_str;
use crate::util::numtype::f64eq;

/// Closed collection of literal values
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum LiteralAST {
    Int(IntLiteralAST),
    Float(FloatLiteralAST),
    String(StringLiteralAST),
}

/// A literal integer value.
#[derive(new, Debug, PartialEq, Eq, Hash)]
pub struct IntLiteralAST {
    value: i64,
}

/// A literal float value.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FloatLiteralAST {
    value: f64eq,
}

/// A literal text value.
#[derive(new, Debug, PartialEq, Eq, Hash)]
pub struct StringLiteralAST {
    value: String,
}

impl FloatLiteralAST {
    pub fn new(value: f64) -> Self {
        FloatLiteralAST { value: f64eq::new(value) }
    }
}

impl ToText for IntLiteralAST {
    fn to_text(&self) -> String {
        format!(" {:} ", self.value)
    }
}

impl ToText for FloatLiteralAST {
    fn to_text(&self) -> String {
        format!(" {:.e} ", self.value)
    }
}

impl ToText for StringLiteralAST {
    fn to_text(&self) -> String {
        format!(" {:} ", to_double_quoted_str(&self.value))
    }
}

impl ToText for LiteralAST {
    fn to_text(&self) -> String {
        match self {
            LiteralAST::Int(val) => val.to_text(),
            LiteralAST::Float(val) => val.to_text(),
            LiteralAST::String(val) => val.to_text(),
        }
    }
}

impl AST for IntLiteralAST {}
impl AST for FloatLiteralAST {}
impl AST for StringLiteralAST {}
