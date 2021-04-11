use ::std::hash;

use ::ustr::ustr;
use ::ustr::Ustr;

use crate::common::codeparts::eqfloat::f64eq;
use crate::common::debug::ToText;
use crate::io::slice::{SourceLocation, SourceSlice};
use crate::lexeme::Lexeme;

#[derive(Debug, Eq, Clone)]
pub struct TextLiteralLexeme {
    text: Ustr,
    source: SourceSlice,
}

impl TextLiteralLexeme {
    pub fn new(text: impl AsRef<str>, source: SourceSlice) -> Self {
        TextLiteralLexeme {
            text: ustr(text.as_ref()),
            source,
        }
    }
}

/// A literal, like 9 or "hello".
/// Note that null does not exist.
#[derive(Debug, Eq, Clone)]
pub enum LiteralLexeme {
    //TODO @mark: to Ustr for cheaper clone:
    Text(TextLiteralLexeme),
    Int(i64, SourceSlice),
    Real(f64eq, SourceSlice),
    Boolean(bool, SourceSlice),
}

impl LiteralLexeme {
    pub fn new_text(text: impl AsRef<str>, source: SourceSlice) -> Self {
        LiteralLexeme::Text(TextLiteralLexeme::new(text, source))
    }
}

impl From<LiteralLexeme> for Lexeme {
    fn from(literal: LiteralLexeme) -> Self {
        Lexeme::Literal(literal)
    }
}

impl PartialEq for TextLiteralLexeme {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl hash::Hash for TextLiteralLexeme {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.text.hash(state)
    }
}

impl SourceLocation for TextLiteralLexeme {
    fn source(&self) -> &SourceSlice {
        &self.source
    }
}

impl ToText for TextLiteralLexeme {
    fn to_text(&self) -> String {
        //TODO @mark: escaping
        format!("\"{}\"", self.text)
    }
}

impl PartialEq for LiteralLexeme {
    fn eq(&self, other: &Self) -> bool {
        use LiteralLexeme::*;
        match (self, other) {
            (Text(left), Text(right)) => left == right,
            (Int(left, _), Int(right, _)) => left == right,
            (Real(left, _), Real(right, _)) => left == right,
            (Boolean(left, _), Boolean(right, _)) => left == right,
            _ => false,
        }
    }
}

impl hash::Hash for LiteralLexeme {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        use LiteralLexeme::*;
        match self {
            Text(TextLiteralLexeme { text: val, source: _ }) => val.hash(state),
            Int(val, _) => val.hash(state),
            Real(val, _) => val.hash(state),
            Boolean(val, _) => val.hash(state),
        }
    }
}

impl SourceLocation for LiteralLexeme {
    fn source(&self) -> &SourceSlice {
        match self {
            LiteralLexeme::Text(tll) => tll.source(),
            LiteralLexeme::Int(_, source) => &source,
            LiteralLexeme::Real(_, source) => &source,
            LiteralLexeme::Boolean(_, source) => &source,
        }
    }
}

impl ToText for LiteralLexeme {
    fn to_text(&self) -> String {
        match self {
            //TODO @mark: escape
            LiteralLexeme::Text(tll) => format!("\"{}\"", tll.text),
            LiteralLexeme::Int(val, _) => format!("{}", val),
            LiteralLexeme::Real(val, _) => format!("{}", val),
            LiteralLexeme::Boolean(val, _) => format!("{}", val),
        }
    }
}
