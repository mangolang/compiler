use mango::ast_full::AST;
use mango::util::encdec::ToText;
use mango::util::format::to_double_quoted_str;

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
        return IntLiteralAST { value };
    }
}

impl ToText for IntLiteralAST {
    fn to_text(&self) -> String {
        return format!(" {:} ", self.value);
    }
}

impl ToText for FloatLiteralAST {
    fn to_text(&self) -> String {
        return format!(" {:.e} ", self.value);
    }
}

impl ToText for StringLiteralAST {
    fn to_text(&self) -> String {
        return format!(" {:} ", to_double_quoted_str(&self.value));
    }
}

impl AST for IntLiteralAST {}
impl AST for FloatLiteralAST {}
impl AST for StringLiteralAST {}

// todo: test that printing and reparsing gives the same value?
