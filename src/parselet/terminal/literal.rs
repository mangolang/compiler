use crate::lexeme::lexemes::LiteralLexeme;
use crate::parselet::{ExpressionParselets, Parselet};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct LiteralParselet {
    lexeme: LiteralLexeme,
}

impl LiteralParselet {
    pub fn new(lexeme: LiteralLexeme) -> Self {
        LiteralParselet { lexeme }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ArrayLiteralParselet {
    //TODO @mark: smallvec?
    lexemes: Vec<ExpressionParselets>,
}

impl ArrayLiteralParselet {
    pub fn new(lexemes: Vec<ExpressionParselets>) -> Self {
        ArrayLiteralParselet { lexemes }
    }
}

impl Parselet for LiteralParselet {}

//TODO @mark: TEMPORARY! REMOVE THIS!
//
// /// Closed collection of literal values
// #[derive(Debug, PartialEq, Eq, Hash)]
// pub enum LiteralParselet {
//     Int(IntLiteralParselet),
//     Float(FloatLiteralParselet),
//     String(StringLiteralParselet),
// }
//
// /// A literal integer value.
// #[derive(new, Debug, PartialEq, Eq, Hash)]
// pub struct IntLiteralParselet {
//     toke
// }
//
// /// A literal float value.
// #[derive(Debug, PartialEq, Eq, Hash)]
// pub struct FloatLiteralParselet {
//     value: f64eq,
// }
//
// /// A literal text value.
// #[derive(new, Debug, PartialEq, Eq, Hash)]
// pub struct StringLiteralParselet {
//     value: String,
// }
//
// impl FloatLiteralParselet {
//     pub fn new(value: f64) -> Self {
//         FloatLiteralParselet { value: f64eq::new(value) }
//     }
// }
//
// impl ToText for IntLiteralParselet {
//     fn to_text(&self) -> String {
//         format!(" {:} ", self.value)
//     }
// }
//
// impl ToText for FloatLiteralParselet {
//     fn to_text(&self) -> String {
//         format!(" {:.e} ", self.value)
//     }
// }
//
// impl ToText for StringLiteralParselet {
//     fn to_text(&self) -> String {
//         format!(" {:} ", to_double_quoted_str(&self.value))
//     }
// }
//
// impl ToText for LiteralParselet {
//     fn to_text(&self) -> String {
//         match self {
//             LiteralParselet::Int(val) => val.to_text(),
//             LiteralParselet::Float(val) => val.to_text(),
//             LiteralParselet::String(val) => val.to_text(),
//         }
//     }
// }
//
// impl Parselet for IntLiteralParselet {}
// impl Parselet for FloatLiteralParselet {}
// impl Parselet for StringLiteralParselet {}
