use mango::util::encdec::ToText;
use mango::util::format::to_double_quoted_str;
use mango::ast_full::BaseAST;

/// Closed collection of literal values
pub enum LiteralAST {
    Int(IntLiteralAST),
    Float(FloatLiteralAST),
    String(StringLiteralAST),
}

/// A literal integer value.
#[derive(Debug, PartialEq, Eq)]
pub struct IntLiteralAST {
    value: i64,
}

/// A literal float value.
#[derive(Debug, PartialEq)]
pub struct FloatLiteralAST {
    value: f64,
}

/// A literal text value.
#[derive(Debug, PartialEq, Eq)]
pub struct StringLiteralAST {
    value: String,
}

impl IntLiteralAST {
    pub fn new(value: i64) -> IntLiteralAST {
        IntLiteralAST { value }
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

impl Eq for FloatLiteralAST {}

impl BaseAST for IntLiteralAST {}
impl BaseAST for FloatLiteralAST {}
impl BaseAST for StringLiteralAST {}

// todo: test that printing and reparsing gives the same value?
