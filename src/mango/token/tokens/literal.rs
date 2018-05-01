use mango::token::Token;
use mango::util::encdec::ToText;
use mango::util::strtype::Msg;
use mango::util::strtype::Name;
use mango::util::strtype::strtype::StrType;
use std::fmt::Display;

// LATER: it is likely that this will be refactored when the type system is in place.

/// A literal, like 9 or "hello".
pub trait LiteralToken<T: Display> {
    fn new(val: T) -> Self;
    fn value(&self) -> T;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct StringLiteralToken {
    value: String,
}

/// Implement ToText for all implementations of LiteralToken.
impl<U: LiteralToken<T>> ToText for U {
    fn to_text(&self) -> String {
        format!("{}", self.value())
    }
}

impl<U: LiteralToken<T>> Token for U {}
