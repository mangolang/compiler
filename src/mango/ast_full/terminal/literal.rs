use mango::ast_full::BaseAST;
use mango::util::encdec::ToText;
use mango::util::format::to_double_quoted_str;
use mango::util::numtype::f64eq;

/// Closed collection of literal values
pub enum LiteralAST {
    Int(IntLiteralAST),
    Float(FloatLiteralAST),
    String(StringLiteralAST),
}

/// A literal integer value.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct IntLiteralAST {
    value: i64,
}

/// A literal float value.
#[derive(Debug, PartialEq, Hash)]
pub struct FloatLiteralAST {
    value: f64eq,
}

/// A literal text value.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct StringLiteralAST {
    value: String,
}

impl IntLiteralAST {
    pub fn new(value: i64) -> Self {
        IntLiteralAST { value }
    }
}

impl FloatLiteralAST {
    pub fn new(value: f64) -> Self {
        FloatLiteralAST {
            value: f64eq::new(value),
        }
    }
}

impl StringLiteralAST {
    pub fn new(value: String) -> Self {
        StringLiteralAST { value }
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

// TODO: I need to make sure than NaN == NaN to do this legitimately
impl Eq for FloatLiteralAST {}

impl BaseAST for IntLiteralAST {}
impl BaseAST for FloatLiteralAST {}
impl BaseAST for StringLiteralAST {}
